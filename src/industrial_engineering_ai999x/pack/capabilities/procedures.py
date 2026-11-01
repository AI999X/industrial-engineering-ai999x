"""Industrial engineering and supply chain procedures: inventory policies, statistical process control, reliability,
queueing, scheduling and sequencing, forecasting accuracy, line balancing, learning curves, facility location,
break-even analysis and work measurement (domain code, loaded by the pack; enabled only once
tools/certify_procedures.py has certified them).

Each procedure states its inputs, outputs, conditions, method and LaTeX, the documented question form it reads,
worked examples (their outputs computed by the INDEPENDENT reference on the build machine, EXAMPLE_OUTPUTS), the
reference itself (used only by certification; SciPy, statsmodels, pandas, NumPy or SymPy imported inside the
reference functions, or an exhaustive search where the procedure uses a rule) and a deliberately wrong variant that the
checks must reject. The shipped code (run, verify, parse) is plain Python (math, re, statistics).

Conventions, stated once and repeated in each procedure where they matter:
  * every input is a plain number in consistent units chosen by the user (demand per year with costs per year,
    demand per day with lead times in days, rates per hour with times in hours); no unit is converted;
  * probabilities, service levels, fill rates, fractions and rates such as a holding-cost rate are decimals (0.95) or
    written with a percent sign (95%);
  * demand during a lead time or review period is normal unless the capability says otherwise (Poisson, uniform,
    discrete); a cycle service level is the probability of no stock-out in a replenishment cycle, a fill rate the
    fraction of demand met from stock;
  * control-chart constants are the standard 3-decimal table values (A2, D3, D4, A3, B3, B4, d2; checked at build time
    against their exact integrals) and limits are 3-sigma;
  * jobs, tasks, items, components and sites are numbered 1, 2, 3, ... in the order they are given; ties in a
    sequencing or assignment rule are broken by the lower number;
  * a count that must be whole (stations, servers, containers, units) is rounded up; a value within 1e-9 (relative) of
    a whole number counts as that whole number;
  * outputs are rounded to 12 significant digits so that every platform prints the same digits.

Questions are read in one documented form: a phrase naming the capability, then `name = value` pairs (lists in
brackets: `processing times = [4, 7, 2]`; matrices as `[[1, 2], [3, 4]]`). A number that is not given as
`name = value`, or a name the capability does not take, makes the question unrecognised (never guessed).
"""
from __future__ import annotations

import math
import re
import statistics
from functools import lru_cache

PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK_REFERENCES: dict = {}
_ALIASES: dict = {}
_TRIGGERS: dict = {}
_SYN: dict = {}
_ND = statistics.NormalDist(0.0, 1.0)


# --- input checks -----------------------------------------------------------------------------------------------
def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(name, v):
    _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), f"{name} must be a finite number")
    return float(v)


def _pos(name, v):
    v = _fin(name, v)
    _need(v > 0, f"{name} must be positive")
    return v


def _nonneg(name, v):
    v = _fin(name, v)
    _need(v >= 0, f"{name} must not be negative")
    return v


def _prob(name, v, lo=0.0, hi=1.0):
    v = _fin(name, v)
    _need(lo < v < hi, f"{name} must lie strictly between {lo:g} and {hi:g} (write 95% or 0.95)")
    return v


def _int(name, v, lo=1, hi=None):
    _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and float(v) == int(v),
          f"{name} must be a whole number")
    v = int(v)
    _need(v >= lo, f"{name} must be at least {lo}")
    _need(hi is None or v <= hi, f"{name} must be at most {hi}")
    return v


def _vec(name, xs, n_min=1, n_max=None, positive=False, nonneg=False):
    _need(isinstance(xs, (list, tuple)) and len(xs) >= n_min, f"{name} needs at least {n_min} values")
    _need(n_max is None or len(xs) <= n_max, f"{name} takes at most {n_max} values")
    out = [_fin(name, x) for x in xs]
    for x in out:
        _need(not positive or x > 0, f"every value of {name} must be positive")
        _need(not nonneg or x >= 0, f"no value of {name} may be negative")
    return out


def _mat(name, m, rows_min=1, cols=None, cols_min=1, cols_max=None):
    _need(isinstance(m, (list, tuple)) and len(m) >= rows_min, f"{name} needs at least {rows_min} rows")
    out = []
    for r in m:
        _need(isinstance(r, (list, tuple)), f"{name} must be a matrix (a list of rows)")
        out.append([_fin(name, x) for x in r])
    w = len(out[0])
    for r in out:
        _need(len(r) == w, f"{name} must be rectangular (rows of equal length)")
    _need(cols is None or w == cols, f"{name} must have {cols} columns")
    _need(w >= cols_min and (cols_max is None or w <= cols_max), f"{name} has the wrong number of columns")
    return out


def _ceil(x):
    """Round up; a value within 1e-9 (relative) of a whole number counts as that whole number."""
    k = round(x)
    if abs(x - k) <= 1e-9 * max(1.0, abs(x)):
        return int(k)
    return int(math.ceil(x))


def _r12(x):
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r12(v) for v in x]
    if not math.isfinite(x) or x == 0.0:
        return 0.0 if x == 0.0 else x
    return float(f"{x:.12g}") + 0.0


def _rd(d):
    out = {}
    for k, v in d.items():
        out[k] = _r12(v)
    return out


# --- distributions (plain Python) ----------------------------------------------------------------------------------
def _cdf(z):
    return 0.5 * math.erfc(-z / math.sqrt(2.0))       # accurate in both tails


def _sf(z):
    return 0.5 * math.erfc(z / math.sqrt(2.0))


def _pdf(z):
    return _ND.pdf(z)


def _zinv(p):
    return _ND.inv_cdf(p)


def _loss(z):
    """Standard normal loss function L(z) = E[(Z - z)+] = phi(z) - z (1 - Phi(z))."""
    return _pdf(z) - z * _sf(z)


def _pois_pmfs(lam, kmax):
    p = math.exp(-lam)
    out = [p]
    for k in range(1, kmax + 1):
        p = p * lam / k
        out.append(p)
    return out


def _binom_cdf(c, n, p):
    s = 0.0
    for k in range(0, c + 1):
        s += math.comb(n, k) * p ** k * (1.0 - p) ** (n - k)
    return min(1.0, s)


def _bisect_root(f, a, b, iters=300):
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa < 0) != (fb < 0), "no solution in the admissible range")
    for _ in range(iters):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm < 0) == (fa < 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


# --- verification: outputs compared with a recomputation (relative, tiny absolute floor) -------------------------
def _close(a, b):
    if isinstance(b, str) or isinstance(a, str):
        return 0.0 if a == b else math.inf
    if isinstance(a, bool) or not isinstance(a, (int, float)) or not math.isfinite(a):
        return math.inf
    d = abs(a - b)
    return 0.0 if d <= 1e-15 else d / max(abs(a), abs(b))


def _residual(got, want) -> float:
    worst = 0.0
    for k, w in want.items():
        if k not in got:
            return math.inf
        o = got[k]
        if isinstance(w, (list, tuple)):
            if not isinstance(o, (list, tuple)) or len(o) != len(w):
                return math.inf
            for a, b in zip(o, w):
                if isinstance(b, (list, tuple)):
                    if not isinstance(a, (list, tuple)) or len(a) != len(b):
                        return math.inf
                    for a2, b2 in zip(a, b):
                        worst = max(worst, _close(a2, b2))
                else:
                    worst = max(worst, _close(a, b))
        else:
            worst = max(worst, _close(o, w))
    return worst


def _verifier(fn):
    def verify(inputs, outputs):
        return _residual(outputs, fn(**inputs))
    return verify


# --- reading questions ----------------------------------------------------------------------------------------
_CONNECT = {"with", "and", "where", "given", "at", "for", "if", "when", "using", "let", "is", "are", "has", "having",
            "assuming", "then", "also", "plus", "a", "an", "the", "of", "its", "to", "from", "by", "in", "on", "as"}
_NUMBER = re.compile(r"[$€£]?\s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?)(\s*%)?")
_WORD = re.compile(r"[a-z][a-z-]*")


def _norm(question: str) -> str:
    q = question.lower().replace("−", "-").replace("–", "-").replace("’", "'").replace("×", "x")
    return re.sub(r"\s+", " ", q).strip()


def _value_at(text, j):
    """(value text, kind, end) of the value written at text[j:], or None."""
    while j < len(text) and text[j] == " ":
        j += 1
    if j >= len(text):
        return None
    if text[j] == "[":
        depth = 0
        for k in range(j, len(text)):
            depth += {"[": 1, "]": -1}.get(text[k], 0)
            if depth == 0:
                return text[j:k + 1], "list", k + 1
        return None
    m = _NUMBER.match(text, j)
    if m:
        return m.group(0), "number", m.end()
    m = _WORD.match(text, j)
    if m:
        return m.group(0), "word", m.end()
    return None


def _pairs(text):
    """Every `name = value` of a question: (segment before '=', value, kind) and the text that is not a pair."""
    pairs, rest, pos = [], [], 0
    for m in re.finditer("=", text):
        p = m.start()
        if p < pos:
            return None
        seg = text[pos:p]
        cut = max(seg.rfind(c) for c in ",;:(")
        rest.append(seg[:cut + 1])
        got = _value_at(text, p + 1)
        if got is None:
            return None
        value, kind, end = got
        if end < len(text) and (text[end].isalnum() or text[end] in "_[="):
            return None
        pairs.append((seg[cut + 1:].strip(), value, kind))
        pos = end
    rest.append(text[pos:])
    return pairs, rest


def _trigger_text(text):
    """The question without its assignments' names and values (an input name must not decide the capability)."""
    got = _pairs(text)
    if got is None:
        return text
    pairs, rest = got
    pieces = list(rest)
    for i, (seg, _v, _k) in enumerate(pairs):
        words = list(re.finditer(r"[a-z0-9_'%$.-]+", seg))
        keep = 0
        for w in words:
            if w.group(0) in _CONNECT:
                keep = w.end()
        pieces[i] = pieces[i] + " " + seg[:keep]
    return " ".join(pieces)


@lru_cache(maxsize=256)
def _winners(text):
    t = _trigger_text(text)
    best, keys = 0, []
    for key, pats in _TRIGGERS.items():
        n = max((len(m.group(0)) for p in pats for m in [re.search(p, t)] if m), default=0)
        if n > best:
            best, keys = n, [key]
        elif n == best and n > 0:
            keys.append(key)
    return tuple(keys)


def _list_value(v):
    inner = v.strip()[1:-1]
    if "%" not in inner or "[" in inner:
        return v
    try:
        return [float(t[:-1]) / 100 if t.endswith("%") else float(t) for t in re.split(r"[,\s;]+", inner.strip()) if t]
    except ValueError:
        return v


def _read(key, text):
    got = _pairs(text)
    if got is None:
        return None
    pairs, rest = got
    aliases, by = _ALIASES[key], {i["symbol"]: i for i in PROCEDURES[key]["inputs"]}
    out, left = {}, list(rest)
    for seg, value, kind in pairs:
        sym = None
        for alias, s in aliases:
            if seg.endswith(alias) and (len(seg) == len(alias) or seg[-len(alias) - 1] == " "):
                before = seg[:-len(alias)].strip()
                words = re.findall(r"[a-z0-9_'%$.-]+", before)
                if not words or words[-1] in _CONNECT:
                    sym = s
                    left.append(before)
                    break
        if sym is None or sym in out:
            return None                                   # a name the capability does not take (or given twice)
        spec = by[sym]
        t = spec.get("type", "number")
        if t == "choice":
            if kind != "word":
                return None
            out[sym] = _SYN.get(key, {}).get(sym, {}).get(value, value)
        elif t in ("vector", "matrix"):
            if kind != "list":
                return None
            out[sym] = _list_value(value) if t == "vector" else value
        else:
            if kind != "number":
                return None
            m = _NUMBER.match(value)
            x = float(m.group(1))
            out[sym] = x / 100 if m.group(2) else x
    words = " ".join(left)
    for pat in _TRIGGERS[key]:                            # the capability's own phrase may hold digits (M/M/1)
        words = pat.sub(" ", words)
    if re.search(r"[\d=\[\]]", words):
        return None                                       # a number that is not given as name = value: never guessed
    return out


def _parser(key):
    def parse(question):
        if not isinstance(question, str):
            return None
        text = _norm(question)
        if key not in _winners(text):
            return None
        return _read(key, text)
    return parse


def _phrase(p):
    """'economic order quantity' -> a regex with word boundaries and flexible spaces or hyphens."""
    return r"\b" + re.sub(r"[ -]+", r"[\\s-]+", p) + r"\b"


# --- declaring a procedure ------------------------------------------------------------------------------------------
def _i(sym, meaning, *aliases, type=None, required=True, choices=None, syn=None, **limits):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if type:
        d["type"] = type
    if choices:
        d["choices"] = list(choices)
    d.update(limits)
    d["_aliases"] = list(aliases) + [sym.replace("_", " ")]
    if syn:
        d["_syn"] = syn
    return d


def _o(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _fmt(v):
    if isinstance(v, str):
        return v
    if isinstance(v, (list, tuple)):
        return "[" + ", ".join(_fmt(x) for x in v) + "]"
    if isinstance(v, int) or float(v) == int(v) and abs(v) < 1e15:
        return str(int(v))
    return repr(float(v))


def _ask_text(phrase, inputs, ex):
    names = {i["symbol"]: next((a for a in i["_aliases"] if len(a) >= 4), i["symbol"].replace("_", " ")) for i in inputs}
    parts = [f"{names[k]} = {_fmt(v)}" for k, v in ex.items()]
    return f"{phrase[0].upper()}{phrase[1:]} with " + ", ".join(parts) + "."


def _add(key, *, name, topic, purpose, inputs, outputs, primary, conditions, method, latex, run, reference, wrong,
         rand, triggers, examples, level, invalid=(), check=None, weak=None, ask=None):
    """Register one procedure. `examples` are input dicts written as a user would give them; their outputs come from
    EXAMPLE_OUTPUTS (computed by the reference) or, until then, from `run`. The documented questions are built from
    them: the first two examples (solved), a missing input (NEEDS_INPUT), an unknown name (UNRECOGNIZED_FORMAT) and
    each `invalid` input set (INVALID_INPUT)."""
    phrase = ask or triggers[0]
    exs = []
    stored = EXAMPLE_OUTPUTS.get(key)
    for n, ex in enumerate(examples):
        outs = stored[n] if stored and n < len(stored) else run(**_coerce_example(inputs, ex))
        exs.append({"inputs": dict(ex), "outputs": outs})
    qs = [{"q": _ask_text(phrase, inputs, e["inputs"]), "outputs": e["outputs"]} for e in exs[:2]]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if len(req) >= 2:
        ex0 = {k: v for k, v in examples[0].items() if k != req[-1]}
        qs.append({"q": _ask_text(phrase, inputs, ex0), "outcome": "NEEDS_INPUT"})
    qs.append({"q": _ask_text(phrase, inputs, examples[0])[:-1] + ", colour = 3.", "outcome": "UNRECOGNIZED_FORMAT"})
    for bad in invalid:
        qs.append({"q": _ask_text(phrase, inputs, bad), "outcome": "INVALID_INPUT"})
    al, syn = [], {}
    for i in inputs:
        for a in i.pop("_aliases"):
            al.append((a.lower(), i["symbol"]))
        if "_syn" in i:
            syn[i["symbol"]] = i.pop("_syn")
    _ALIASES[key] = sorted(set(al), key=lambda t: -len(t[0]))
    _TRIGGERS[key] = [re.compile(t[3:] if t.startswith("re:") else _phrase(t)) for t in triggers]
    if syn:
        _SYN[key] = syn
    LEVELS[key] = level
    if weak:
        WEAK_REFERENCES[key] = weak
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": conditions, "method": method, "latex": latex, "run": run,
        "verify": _verifier(check or run), "parse": _parser(key), "reference": reference, "wrong": wrong,
        "random_inputs": rand, "templates": [qs[0]["q"]], "examples": exs, "questions": qs,
        "keywords": [t for t in triggers if not t.startswith("re:")]}


def _coerce_example(inputs, ex):
    by = {i["symbol"]: i for i in inputs}
    out = {}
    for k, v in ex.items():
        t = by[k].get("type", "number")
        if t == "integer":
            out[k] = int(v)
        elif t == "vector":
            out[k] = [float(x) for x in v]
        elif t == "matrix":
            out[k] = [[float(x) for x in r] for r in v]
        else:
            out[k] = v
    return out


def _u(rng, a, b, nd=4):
    return round(rng.uniform(a, b), nd)


def _drop_none(d):
    return {k: v for k, v in d.items() if v is not None}


# --- reference helpers (build machine only) ---------------------------------------------------------------------
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=1000)


def _foc_min(expr, var, lo=1e-9, hi=1e12):
    """Minimiser of a convex cost expression (SymPy) by a root of its derivative (SymPy differentiation, brentq)."""
    import sympy as sp
    f = sp.lambdify(var, sp.diff(expr, var), "math")
    return _brentq(f, lo, hi)


def _quad(f, a, b):
    from scipy.integrate import quad
    v, _e = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500)
    return v


# Outputs of the documented examples, computed by each capability's INDEPENDENT reference on the build machine and
# rounded to 12 significant digits; certification checks the procedure against them.
EXAMPLE_OUTPUTS: dict = {
    'ie_eoq': [{'Q': 200.0, 'orders_per_year': 6.0, 'cycle_time': 0.166666666667, 'total_cost': 1200.0}, {'Q': 536.6563146, 'orders_per_year': 14.90711985, 'cycle_time': 0.067082039325, 'total_cost': 1341.6407865}],
    'ie_inventory_total_cost': [{'total_cost': 1300.0, 'ordering_cost': 400.0, 'holding_cost_per_year': 900.0, 'average_inventory': 150.0}, {'total_cost': 61550.0, 'ordering_cost': 750.0, 'holding_cost_per_year': 800.0, 'average_inventory': 200.0}],
    'ie_eoq_backorders': [{'Q': 230.940107676, 'max_backorder': 57.735026919, 'max_inventory': 173.205080757, 'total_cost': 1039.23048454}, {'Q': 1118.03398875, 'max_backorder': 223.60679775, 'max_inventory': 894.427191, 'total_cost': 2683.281573}],
    'ie_eoq_quantity_discount': [{'Q': 1000.0, 'unit_price': 4.8, 'total_cost': 24725.0}, {'Q': 600.0, 'unit_price': 18.5, 'total_cost': 23787.5}],
    'ie_epq': [{'Q': 2065.59111798, 'max_inventory': 1549.19333848, 'production_run_time': 0.0430331482912, 'total_cost': 4647.58001545}, {'Q': 230.940107676, 'max_inventory': 173.205080757, 'production_run_time': 0.057735026919, 'total_cost': 86.6025403784}],
    'ie_reorder_point': [{'reorder_point': 200.0, 'lead_time_demand': 200.0}, {'reorder_point': 480.0, 'lead_time_demand': 420.0}],
    'ie_safety_stock': [{'safety_stock': 59.2147305703, 'reorder_point': 509.21473057, 'z': 1.64485362695}, {'safety_stock': 186.107829923, 'reorder_point': 986.107829923, 'z': 2.32634787404}],
    'ie_safety_stock_variable_lead_time': [{'safety_stock': 338.696091583, 'reorder_point': 938.696091583, 'sd_lead_time_demand': 205.91260282}, {'safety_stock': 142.287898368, 'reorder_point': 622.287898368, 'sd_lead_time_demand': 69.2820323028}],
    'ie_newsvendor_normal': [{'Q': 500.0, 'critical_ratio': 0.5, 'expected_sales': 460.10577196, 'expected_profit': 1680.84617568}, {'Q': 1131.10012818, 'critical_ratio': 0.7, 'expected_sales': 952.406884903, 'expected_profit': 3065.38423225}],
    'ie_newsvendor_uniform': [{'Q': 211.111111111, 'critical_ratio': 0.555555555556, 'expected_sales': 180.24691358, 'expected_profit': 777.777777778}, {'Q': 375.0, 'critical_ratio': 0.375, 'expected_sales': 304.6875, 'expected_profit': 562.5}],
    'ie_newsvendor_poisson': [{'Q': 13, 'critical_ratio': 0.6, 'expected_sales': 11.0516197653, 'expected_profit': 117.032395306, 'stockout_probability': 0.31846436788}, {'Q': 40, 'critical_ratio': 0.5, 'expected_sales': 37.4821184231, 'expected_profit': 699.284736922, 'stockout_probability': 0.458081821637}],
    'ie_newsvendor_discrete': [{'Q': 40.0, 'expected_profit': 170.0}, {'Q': 200.0, 'expected_profit': 375.0}],
    'ie_rq_fill_rate': [{'reorder_point': 590.234634751, 'safety_stock': 90.234634751, 'cycle_service_level': 0.816563543807}, {'reorder_point': 221.361160524, 'safety_stock': 21.3611605242, 'cycle_service_level': 0.703339269414}],
    'ie_service_level_from_reorder_point': [{'cycle_service_level': 0.933192798731, 'expected_shortage_per_cycle': 2.93067937626, 'fill_rate': 0.997069320624}, {'cycle_service_level': 0.747507462453, 'expected_shortage_per_cycle': 4.53358941473}],
    'ie_order_up_to_level': [{'order_up_to_level': 450.150665011, 'safety_stock': 30.1506650105, 'order_quantity': 300.150665011}, {'order_up_to_level': 1162.36310749, 'safety_stock': 162.363107492}],
    'ie_abc_classification': [{'classes': ['A', 'B', 'C', 'A', 'C', 'B', 'C', 'C', 'A', 'B'], 'count_a': 3, 'share_a': 0.811541929666}, {'classes': ['C', 'A', 'C', 'A', 'C'], 'count_a': 2, 'share_a': 0.952380952381}],
    'ie_inventory_turnover': [{'turnover': 8.0, 'days_of_supply': 45.625, 'weeks_of_supply': 6.5}, {'turnover': 4.0, 'days_of_supply': 62.5, 'weeks_of_supply': 13.0}],
    'ie_kanban_count': [{'kanbans': 11, 'exact': 11.0, 'max_inventory': 220.0}, {'kanbans': 7, 'exact': 7.0, 'max_inventory': 175.0}],
    'ie_xbar_r_limits': [{'xbar_ucl': 51.154, 'xbar_lcl': 48.846, 'r_ucl': 4.228, 'r_lcl': 0.0}, {'xbar_ucl': 10.45515, 'xbar_lcl': 9.94485, 'r_ucl': 0.7987, 'r_lcl': 0.0}],
    'ie_xbar_s_limits': [{'xbar_ucl': 51.2843, 'xbar_lcl': 48.7157, 's_ucl': 1.8801, 's_lcl': 0.0}, {'xbar_ucl': 74.0134138, 'xbar_lcl': 73.9865862, 's_ucl': 0.0196366, 's_lcl': 0.0}],
    'ie_sigma_from_range': [{'sigma': 0.859845227859, 'natural_tolerance': 5.15907136715}, {'sigma': 0.16998542982, 'natural_tolerance': 1.01991257892}],
    'ie_p_chart_limits': [{'ucl': 0.0987877538268, 'lcl': 0.0, 'sigma_p': 0.0195959179423}, {'ucl': 0.301492574075, 'lcl': 0.0, 'sigma_p': 0.0504975246918}],
    'ie_np_chart_limits': [{'ucl': 9.87877538268, 'lcl': 0.0, 'center': 4.0, 'sigma_np': 1.95959179423}, {'ucl': 32.7279220614, 'lcl': 7.27207793864, 'center': 20.0, 'sigma_np': 4.24264068712}],
    'ie_c_chart_limits': [{'ucl': 18.0, 'lcl': 0.0, 'sigma_c': 3.0}, {'ucl': 7.24341649025, 'lcl': 0.0, 'sigma_c': 1.58113883008}],
    'ie_u_chart_limits': [{'ucl': 2.66189500386, 'lcl': 0.338104996138, 'sigma_u': 0.387298334621}, {'ucl': 0.65941125497, 'lcl': 0.0, 'sigma_u': 0.11313708499}],
    'ie_p_chart_from_samples': [{'p_bar': 0.043, 'ucl': 0.103857127767, 'lcl': 0.0, 'out_of_control': [5], 'count_out': 1}, {'p_bar': 0.0491666666667, 'ucl': 0.102128524011, 'lcl': 0.0, 'out_of_control': [], 'count_out': 0}],
    'ie_process_capability': [{'cpk': 1.11111111111, 'cp': 1.38888888889, 'cpu': 1.11111111111, 'cpl': 1.66666666667}, {'cpk': 1.04166666667, 'cp': 1.25, 'cpu': 1.04166666667, 'cpl': 1.45833333333}],
    'ie_process_performance': [{'ppk': 1.14328420151, 'pp': 1.24721912892, 'mean': 10.05, 'sd': 0.160356745147}, {'ppk': 1.1029305038, 'pp': 1.24524411719, 'mean': 50.2, 'sd': 1.87379590967}],
    'ie_cpm_taguchi': [{'cpm': 1.41421356237, 'tau': 0.141421356237}, {'cpm': 1.04115841259, 'tau': 3.20156211872}],
    'ie_fraction_nonconforming': [{'fraction_out': 0.000858120666394, 'below_lsl': 0.000429060333197, 'above_usl': 0.000429060333197, 'ppm': 858.120666394}, {'fraction_out': 0.000889025299108, 'below_lsl': 0.0, 'above_usl': 0.000889025299108, 'ppm': 889.025299108}],
    'ie_arl_shewhart': [{'arl': 4.49531222661, 'detection_probability': 0.22245395861, 'arl_in_control': 370.398347345}, {'arl': 1.99999999605, 'detection_probability': 0.500000000987, 'arl_in_control': 370.398347345}],
    'ie_ewma_limits': [{'ucl': 10.27, 'lcl': 9.73, 'ucl_steady': 10.6194224815}, {'ucl': 51.8895775375, 'lcl': 48.1104224625, 'ucl_steady': 52.0}],
    'ie_tabular_cusum': [{'first_signal': 14, 'c_plus': [0.0, 0.0, 0.0, 1.16, 2.82, 2.5, 0.04, 1.0, 0.0, 0.0, 1.3, 3.0, 4.4, 6.4], 'c_minus': [0.05, 1.56, 1.77, 0.0, 0.0, 0.0, 1.46, 0.0, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0]}, {'first_signal': 6, 'c_plus': [0.0, 0.0, 0.0, 1.3, 3.4, 5.2, 8.2], 'c_minus': [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]}],
    'ie_dpmo_sigma_level': [{'dpmo': 5250.0, 'dpu': 0.042, 'sigma_level': 4.05891305037}, {'dpmo': 300.0, 'dpu': 0.003, 'sigma_level': 4.93161440362}],
    'ie_rolled_throughput_yield': [{'rty': 0.829521, 'normalized_yield': 0.954348172614, 'total_dpu': 0.186906853216}, {'rty': 0.8924, 'normalized_yield': 0.94466925429, 'total_dpu': 0.113840816424}],
    'ie_acceptance_sampling': [{'probability_accept': 0.735771394462, 'aoq': 0.014347542192, 'ati': 565.2457808}, {'probability_accept': 0.939689918314, 'aoq': 0.00931326678041, 'ati': 686.733219594}],
    'ie_series_reliability': [{'reliability': 0.92169, 'unreliability': 0.07831}, {'reliability': 0.667845, 'unreliability': 0.332155}],
    'ie_parallel_reliability': [{'reliability': 0.98, 'unreliability': 0.02}, {'reliability': 0.99925, 'unreliability': 0.00075}],
    'ie_k_out_of_n_reliability': [{'reliability': 0.972}, {'reliability': 0.998841875}],
    'ie_series_parallel_reliability': [{'reliability': 0.948024, 'unreliability': 0.051976}, {'reliability': 0.97909, 'unreliability': 0.02091}],
    'ie_exponential_reliability': [{'reliability': 0.818730753078, 'unreliability': 0.181269246922, 'mtbf': 5000.0}, {'reliability': 0.606530659713, 'unreliability': 0.393469340287, 'mtbf': 1000.0}],
    'ie_availability': [{'availability': 0.961538461538, 'unavailability': 0.0384615384615, 'downtime_per_year': 336.923076923}, {'availability': 0.993377483444, 'unavailability': 0.00662251655629, 'downtime_per_year': 39.7350993377}],
    'ie_weibull_reliability': [{'reliability': 0.778800783071, 'hazard_rate': 0.001, 'mttf': 886.226925453, 'b10_life': 324.592845975}, {'reliability': 0.805735301873, 'hazard_rate': 0.00018, 'mttf': 4713.72646475, 'b10_life': 1315.37762818}],
    'ie_weibull_median_rank_regression': [{'shape': 1.4269671106, 'scale': 76.3454154554, 'r_squared': 0.991181155764}, {'shape': 1.90306129812, 'scale': 1930.68248465, 'r_squared': 0.982191030355}],
    'ie_redundancy_needed': [{'components': 3, 'achieved_reliability': 0.999}, {'components': 7, 'achieved_reliability': 0.999938964844}],
    'ie_system_failure_rate': [{'system_failure_rate': 0.00035, 'mtbf': 2857.14285714, 'reliability': 0.704688089719}, {'system_failure_rate': 0.003, 'mtbf': 333.333333333}],
    'ie_parallel_mttf': [{'mttf': 1500.0, 'improvement_factor': 1.5}, {'mttf': 3666.66666667, 'improvement_factor': 1.83333333333}],
    'ie_standby_redundancy': [{'reliability': 0.735758882343, 'mttf': 2000.0}, {'reliability': 0.919698602929, 'mttf': 1500.0}],
    'ie_mtbf_confidence': [{'mtbf': 2500.0, 'lower_one_sided': 1251.00243043, 'lower_two_sided': 1092.47601615, 'upper_two_sided': 7318.93826782}, {'mtbf': 2500.0, 'lower_one_sided': 1053.99300098, 'lower_two_sided': 897.401296022, 'upper_two_sided': 20643.3049554}],
    'ie_mm1_queue': [{'utilization': 0.666666666667, 'L': 2.0, 'Lq': 1.33333333333, 'W': 0.999999999999, 'Wq': 0.666666666666, 'p0': 0.333333333333}, {'utilization': 0.9, 'L': 9.00000000007, 'Lq': 8.10000000007, 'W': 0.500000000004, 'Wq': 0.450000000004, 'p0': 0.1}],
    'ie_mmc_queue': [{'p_wait': 0.702247191011, 'Lq': 3.51123595506, 'Wq': 0.351123595506, 'W': 0.601123595506, 'L': 6.01123595506}, {'p_wait': 0.509148620661, 'Lq': 2.54574310331, 'Wq': 0.0565720689624, 'W': 0.223238735629, 'L': 10.0457431033}],
    'ie_md1_queue': [{'Lq': 0.666666666667, 'Wq': 0.333333333333, 'W': 0.666666666667, 'L': 1.33333333333}, {'Lq': 1.6, 'Wq': 0.2, 'W': 0.3, 'L': 2.4}],
    'ie_mg1_queue': [{'Lq': 1.625, 'Wq': 3.25, 'W': 4.75, 'L': 2.375}, {'Lq': 2.225, 'Wq': 0.2225, 'W': 0.3025, 'L': 3.025}],
    'ie_mm1k_queue': [{'p0': 0.338753387534, 'p_block': 0.173441734417, 'L': 1.22493224932, 'Lq': 0.563685636856, 'effective_arrival_rate': 3.30623306233, 'W': 0.370491803279}, {'p0': 0.0774239263469, 'p_block': 0.231186605289, 'L': 3.70983742214, 'Lq': 2.78726134849, 'effective_arrival_rate': 9.22576073653, 'W': 0.402117237601}],
    'ie_erlang_c_staffing': [{'servers': 11, 'service_level': 0.896774817908, 'p_wait': 0.299619350037, 'mean_wait': 0.00936310468866}, {'servers': 5, 'service_level': 0.913124680073, 'p_wait': 0.236151603499, 'mean_wait': 0.0118075801749}],
    'ie_queue_server_cost': [{'servers': 5, 'total_cost': 284.466875735, 'L': 3.98667189338}, {'servers': 4, 'total_cost': 128.470066519, 'L': 3.42350332594}],
    'ie_littles_law': [{'arrival_rate': 12.0, 'number_in_system': 6.0, 'time_in_system': 0.5}, {'arrival_rate': 120.0, 'number_in_system': 300.0, 'time_in_system': 2.5}],
    'ie_spt_sequence': [{'sequence': [2, 4, 1, 3, 5], 'mean_flow_time': 13.0, 'makespan': 28.0, 'average_jobs_in_system': 2.32142857143, 'utilization': 0.430769230769}, {'sequence': [3, 1, 4, 2], 'mean_flow_time': 9.25, 'makespan': 18.0, 'average_jobs_in_system': 2.05555555556, 'utilization': 0.486486486486, 'total_tardiness': 3.0, 'mean_tardiness': 0.75, 'max_tardiness': 3.0, 'max_lateness': 3.0, 'tardy_jobs': 1}],
    'ie_edd_sequence': [{'sequence': [3, 1, 4, 2], 'mean_flow_time': 9.25, 'makespan': 18.0, 'average_jobs_in_system': 2.05555555556, 'utilization': 0.486486486486, 'total_tardiness': 3.0, 'mean_tardiness': 0.75, 'max_tardiness': 3.0, 'max_lateness': 3.0, 'tardy_jobs': 1}, {'sequence': [3, 1, 5, 4, 2], 'mean_flow_time': 11.2, 'makespan': 23.0, 'average_jobs_in_system': 2.4347826087, 'utilization': 0.410714285714, 'total_tardiness': 3.0, 'mean_tardiness': 0.6, 'max_tardiness': 3.0, 'max_lateness': 3.0, 'tardy_jobs': 1}],
    'ie_dispatching_rule': [{'sequence': [1, 2, 3, 4, 5], 'mean_flow_time': 15.4, 'makespan': 28.0, 'average_jobs_in_system': 2.75, 'utilization': 0.363636363636, 'total_tardiness': 11.0, 'mean_tardiness': 2.2, 'max_tardiness': 5.0, 'max_lateness': 5.0, 'tardy_jobs': 3}, {'sequence': [1, 3, 5, 2, 4], 'mean_flow_time': 19.2, 'makespan': 28.0, 'average_jobs_in_system': 3.42857142857, 'utilization': 0.291666666667, 'total_tardiness': 32.0, 'mean_tardiness': 6.4, 'max_tardiness': 19.0, 'max_lateness': 19.0, 'tardy_jobs': 2}],
    'ie_johnson_two_machine': [{'sequence': [3, 2, 4, 5, 1], 'makespan': 23.0, 'idle_time_machine2': 5.0}, {'sequence': [1, 3, 4, 2], 'makespan': 23.0, 'idle_time_machine2': 5.0}],
    'ie_johnson_three_machine': [{'makespan': 51.0}, {'makespan': 36.0}],
    'ie_moore_hodgson': [{'number_late': 2}, {'number_late': 2}],
    'ie_flow_shop_makespan': [{'makespan': 27.0, 'completion_times': [14.0, 19.0, 24.0, 27.0]}, {'makespan': 13.0, 'completion_times': [9.0, 11.0, 13.0]}],
    'ie_moving_average_forecast': [{'forecast': 146.333333333, 'forecasts': [127.666666667, 135.0, 140.0, 146.333333333]}, {'forecast': 43.0, 'forecasts': [42.0, 41.5, 43.0]}],
    'ie_weighted_moving_average': [{'forecast': 113.333333333}, {'forecast': 58.0}],
    'ie_exponential_smoothing': [{'forecast': 143.2288, 'forecasts': [140.0, 140.4, 142.92, 141.536, 143.2288]}, {'forecast': 24.3136, 'forecasts': [20.0, 20.0, 21.6, 21.76, 23.856, 24.3136]}],
    'ie_holt_trend': [{'forecast': 27.8592460759, 'level': 25.3133157786, 'trend': 2.54593029734}, {'forecast': 143.01994375, 'level': 125.1956875, 'trend': 5.94141875}],
    'ie_forecast_errors': [{'mad': 4.0, 'mse': 17.5, 'rmse': 4.18330013267, 'bias': -1.0, 'mape': 3.51544289044, 'tracking_signal': -1.0}, {'mad': 3.4, 'mse': 13.8, 'rmse': 3.7148351242, 'bias': 1.4, 'mape': 5.98379577412, 'tracking_signal': 2.05882352941}],
    'ie_tracking_signal': [{'final_signal': 2.47058823529, 'signals': [-1.0, -2.0, 0.0, -1.0, 0.454545454545, 2.47058823529], 'first_out_of_control': 0}, {'final_signal': 2.05882352941, 'signals': [1.0, 0.0, 1.8, 0.923076923077, 2.05882352941], 'first_out_of_control': 0}],
    'ie_linear_trend_forecast': [{'forecast': 141.0, 'intercept': 56.7142857143, 'slope': 10.5357142857}, {'forecast': 41.7, 'intercept': 16.5, 'slope': 3.6}],
    'ie_seasonal_index': [{'indices': [0.841720036597, 0.962488563586, 0.995425434584, 1.20036596523], 'overall_average': 91.0833333333, 'forecasts': [252.516010979, 288.746569076, 298.627630375, 360.10978957]}, {'indices': [0.887323943662, 1.43661971831, 0.676056338028], 'overall_average': 236.666666667}],
    'ie_regression_forecast': [{'intercept': 1.75, 'slope': 0.25, 'r_squared': 0.8125, 'standard_error': 0.306186217848, 'forecast': 3.25}, {'intercept': 36.2088235294, 'slope': 7.27941176471, 'r_squared': 0.989050511509, 'standard_error': 3.64651324192}],
    'ie_takt_time': [{'cycle_time': 8.0, 'output_rate': 0.125, 'minimum_stations': 4, 'exact_stations': 3.75}, {'cycle_time': 0.5, 'output_rate': 2.0}],
    'ie_line_efficiency': [{'efficiency': 0.75, 'balance_delay': 0.25, 'idle_time': 1.5, 'total_task_time': 4.5, 'minimum_stations': 3}, {'efficiency': 0.733333333333, 'balance_delay': 0.266666666667, 'idle_time': 80.0, 'total_task_time': 220.0, 'minimum_stations': 4}],
    'ie_largest_candidate_rule': [{'stations': 3, 'assignment': [1, 1, 1, 2, 2, 3, 3, 3], 'station_times': [9.0, 9.0, 9.0], 'efficiency': 0.9}, {'stations': 3, 'assignment': [1, 1, 2, 2, 3, 3], 'station_times': [8.0, 7.0, 7.0], 'efficiency': 0.916666666667}],
    'ie_ranked_positional_weight': [{'stations': 3, 'assignment': [1, 1, 1, 2, 2, 3, 3, 3], 'station_times': [9.0, 9.0, 9.0], 'efficiency': 0.9, 'positional_weights': [27.0, 15.0, 19.0, 12.0, 15.0, 9.0, 5.0, 3.0]}, {'stations': 3, 'assignment': [1, 1, 2, 2, 3, 3], 'station_times': [8.0, 7.0, 7.0], 'efficiency': 0.916666666667, 'positional_weights': [19.0, 17.0, 14.0, 4.0, 7.0, 1.0]}],
    'ie_learning_curve_unit': [{'unit_time': 51.2, 'exponent': -0.321928094887}, {'unit_time': 29.1410201163, 'exponent': -0.234465253637}],
    'ie_learning_curve_total': [{'total_time': 534.591353327, 'average_time': 66.8239191659, 'last_unit_time': 51.2}, {'total_time': 1062.79380442, 'average_time': 42.5117521767, 'last_unit_time': 36.7840923797}],
    'ie_learning_rate_from_two_units': [{'learning_rate': 0.8, 'exponent': -0.321928094887, 'first_unit_time': 100.0}, {'learning_rate': 0.821583836258, 'exponent': -0.283520296362, 'first_unit_time': 76.838749376}],
    'ie_learning_curve_fit': [{'learning_rate': 0.795483081455, 'exponent': -0.330096847625, 'first_unit_time': 101.153671947, 'r_squared': 0.997447162976}, {'learning_rate': 0.859167724413, 'exponent': -0.218988296983, 'first_unit_time': 84.954961014, 'r_squared': 0.999367454237}],
    'ie_learning_curve_cumulative_average': [{'cumulative_average_time': 64.0, 'total_time': 256.0, 'unit_time': 45.3688891664}, {'cumulative_average_time': 799.246167926, 'total_time': 39962.3083963, 'unit_time': 613.297229643}],
    'ie_center_of_gravity': [{'x': 66.6666666667, 'y': 93.3333333333, 'total_weight': 6000.0}, {'x': 4.66666666667, 'y': 5.22222222222, 'total_weight': 900.0}],
    'ie_weber_location': [{'x': 3.1914893617, 'y': 5.10638297872, 'total_weighted_distance': 19.4339811321}, {'x': 63.8644917786, 'y': 97.2781568331, 'total_weighted_distance': 299233.688043}],
    'ie_rectilinear_location': [{'x': 3.0, 'y': 4.0, 'total_weighted_distance': 47.0}, {'x': 60.0, 'y': 120.0, 'total_weighted_distance': 380000.0}],
    'ie_factor_rating': [{'weighted_scores': [71.5, 75.25], 'best_site': 2, 'best_score': 75.25}, {'weighted_scores': [60.0, 80.0, 75.0], 'best_site': 2, 'best_score': 80.0}],
    'ie_location_cost_volume': [{'total_costs': [180000.0, 150000.0, 160000.0], 'best_site': 2, 'best_cost': 150000.0}, {'total_costs': [367000.0, 383000.0], 'best_site': 1, 'best_cost': 367000.0}],
    'ie_break_even': [{'break_even_units': 1000.0, 'break_even_revenue': 25000.0}, {'break_even_units': 10000.0, 'break_even_revenue': 80000.0, 'units_for_target_profit': 14000.0}],
    'ie_multiproduct_break_even': [{'break_even_revenue': 4503.58760368, 'revenue_by_product': [2251.79380184, 1351.07628111, 900.717520737], 'units_by_product': [763.319932828, 1080.86102488, 1125.89690092]}, {'break_even_revenue': 272727.272727, 'revenue_by_product': [163636.363636, 109090.909091], 'units_by_product': [3272.72727273, 1363.63636364]}],
    'ie_standard_time': [{'standard_time': 5.4395, 'normal_time': 4.73, 'mean_observed_time': 4.3}, {'standard_time': 12.9545454545, 'normal_time': 11.4, 'mean_observed_time': 12.0}],
    'ie_time_study_sample_size': [{'sample_size': 43, 'exact': 42.6828757855, 'z': 1.95996398454}, {'sample_size': 23, 'exact': 22.3006246868, 'z': 2.57582930355}],
    'ie_work_sampling_sample_size': [{'sample_size': 246, 'exact': 245.853364524, 'z': 1.95996398454}, {'sample_size': 609, 'exact': 608.747277171, 'z': 1.64485362695}],
    'ie_oee': [{'oee': 0.678571428571, 'availability': 0.888095238095, 'quality': 0.95}, {'oee': 0.708333333333, 'availability': 0.875, 'quality': 0.971428571429}],
    'ie_capacity_utilization': [{'utilization': 0.666666666667, 'efficiency': 0.8, 'effective_share': 0.833333333333}, {'utilization': 0.72, 'efficiency': 0.9, 'effective_share': 0.8}],
}


# =====================================================================================================================
# INVENTORY POLICIES
# =====================================================================================================================
_INV = "inventory"
_D_IN = lambda: _i("demand", "demand per year (units)", "annual demand", "d", "yearly demand", "demand per year")
_S_IN = lambda: _i("order_cost", "fixed cost per order or setup", "ordering cost", "setup cost", "cost per order", "s",
                   "set-up cost", "order cost per order")
_H_IN = lambda: _i("holding_cost", "holding cost per unit per year", "h", "carrying cost", "holding cost per unit",
                   "annual holding cost")


def eoq(demand, order_cost, holding_cost):
    D, S, H = _pos("demand", demand), _pos("order_cost", order_cost), _pos("holding_cost", holding_cost)
    Q = math.sqrt(2 * D * S / H)
    return _rd({"Q": Q, "orders_per_year": D / Q, "cycle_time": Q / D, "ordering_cost": D * S / Q,
                "holding_cost_per_year": H * Q / 2, "total_cost": math.sqrt(2 * D * S * H)})


def _ref_eoq(demand, order_cost, holding_cost):
    import sympy as sp
    q = sp.Symbol("q", positive=True)
    tc = demand * order_cost / q + holding_cost * q / 2
    Q = _foc_min(tc, q)
    return {"Q": Q, "orders_per_year": demand / Q, "cycle_time": Q / demand, "total_cost": float(tc.subs(q, Q))}


_add("ie_eoq", name="Economic order quantity (EOQ)", topic="inventory / EOQ", level="beginner",
     purpose="The order quantity that minimises annual ordering plus holding cost for constant demand: Q* = sqrt(2DS/H).",
     inputs=[_D_IN(), _S_IN(), _H_IN()],
     outputs=[_o("Q", "economic order quantity Q*"), _o("orders_per_year", "orders per year D/Q*"),
              _o("cycle_time", "time between orders Q*/D (years)"), _o("ordering_cost", "annual ordering cost DS/Q*"),
              _o("holding_cost_per_year", "annual holding cost HQ*/2"), _o("total_cost", "annual ordering plus holding cost")],
     primary="Q", conditions=["demand > 0", "order_cost > 0", "holding_cost > 0 (per unit per year)"],
     method="Q* = sqrt(2 D S / H); TC = D S / Q + H Q / 2 = sqrt(2 D S H)",
     latex=r"Q^* = \sqrt{\frac{2DS}{H}},\quad TC^* = \sqrt{2DSH}", run=eoq, reference=_ref_eoq,
     wrong=lambda demand, order_cost, holding_cost: {"Q": math.sqrt(demand * order_cost / holding_cost),
                                                    "total_cost": math.sqrt(demand * order_cost * holding_cost)},
     rand=lambda rng: {"demand": _u(rng, 100, 100000, 0), "order_cost": _u(rng, 5, 500, 2), "holding_cost": _u(rng, 0.1, 50, 2)},
     triggers=["economic order quantity", "eoq"], ask="economic order quantity",
     examples=[{"demand": 1200, "order_cost": 100, "holding_cost": 6}, {"demand": 8000, "order_cost": 45, "holding_cost": 2.5}],
     invalid=[{"demand": 1200, "order_cost": 100, "holding_cost": 0}])


def inventory_total_cost(demand, order_cost, holding_cost, order_quantity, unit_cost=0.0):
    D, S, H = _pos("demand", demand), _pos("order_cost", order_cost), _pos("holding_cost", holding_cost)
    Q, c = _pos("order_quantity", order_quantity), _nonneg("unit_cost", unit_cost)
    oc, hc = D * S / Q, H * Q / 2
    return _rd({"total_cost": oc + hc + D * c, "ordering_cost": oc, "holding_cost_per_year": hc,
                "purchase_cost": D * c, "average_inventory": Q / 2})


def _ref_total_cost(demand, order_cost, holding_cost, order_quantity, unit_cost=0.0):
    T = order_quantity / demand                            # one cycle: stock falls from Q to 0 at rate D
    avg = _quad(lambda t: order_quantity - demand * t, 0.0, T) / T
    oc = order_cost / T
    return {"total_cost": oc + holding_cost * avg + demand * unit_cost, "average_inventory": avg,
            "ordering_cost": oc, "holding_cost_per_year": holding_cost * avg}


_add("ie_inventory_total_cost", name="Annual inventory cost of an order quantity", topic="inventory / EOQ",
     level="beginner",
     purpose="Annual ordering, holding and purchase cost of ordering a given quantity Q each time (constant demand).",
     inputs=[_D_IN(), _S_IN(), _H_IN(),
             _i("order_quantity", "quantity ordered each time", "q", "lot size", "order size", "quantity ordered"),
             _i("unit_cost", "purchase price per unit (optional, default 0)", "c", "price per unit", "unit price",
                "purchase cost per unit", required=False)],
     outputs=[_o("total_cost", "annual total cost DS/Q + HQ/2 + Dc"), _o("ordering_cost", "annual ordering cost"),
              _o("holding_cost_per_year", "annual holding cost"), _o("purchase_cost", "annual purchase cost"),
              _o("average_inventory", "average inventory Q/2")],
     primary="total_cost", conditions=["demand, order_cost, holding_cost, order_quantity > 0", "unit_cost >= 0"],
     method="TC = D S / Q + H Q / 2 + D c", latex=r"TC(Q) = \frac{D}{Q}S + \frac{Q}{2}H + Dc",
     run=inventory_total_cost, reference=_ref_total_cost,
     wrong=lambda demand, order_cost, holding_cost, order_quantity, unit_cost=0.0: {
         "total_cost": demand * order_cost / order_quantity + holding_cost * order_quantity + demand * unit_cost},
     rand=lambda rng: {"demand": _u(rng, 100, 50000, 0), "order_cost": _u(rng, 5, 400, 2), "holding_cost": _u(rng, 0.2, 40, 2),
                       "order_quantity": _u(rng, 20, 3000, 0), "unit_cost": rng.choice([0, _u(rng, 1, 200, 2)])},
     triggers=["annual inventory cost", "total inventory cost", "total annual inventory cost"],
     ask="total annual inventory cost",
     examples=[{"demand": 1200, "order_cost": 100, "holding_cost": 6, "order_quantity": 300},
               {"demand": 5000, "order_cost": 60, "holding_cost": 4, "order_quantity": 400, "unit_cost": 12}],
     invalid=[{"demand": 1200, "order_cost": 100, "holding_cost": 6, "order_quantity": -5}])


def eoq_backorders(demand, order_cost, holding_cost, backorder_cost):
    D, S, H = _pos("demand", demand), _pos("order_cost", order_cost), _pos("holding_cost", holding_cost)
    B = _pos("backorder_cost", backorder_cost)
    Q = math.sqrt(2 * D * S / H * (H + B) / B)
    b = Q * H / (H + B)
    return _rd({"Q": Q, "max_backorder": b, "max_inventory": Q - b, "total_cost": math.sqrt(2 * D * S * H * B / (H + B)),
                "orders_per_year": D / Q, "fraction_backordered_time": H / (H + B)})


def _ref_backorders(demand, order_cost, holding_cost, backorder_cost):
    import sympy as sp
    q, b = sp.symbols("q b", positive=True)
    H, B = sp.Float(holding_cost, 30), sp.Float(backorder_cost, 30)
    tc = demand * order_cost / q + H * (q - b) ** 2 / (2 * q) + B * b ** 2 / (2 * q)
    bq = sp.solve(sp.diff(tc, b), b)[0]                    # best backorder level for each Q
    reduced = tc.subs(b, bq)
    Q = _foc_min(reduced, q)
    bb = float(bq.subs(q, Q))
    return {"Q": Q, "max_backorder": bb, "max_inventory": Q - bb, "total_cost": float(reduced.subs(q, Q))}


_add("ie_eoq_backorders", name="EOQ with planned backorders", topic="inventory / EOQ", level="intermediate",
     purpose="Order quantity and maximum backorder level minimising ordering, holding and backorder cost when shortages "
             "are allowed and fully backordered.",
     inputs=[_D_IN(), _S_IN(), _H_IN(),
             _i("backorder_cost", "backorder (shortage) cost per unit per year", "b", "shortage cost", "stockout cost",
                "backorder cost per unit", "shortage cost per unit")],
     outputs=[_o("Q", "optimal order quantity"), _o("max_backorder", "maximum backorder level b*"),
              _o("max_inventory", "maximum inventory Q* - b*"), _o("total_cost", "minimum annual cost"),
              _o("orders_per_year", "orders per year"), _o("fraction_backordered_time", "fraction of time out of stock H/(H+B)")],
     primary="Q", conditions=["all inputs > 0", "every shortage is backordered"],
     method="Q* = sqrt(2DS/H (H+B)/B), b* = Q* H/(H+B), TC* = sqrt(2DSH B/(H+B))",
     latex=r"Q^*=\sqrt{\frac{2DS}{H}\cdot\frac{H+B}{B}},\quad b^*=Q^*\frac{H}{H+B}",
     run=eoq_backorders, reference=_ref_backorders,
     wrong=lambda demand, order_cost, holding_cost, backorder_cost: {
         "Q": math.sqrt(2 * demand * order_cost / holding_cost), "total_cost": math.sqrt(2 * demand * order_cost * holding_cost)},
     rand=lambda rng: {"demand": _u(rng, 200, 50000, 0), "order_cost": _u(rng, 10, 400, 2), "holding_cost": _u(rng, 0.5, 30, 2),
                       "backorder_cost": _u(rng, 0.5, 60, 2)},
     triggers=["eoq with backorders", "eoq with planned backorders", "planned backorders", "eoq with planned shortages",
               "eoq with shortages", "backorder model"],
     ask="EOQ with planned backorders",
     examples=[{"demand": 1200, "order_cost": 100, "holding_cost": 6, "backorder_cost": 18},
               {"demand": 20000, "order_cost": 75, "holding_cost": 3, "backorder_cost": 12}],
     invalid=[{"demand": 1200, "order_cost": 100, "holding_cost": 6, "backorder_cost": 0}])


def eoq_discount(demand, order_cost, holding_rate, break_quantities, unit_prices):
    D, S, i = _pos("demand", demand), _pos("order_cost", order_cost), _prob("holding_rate", holding_rate, 0.0, 10.0)
    qs = _vec("break_quantities", break_quantities, 1, 50, nonneg=True)
    ps = _vec("unit_prices", unit_prices, 1, 50, positive=True)
    _need(len(qs) == len(ps), "break_quantities and unit_prices must have the same number of values")
    _need(qs[0] == 0, "the first break quantity must be 0 (the price for any quantity)")
    for k in range(1, len(qs)):
        _need(qs[k] > qs[k - 1], "break quantities must increase")
        _need(ps[k] < ps[k - 1], "unit prices must fall as the break quantity rises")
    best = None
    for k in range(len(qs)):
        H = i * ps[k]
        q = math.sqrt(2 * D * S / H)
        if k + 1 < len(qs) and q >= qs[k + 1]:
            continue                                       # this price's best quantity lies in a cheaper tier
        if q < qs[k]:
            q = qs[k]
        tc = D * S / q + H * q / 2 + D * ps[k]
        if best is None or tc < best[0] - 1e-12 * tc:
            best = (tc, q, ps[k], k + 1)
    tc, q, p, tier = best
    return _rd({"Q": q, "unit_price": p, "total_cost": tc, "tier": tier, "ordering_cost": D * S / q,
                "holding_cost_per_year": i * p * q / 2, "purchase_cost": D * p})


def _ref_discount(demand, order_cost, holding_rate, break_quantities, unit_prices):
    import sympy as sp
    q = sp.Symbol("q", positive=True)
    cands = []
    n = len(break_quantities)
    for k in range(n):
        p = unit_prices[k]
        tc = demand * order_cost / q + holding_rate * p * q / 2 + demand * p
        lo, hi = break_quantities[k], (break_quantities[k + 1] if k + 1 < n else math.inf)
        x = _foc_min(tc, q, 1e-9, 1e13)
        if x >= hi:
            continue
        x = max(x, lo) if lo > 0 else x
        cands.append((float(tc.subs(q, x)), x, p))
    tcm, x, p = min(cands)
    return {"Q": x, "unit_price": p, "total_cost": tcm}


def _rand_discount(rng):
    p0 = _u(rng, 5, 100, 2)
    q1 = rng.randint(50, 2000)
    q2 = q1 + rng.randint(50, 3000)
    return {"demand": _u(rng, 500, 50000, 0), "order_cost": _u(rng, 10, 300, 2), "holding_rate": _u(rng, 0.1, 0.4, 3),
            "break_quantities": [0, q1, q2], "unit_prices": [p0, round(p0 * rng.uniform(0.9, 0.99), 2),
                                                             round(p0 * rng.uniform(0.8, 0.89), 2)]}


_add("ie_eoq_quantity_discount", name="EOQ with all-units quantity discounts", topic="inventory / EOQ",
     level="intermediate",
     purpose="Best order quantity when the unit price falls at quantity breaks (all-units discount) and holding cost is "
             "a fraction of the unit price.",
     inputs=[_D_IN(), _S_IN(),
             _i("holding_rate", "annual holding cost as a fraction of the unit price (decimal; 20% = 0.2)", "i",
                "holding cost rate", "carrying cost rate", "carrying rate", "annual holding rate"),
             _i("break_quantities", "minimum quantity of each price tier, starting with 0", "breaks", "price breaks",
                "quantity breaks", type="vector"),
             _i("unit_prices", "unit price of each tier (falling)", "prices", "unit costs", "price per unit", type="vector")],
     outputs=[_o("Q", "best order quantity"), _o("unit_price", "unit price paid"),
              _o("total_cost", "annual ordering + holding + purchase cost"), _o("tier", "price tier chosen (1 = first)"),
              _o("ordering_cost", "annual ordering cost"), _o("holding_cost_per_year", "annual holding cost"),
              _o("purchase_cost", "annual purchase cost")],
     primary="total_cost",
     conditions=["break_quantities start at 0 and increase", "unit_prices fall tier by tier", "0 < holding_rate < 10"],
     method="for each tier: EOQ at its price, raised to the tier's minimum if below it (skipped if above the next "
            "break); the lowest total cost D S/Q + i p Q/2 + D p wins",
     latex=r"TC_k(Q) = \frac{D}{Q}S + \frac{Q}{2}\,i\,p_k + D p_k",
     run=eoq_discount, reference=_ref_discount,
     wrong=lambda demand, order_cost, holding_rate, break_quantities, unit_prices: {
         "total_cost": min(demand * order_cost / max(math.sqrt(2 * demand * order_cost / (holding_rate * p)), b)
                           + holding_rate * p * max(math.sqrt(2 * demand * order_cost / (holding_rate * p)), b) / 2
                           for b, p in zip(break_quantities, unit_prices))},
     rand=_rand_discount,
     triggers=["quantity discount", "quantity discounts", "all units discount", "all-units discount", "price breaks model"],
     ask="EOQ with quantity discounts",
     examples=[{"demand": 5000, "order_cost": 49, "holding_rate": 0.2, "break_quantities": [0, 1000, 2000],
                "unit_prices": [5.0, 4.8, 4.75]},
               {"demand": 1200, "order_cost": 100, "holding_rate": 0.25, "break_quantities": [0, 300, 600],
                "unit_prices": [20, 19, 18.5]}],
     invalid=[{"demand": 5000, "order_cost": 49, "holding_rate": 0.2, "break_quantities": [0, 1000, 2000],
               "unit_prices": [5.0, 5.2, 4.75]}])


def epq(demand, production_rate, order_cost, holding_cost):
    D, P = _pos("demand", demand), _pos("production_rate", production_rate)
    S, H = _pos("order_cost", order_cost), _pos("holding_cost", holding_cost)
    _need(P > D, "the production rate must exceed the demand rate")
    f = 1 - D / P
    Q = math.sqrt(2 * D * S / (H * f))
    return _rd({"Q": Q, "max_inventory": Q * f, "production_run_time": Q / P, "cycle_time": Q / D,
                "runs_per_year": D / Q, "total_cost": math.sqrt(2 * D * S * H * f)})


def _ref_epq(demand, production_rate, order_cost, holding_cost):
    import sympy as sp
    q = sp.Symbol("q", positive=True)
    # average stock of one cycle from its triangular profile (build-up at P - D, then depletion at D), by quadrature
    def avg_inv(Q):
        tp, T = Q / production_rate, Q / demand
        peak = (production_rate - demand) * tp
        area = _quad(lambda t: (production_rate - demand) * t, 0.0, tp) + _quad(lambda t: peak - demand * (t - tp), tp, T)
        return area / T
    k = avg_inv(1.0)                                       # average stock is proportional to Q
    tc = demand * order_cost / q + holding_cost * k * q
    Q = _foc_min(tc, q)
    return {"Q": Q, "max_inventory": Q * (1 - demand / production_rate), "total_cost": float(tc.subs(q, Q)),
            "production_run_time": Q / production_rate}


_add("ie_epq", name="Economic production quantity (EPQ)", topic="inventory / EOQ", level="intermediate",
     purpose="Production lot size minimising setup plus holding cost when stock builds up gradually at a finite "
             "production rate.",
     inputs=[_D_IN(), _i("production_rate", "production rate per year (same units as demand)", "p",
                         "annual production rate", "production per year", "production rate per year"),
             _S_IN(), _H_IN()],
     outputs=[_o("Q", "economic production quantity"), _o("max_inventory", "maximum inventory Q(1 - D/P)"),
              _o("production_run_time", "length of a production run Q/P (years)"), _o("cycle_time", "cycle Q/D (years)"),
              _o("runs_per_year", "production runs per year"), _o("total_cost", "annual setup plus holding cost")],
     primary="Q", conditions=["all inputs > 0", "production_rate > demand"],
     method="Q* = sqrt(2DS / (H (1 - D/P))), I_max = Q*(1 - D/P)",
     latex=r"Q^* = \sqrt{\frac{2DS}{H\,(1 - D/P)}}", run=epq, reference=_ref_epq,
     wrong=lambda demand, production_rate, order_cost, holding_cost: {
         "Q": math.sqrt(2 * demand * order_cost / holding_cost), "max_inventory": math.sqrt(2 * demand * order_cost / holding_cost)},
     rand=lambda rng: (lambda D: {"demand": D, "production_rate": round(D * rng.uniform(1.3, 6), 0),
                                  "order_cost": _u(rng, 20, 800, 2), "holding_cost": _u(rng, 0.3, 30, 2)})(_u(rng, 500, 50000, 0)),
     triggers=["economic production quantity", "epq", "production lot size", "economic lot size",
               "production order quantity"], ask="economic production quantity",
     examples=[{"demand": 12000, "production_rate": 48000, "order_cost": 400, "holding_cost": 3},
               {"demand": 1000, "production_rate": 4000, "order_cost": 10, "holding_cost": 0.5}],
     invalid=[{"demand": 12000, "production_rate": 9000, "order_cost": 400, "holding_cost": 3}])


def reorder_point(demand_rate, lead_time, safety_stock=0.0):
    d, L, ss = _pos("demand_rate", demand_rate), _nonneg("lead_time", lead_time), _nonneg("safety_stock", safety_stock)
    return _rd({"reorder_point": d * L + ss, "lead_time_demand": d * L})


def _ref_rop(demand_rate, lead_time, safety_stock=0.0):
    ltd = _quad(lambda t: demand_rate, 0.0, lead_time) if lead_time > 0 else 0.0
    return {"reorder_point": ltd + safety_stock, "lead_time_demand": ltd}


_add("ie_reorder_point", name="Reorder point (constant demand)", topic="inventory / reorder point", level="beginner",
     purpose="Inventory level at which to reorder: demand during the lead time plus any safety stock.",
     inputs=[_i("demand_rate", "demand per period (per day, say)", "daily demand", "demand per day", "d", "demand rate",
                "usage rate"),
             _i("lead_time", "lead time in the same periods", "l", "lead time in days", "replenishment lead time"),
             _i("safety_stock", "safety stock (optional, default 0)", "ss", "buffer stock", required=False)],
     outputs=[_o("reorder_point", "reorder point d L + SS"), _o("lead_time_demand", "demand during the lead time d L")],
     primary="reorder_point", conditions=["demand_rate > 0", "lead_time >= 0", "safety_stock >= 0"],
     method="ROP = d L + SS", latex=r"ROP = d\,L + SS", run=reorder_point, reference=_ref_rop,
     wrong=lambda demand_rate, lead_time, safety_stock=0.0: {"reorder_point": demand_rate * lead_time * 0 + demand_rate + safety_stock},
     rand=lambda rng: {"demand_rate": _u(rng, 1, 2000, 1), "lead_time": rng.choice([rng.randint(2, 40), _u(rng, 1.5, 30, 1)]),
                       "safety_stock": rng.choice([0, _u(rng, 5, 500, 0)])},
     triggers=["reorder point", "re-order point", "rop"], ask="reorder point",
     examples=[{"demand_rate": 40, "lead_time": 5}, {"demand_rate": 120, "lead_time": 3.5, "safety_stock": 60}],
     invalid=[{"demand_rate": 40, "lead_time": -2}],
     weak="the reference integrates the constant demand rate over the lead time (the same definition)")


def safety_stock_normal(mean_demand, sd_demand, lead_time, service_level):
    d, s = _pos("mean_demand", mean_demand), _pos("sd_demand", sd_demand)
    L, csl = _pos("lead_time", lead_time), _prob("service_level", service_level, 0.5, 1.0)
    z = _zinv(csl)
    sl = s * math.sqrt(L)
    return _rd({"safety_stock": z * sl, "reorder_point": d * L + z * sl, "z": z, "sd_lead_time_demand": sl,
                "lead_time_demand": d * L})


def _ref_ss_normal(mean_demand, sd_demand, lead_time, service_level):
    from scipy.stats import norm
    sl = float(norm(0, sd_demand).std() * math.sqrt(lead_time))
    rop = float(norm.ppf(service_level, loc=mean_demand * lead_time, scale=sl))
    return {"safety_stock": rop - mean_demand * lead_time, "reorder_point": rop, "z": float(norm.ppf(service_level))}


_SL_IN = lambda: _i("service_level", "cycle service level: probability of no stock-out per cycle (decimal)",
                    "csl", "cycle service level", "service", "in-stock probability")
_MD_IN = lambda: _i("mean_demand", "mean demand per period (per day, say)", "average daily demand", "mean daily demand",
                    "average demand", "d", "daily demand")
_SD_IN = lambda: _i("sd_demand", "standard deviation of demand per period", "standard deviation of daily demand",
                    "sigma", "demand standard deviation", "sd of daily demand", "sigma d")

_add("ie_safety_stock", name="Safety stock and reorder point (normal demand)", topic="inventory / safety stock",
     level="intermediate",
     purpose="Safety stock and reorder point for a cycle service level when demand per period is normal and the lead "
             "time is fixed.",
     inputs=[_MD_IN(), _SD_IN(), _i("lead_time", "lead time in periods", "l", "lead time in days"), _SL_IN()],
     outputs=[_o("safety_stock", "safety stock z sigma sqrt(L)"), _o("reorder_point", "reorder point d L + SS"),
              _o("z", "service factor z"), _o("sd_lead_time_demand", "standard deviation of lead-time demand"),
              _o("lead_time_demand", "mean lead-time demand")],
     primary="safety_stock", conditions=["mean_demand, sd_demand, lead_time > 0", "0.5 < service_level < 1",
                                         "demand per period independent and normal"],
     method="sigma_L = sigma_d sqrt(L), z = Phi^-1(CSL), SS = z sigma_L, ROP = d L + SS",
     latex=r"SS = z\,\sigma_d\sqrt{L},\quad ROP = \bar d L + SS", run=safety_stock_normal, reference=_ref_ss_normal,
     wrong=lambda mean_demand, sd_demand, lead_time, service_level: {
         "safety_stock": _zinv(service_level) * sd_demand * lead_time},
     rand=lambda rng: {"mean_demand": _u(rng, 5, 1000, 1), "sd_demand": _u(rng, 1, 200, 1),
                       "lead_time": rng.choice([rng.randint(2, 30), _u(rng, 2, 20, 1)]), "service_level": _u(rng, 0.8, 0.995, 3)},
     triggers=["safety stock", "safety stock and reorder point"], ask="safety stock",
     examples=[{"mean_demand": 50, "sd_demand": 12, "lead_time": 9, "service_level": 0.95},
               {"mean_demand": 200, "sd_demand": 40, "lead_time": 4, "service_level": 0.99}],
     invalid=[{"mean_demand": 50, "sd_demand": 12, "lead_time": 9, "service_level": 1.2}])


def safety_stock_vlt(mean_demand, sd_demand, mean_lead_time, sd_lead_time, service_level):
    d, s = _pos("mean_demand", mean_demand), _nonneg("sd_demand", sd_demand)
    L, sL = _pos("mean_lead_time", mean_lead_time), _nonneg("sd_lead_time", sd_lead_time)
    csl = _prob("service_level", service_level, 0.5, 1.0)
    _need(s > 0 or sL > 0, "demand or lead time must vary (a standard deviation must be positive)")
    sig = math.sqrt(L * s * s + d * d * sL * sL)
    z = _zinv(csl)
    return _rd({"safety_stock": z * sig, "reorder_point": d * L + z * sig, "sd_lead_time_demand": sig, "z": z})


def _ref_ss_vlt(mean_demand, sd_demand, mean_lead_time, sd_lead_time, service_level):
    from scipy.stats import norm
    # law of total variance over the lead time L: E[Var(D|L)] + Var(E[D|L]), each by quadrature over L's density
    if sd_lead_time > 0:
        f = norm(mean_lead_time, sd_lead_time).pdf
        lo, hi = mean_lead_time - 40 * sd_lead_time, mean_lead_time + 40 * sd_lead_time
        e_var = _quad(lambda t: sd_demand ** 2 * t * f(t), lo, hi)
        var_e = _quad(lambda t: (mean_demand * (t - mean_lead_time)) ** 2 * f(t), lo, hi)
    else:
        e_var, var_e = sd_demand ** 2 * mean_lead_time, 0.0
    sig = math.sqrt(e_var + var_e)
    z = float(norm.ppf(service_level))
    return {"safety_stock": z * sig, "sd_lead_time_demand": sig, "reorder_point": mean_demand * mean_lead_time + z * sig}


_add("ie_safety_stock_variable_lead_time", name="Safety stock with variable demand and lead time",
     topic="inventory / safety stock", level="advanced",
     purpose="Safety stock and reorder point when both the demand per period and the lead time vary.",
     inputs=[_MD_IN(), _SD_IN(),
             _i("mean_lead_time", "mean lead time in periods", "average lead time", "mean lead time in days", "l"),
             _i("sd_lead_time", "standard deviation of the lead time", "standard deviation of lead time",
                "lead time standard deviation", "sigma l", "sd of lead time"),
             _SL_IN()],
     outputs=[_o("safety_stock", "safety stock z sigma_DL"), _o("reorder_point", "reorder point d L + SS"),
              _o("sd_lead_time_demand", "sigma_DL = sqrt(L sigma_d^2 + d^2 sigma_L^2)"), _o("z", "service factor")],
     primary="safety_stock",
     conditions=["mean_demand, mean_lead_time > 0", "standard deviations >= 0, not both 0", "0.5 < service_level < 1",
                 "demand and lead time independent; lead-time demand treated as normal"],
     method="sigma_DL = sqrt(L sigma_d^2 + d^2 sigma_L^2), SS = z sigma_DL",
     latex=r"\sigma_{DL} = \sqrt{\bar L\,\sigma_d^2 + \bar d^{\,2}\sigma_L^2},\quad SS = z\,\sigma_{DL}",
     run=safety_stock_vlt, reference=_ref_ss_vlt,
     wrong=lambda mean_demand, sd_demand, mean_lead_time, sd_lead_time, service_level: {
         "safety_stock": _zinv(service_level) * (sd_demand * math.sqrt(mean_lead_time) + mean_demand * sd_lead_time)},
     rand=lambda rng: {"mean_demand": _u(rng, 5, 800, 1), "sd_demand": _u(rng, 1, 150, 1), "mean_lead_time": _u(rng, 3, 30, 1),
                       "sd_lead_time": _u(rng, 0.3, 5, 2), "service_level": _u(rng, 0.8, 0.995, 3)},
     triggers=["safety stock with variable lead time", "variable lead time safety stock", "variable lead time",
               "uncertain lead time"], ask="safety stock with variable lead time",
     examples=[{"mean_demand": 100, "sd_demand": 20, "mean_lead_time": 6, "sd_lead_time": 2, "service_level": 0.95},
               {"mean_demand": 40, "sd_demand": 10, "mean_lead_time": 12, "sd_lead_time": 1.5, "service_level": 0.98}],
     invalid=[{"mean_demand": 100, "sd_demand": 20, "mean_lead_time": 6, "sd_lead_time": -2, "service_level": 0.95}])


_P_IN = lambda: _i("price", "selling price per unit", "selling price", "p", "sale price", "retail price")
_C_IN = lambda: _i("cost", "purchase cost per unit", "unit cost", "c", "purchase cost", "cost per unit")
_V_IN = lambda: _i("salvage", "salvage value per unsold unit (optional, default 0)", "salvage value", "v",
                   "residual value", required=False)


def _nv_costs(price, cost, salvage):
    p, c, v = _pos("price", price), _pos("cost", cost), _fin("salvage", salvage)
    _need(v < c < p, "the newsvendor needs salvage < cost < price")
    return p, c, v, (p - c) / (p - v)


def newsvendor_normal(price, cost, mean_demand, sd_demand, salvage=0.0):
    p, c, v, cr = _nv_costs(price, cost, salvage)
    mu, sd = _fin("mean_demand", mean_demand), _pos("sd_demand", sd_demand)
    z = _zinv(cr)
    Q = mu + z * sd
    sales = mu - sd * _loss(z)
    left = Q - sales
    return _rd({"Q": Q, "critical_ratio": cr, "z": z, "expected_sales": sales, "expected_leftover": left,
                "expected_shortage": mu - sales, "expected_profit": p * sales + v * left - c * Q})


def _ref_nv_normal(price, cost, mean_demand, sd_demand, salvage=0.0):
    from scipy.stats import norm
    cu, co = price - cost, cost - salvage
    Q = float(norm.ppf(cu / (cu + co), mean_demand, sd_demand))
    f = norm(mean_demand, sd_demand).pdf
    lo, hi = mean_demand - 40 * sd_demand, mean_demand + 40 * sd_demand
    sales = _quad(lambda x: x * f(x), lo, Q) + Q * float(norm.sf(Q, mean_demand, sd_demand))
    return {"Q": Q, "critical_ratio": cu / (cu + co), "expected_sales": sales,
            "expected_profit": price * sales + salvage * (Q - sales) - cost * Q}


_add("ie_newsvendor_normal", name="Newsvendor order quantity (normal demand)", topic="inventory / newsvendor",
     level="intermediate",
     purpose="Single-period order quantity that maximises expected profit when demand is normal: the critical ratio "
             "Cu/(Cu + Co).",
     inputs=[_P_IN(), _C_IN(), _MD_IN(), _SD_IN(), _V_IN()],
     outputs=[_o("Q", "optimal order quantity mu + z sigma"), _o("critical_ratio", "critical ratio Cu/(Cu+Co)"),
              _o("z", "z of the critical ratio"), _o("expected_sales", "expected units sold"),
              _o("expected_leftover", "expected units left over"), _o("expected_shortage", "expected unmet demand"),
              _o("expected_profit", "expected profit")],
     primary="Q", conditions=["salvage < cost < price", "sd_demand > 0", "demand normal"],
     method="Cu = p - c, Co = c - v, CR = Cu/(Cu+Co), Q = mu + Phi^-1(CR) sigma; E[sales] = mu - sigma L(z)",
     latex=r"F(Q^*) = \frac{C_u}{C_u + C_o},\quad Q^* = \mu + z\sigma", run=newsvendor_normal, reference=_ref_nv_normal,
     wrong=lambda price, cost, mean_demand, sd_demand, salvage=0.0: {
         "Q": mean_demand + _zinv((cost - salvage) / (price - salvage)) * sd_demand},
     rand=lambda rng: (lambda c: {"price": round(c * rng.uniform(1.2, 4), 2), "cost": c,
                                  "salvage": rng.choice([0, round(c * rng.uniform(0.05, 0.6), 2)]),
                                  "mean_demand": _u(rng, 50, 5000, 0), "sd_demand": _u(rng, 10, 800, 1)})(_u(rng, 1, 80, 2)),
     triggers=["newsvendor", "newsboy", "single period inventory", "single-period order quantity"],
     ask="newsvendor order quantity",
     examples=[{"price": 10, "cost": 6, "mean_demand": 500, "sd_demand": 100, "salvage": 2},
               {"price": 5, "cost": 1.5, "mean_demand": 1000, "sd_demand": 250}],
     invalid=[{"price": 5, "cost": 6, "mean_demand": 500, "sd_demand": 100}])


def newsvendor_uniform(price, cost, low, high, salvage=0.0):
    p, c, v, cr = _nv_costs(price, cost, salvage)
    a, b = _fin("low", low), _fin("high", high)
    _need(b > a, "high must exceed low")
    Q = a + cr * (b - a)
    sales = Q - (Q - a) ** 2 / (2 * (b - a))
    return _rd({"Q": Q, "critical_ratio": cr, "expected_sales": sales, "expected_leftover": Q - sales,
                "expected_profit": p * sales + v * (Q - sales) - c * Q})


def _ref_nv_uniform(price, cost, low, high, salvage=0.0):
    from scipy.stats import uniform
    dist = uniform(loc=low, scale=high - low)
    cr = (price - cost) / (price - salvage)
    Q = float(dist.ppf(cr))
    sales = _quad(lambda x: x * dist.pdf(x), low, Q) + Q * float(dist.sf(Q))
    return {"Q": Q, "critical_ratio": cr, "expected_sales": sales,
            "expected_profit": price * sales + salvage * (Q - sales) - cost * Q}


_add("ie_newsvendor_uniform", name="Newsvendor order quantity (uniform demand)", topic="inventory / newsvendor",
     level="intermediate",
     purpose="Single-period order quantity and expected profit when demand is uniform between two bounds.",
     inputs=[_P_IN(), _C_IN(), _i("low", "lowest possible demand", "minimum demand", "a", "lower bound"),
             _i("high", "highest possible demand", "maximum demand", "b", "upper bound"), _V_IN()],
     outputs=[_o("Q", "optimal order quantity a + CR (b - a)"), _o("critical_ratio", "critical ratio"),
              _o("expected_sales", "expected units sold"), _o("expected_leftover", "expected units left over"),
              _o("expected_profit", "expected profit")],
     primary="Q", conditions=["salvage < cost < price", "high > low", "demand uniform on [low, high]"],
     method="Q = a + CR (b - a), E[sales] = Q - (Q - a)^2 / (2 (b - a))",
     latex=r"Q^* = a + \frac{C_u}{C_u + C_o}(b - a)", run=newsvendor_uniform, reference=_ref_nv_uniform,
     wrong=lambda price, cost, low, high, salvage=0.0: {"Q": low + (cost - salvage) / (price - salvage) * (high - low)},
     rand=lambda rng: (lambda c, a: {"price": round(c * rng.uniform(1.2, 4), 2), "cost": c,
                                     "salvage": rng.choice([0, round(c * rng.uniform(0.05, 0.6), 2)]),
                                     "low": a, "high": a + _u(rng, 20, 3000, 0)})(_u(rng, 1, 80, 2), _u(rng, 0, 2000, 0)),
     triggers=["newsvendor with uniform demand", "uniform demand newsvendor", "newsvendor uniform"],
     ask="newsvendor with uniform demand",
     examples=[{"price": 12, "cost": 7, "low": 100, "high": 300, "salvage": 3},
               {"price": 8, "cost": 5, "low": 0, "high": 1000}],
     invalid=[{"price": 12, "cost": 7, "low": 300, "high": 100}])


def newsvendor_poisson(price, cost, mean_demand, salvage=0.0):
    p, c, v, cr = _nv_costs(price, cost, salvage)
    lam = _pos("mean_demand", mean_demand)
    _need(lam <= 500, "mean_demand must be at most 500 for the Poisson model")
    kmax = int(lam + 40 * math.sqrt(lam) + 40)
    pm = _pois_pmfs(lam, kmax)
    F, Q = 0.0, None
    for k in range(kmax + 1):
        F += pm[k]
        if abs(F - cr) <= 1e-12:
            raise ValueError("two order quantities are equally good (the cumulative probability equals the critical ratio)")
        if F > cr:
            Q = k
            break
    short = 0.0
    for k in range(Q):
        short += (Q - k) * pm[k]                          # expected leftover
    sales = Q - short
    return _rd({"Q": Q, "critical_ratio": cr, "expected_sales": sales, "expected_leftover": short,
                "expected_profit": p * sales + v * short - c * Q, "stockout_probability": 1 - F})


def _ref_nv_poisson(price, cost, mean_demand, salvage=0.0):
    from scipy.stats import poisson
    cr = (price - cost) / (price - salvage)
    Q = int(poisson.ppf(cr, mean_demand))
    left = sum((Q - k) * float(poisson.pmf(k, mean_demand)) for k in range(Q))
    sales = Q - left
    return {"Q": Q, "critical_ratio": cr, "expected_sales": sales,
            "expected_profit": price * sales + salvage * left - cost * Q, "stockout_probability": float(poisson.sf(Q, mean_demand))}


_add("ie_newsvendor_poisson", name="Newsvendor order quantity (Poisson demand)", topic="inventory / newsvendor",
     level="advanced",
     purpose="Whole-unit single-period order quantity when demand is Poisson: the smallest Q with F(Q) >= critical ratio.",
     inputs=[_P_IN(), _C_IN(), _i("mean_demand", "mean (Poisson) demand", "average demand", "lambda", "mean"), _V_IN()],
     outputs=[_o("Q", "optimal whole order quantity"), _o("critical_ratio", "critical ratio"),
              _o("expected_sales", "expected units sold"), _o("expected_leftover", "expected units left over"),
              _o("expected_profit", "expected profit"), _o("stockout_probability", "probability demand exceeds Q")],
     primary="expected_profit", conditions=["salvage < cost < price", "0 < mean_demand <= 500", "demand Poisson"],
     method="smallest Q with P(D <= Q) >= Cu/(Cu+Co); E[leftover] = sum_{k<Q} (Q - k) P(D = k)",
     latex=r"Q^* = \min\{Q : F(Q) \ge \tfrac{C_u}{C_u+C_o}\}", run=newsvendor_poisson, reference=_ref_nv_poisson,
     wrong=lambda price, cost, mean_demand, salvage=0.0: {"expected_profit": (price - cost) * mean_demand},
     rand=lambda rng: (lambda c: {"price": round(c * rng.uniform(1.2, 4), 2), "cost": c,
                                  "salvage": rng.choice([0, round(c * rng.uniform(0.05, 0.6), 2)]),
                                  "mean_demand": _u(rng, 1, 200, 1)})(_u(rng, 1, 80, 2)),
     triggers=["newsvendor with poisson demand", "poisson demand newsvendor", "newsvendor poisson"],
     ask="newsvendor with Poisson demand",
     examples=[{"price": 20, "cost": 8, "mean_demand": 12}, {"price": 50, "cost": 30, "mean_demand": 40, "salvage": 10}],
     invalid=[{"price": 20, "cost": 8, "mean_demand": -3}])


def newsvendor_discrete(price, cost, demands, probabilities, salvage=0.0):
    p, c, v, cr = _nv_costs(price, cost, salvage)
    d = _vec("demands", demands, 2, 200, nonneg=True)
    pr = _vec("probabilities", probabilities, 2, 200, nonneg=True)
    _need(len(d) == len(pr), "demands and probabilities must have the same number of values")
    for k in range(1, len(d)):
        _need(d[k] > d[k - 1], "demands must be listed in increasing order")
    tot = sum(pr)
    _need(abs(tot - 1.0) <= 1e-9, "the probabilities must add up to 1")
    F, Q = 0.0, None
    for k in range(len(d)):
        F += pr[k]
        if abs(F - cr) <= 1e-12 and k < len(d) - 1:
            raise ValueError("two order quantities are equally good (the cumulative probability equals the critical ratio)")
        if F > cr or k == len(d) - 1:
            Q = d[k]
            break
    sales = 0.0
    for k in range(len(d)):
        sales += pr[k] * min(d[k], Q)
    return _rd({"Q": Q, "critical_ratio": cr, "expected_sales": sales,
                "expected_profit": p * sales + v * (Q - sales) - c * Q})


def _ref_nv_discrete(price, cost, demands, probabilities, salvage=0.0):
    import numpy as np
    d, w = np.asarray(demands, float), np.asarray(probabilities, float)
    profits = [float(np.sum(w * (price * np.minimum(d, q) + salvage * np.maximum(q - d, 0)))) - cost * q for q in d]
    k = int(np.argmax(profits))
    return {"Q": float(d[k]), "expected_profit": profits[k]}


def _rand_nv_discrete(rng):
    c = _u(rng, 1, 50, 2)
    n = rng.randint(3, 8)
    d = sorted(rng.sample(range(0, 500, 10), n))
    w = [rng.randint(1, 9) for _ in range(n)]
    s = sum(w)
    pr = [round(x / s, 6) for x in w[:-1]]
    pr.append(round(1 - sum(pr), 6))
    return {"price": round(c * rng.uniform(1.2, 4), 2), "cost": c, "demands": d, "probabilities": pr,
            "salvage": rng.choice([0, round(c * rng.uniform(0.05, 0.6), 2)])}


_add("ie_newsvendor_discrete", name="Newsvendor order quantity (discrete demand table)", topic="inventory / newsvendor",
     level="advanced",
     purpose="Best single-period order quantity among the listed demand values, given their probabilities.",
     inputs=[_P_IN(), _C_IN(), _i("demands", "possible demand values (increasing)", "demand values", "demand levels",
                                   type="vector"),
             _i("probabilities", "probability of each demand value (sum 1)", "probs", "demand probabilities", type="vector"),
             _V_IN()],
     outputs=[_o("Q", "optimal order quantity (one of the demand values)"), _o("critical_ratio", "critical ratio"),
              _o("expected_sales", "expected units sold"), _o("expected_profit", "expected profit")],
     primary="expected_profit",
     conditions=["salvage < cost < price", "demands increasing", "probabilities add to 1",
                 "refused when two quantities are equally good"],
     method="smallest demand value whose cumulative probability exceeds Cu/(Cu+Co); expected profit sum p_k profit(Q, d_k)",
     latex=r"Q^* = \min\{d_k : F(d_k) \ge \tfrac{C_u}{C_u+C_o}\}", run=newsvendor_discrete, reference=_ref_nv_discrete,
     wrong=lambda price, cost, demands, probabilities, salvage=0.0: {
         "expected_profit": sum(w * (price - cost) * x for x, w in zip(demands, probabilities))},
     rand=_rand_nv_discrete,
     triggers=["newsvendor with discrete demand", "discrete demand newsvendor", "newsvendor discrete",
               "payoff table order quantity"], ask="newsvendor with discrete demand",
     examples=[{"price": 10, "cost": 4, "demands": [20, 30, 40, 50], "probabilities": [0.2, 0.3, 0.3, 0.2]},
               {"price": 6, "cost": 3.5, "demands": [100, 200, 300], "probabilities": [0.25, 0.5, 0.25], "salvage": 1}],
     invalid=[{"price": 10, "cost": 4, "demands": [20, 30, 40, 50], "probabilities": [0.2, 0.3, 0.3, 0.3]}])


def rq_fill_rate(mean_ltd, sd_ltd, order_quantity, fill_rate):
    mu, s = _fin("mean_ltd", mean_ltd), _pos("sd_ltd", sd_ltd)
    Q, beta = _pos("order_quantity", order_quantity), _prob("fill_rate", fill_rate)
    target = (1 - beta) * Q / s                            # required L(z)
    _need(target < _loss(-8.0), "the fill rate is too low for this order quantity (the reorder point would be negative)")
    _need(target > _loss(8.0), "the fill rate is too close to 1 for this order quantity")
    z = _bisect_root(lambda x: _loss(x) - target, -8.0, 8.0)
    return _rd({"reorder_point": mu + z * s, "safety_stock": z * s, "z": z, "expected_shortage_per_cycle": target * s,
                "cycle_service_level": _cdf(z)})


def _ref_rq(mean_ltd, sd_ltd, order_quantity, fill_rate):
    from scipy.stats import norm
    esc = (1 - fill_rate) * order_quantity
    g = lambda r: sd_ltd * float(norm.pdf((r - mean_ltd) / sd_ltd)) - (r - mean_ltd) * float(norm.sf(r, mean_ltd, sd_ltd)) - esc
    R = _brentq(g, mean_ltd - 8 * sd_ltd, mean_ltd + 8 * sd_ltd)
    return {"reorder_point": R, "safety_stock": R - mean_ltd, "cycle_service_level": float(norm.cdf(R, mean_ltd, sd_ltd))}


_add("ie_rq_fill_rate", name="(R, Q) reorder point for a target fill rate", topic="inventory / continuous review",
     level="expert",
     purpose="Reorder point of a continuous-review (R, Q) policy that meets a fill rate (fraction of demand met from "
             "stock) with normal lead-time demand.",
     inputs=[_i("mean_ltd", "mean demand during the lead time", "mean lead time demand", "lead time demand",
                "mean lead-time demand", "mu"),
             _i("sd_ltd", "standard deviation of lead-time demand", "standard deviation of lead time demand",
                "sigma", "sd of lead time demand", "sigma l"),
             _i("order_quantity", "order quantity Q", "q", "lot size"),
             _i("fill_rate", "target fill rate (decimal)", "beta", "target fill rate", "item fill rate")],
     outputs=[_o("reorder_point", "reorder point R"), _o("safety_stock", "safety stock R - mu"), _o("z", "service factor"),
              _o("expected_shortage_per_cycle", "expected units short per cycle (1 - beta) Q"),
              _o("cycle_service_level", "probability of no stock-out per cycle")],
     primary="reorder_point", conditions=["sd_ltd, order_quantity > 0", "0 < fill_rate < 1", "lead-time demand normal"],
     method="solve sigma L(z) = (1 - beta) Q for z (L the standard normal loss function); R = mu + z sigma",
     latex=r"\sigma_L\,L(z) = (1-\beta)\,Q,\quad R = \mu_L + z\,\sigma_L", run=rq_fill_rate, reference=_ref_rq,
     wrong=lambda mean_ltd, sd_ltd, order_quantity, fill_rate: {"reorder_point": mean_ltd + _zinv(fill_rate) * sd_ltd},
     rand=lambda rng: (lambda s: {"mean_ltd": _u(rng, 50, 5000, 0), "sd_ltd": s, "order_quantity": round(s * rng.uniform(1, 20), 0),
                                  "fill_rate": _u(rng, 0.9, 0.995, 3)})(_u(rng, 10, 400, 1)),
     triggers=["fill rate reorder point", "reorder point for a fill rate", "r q policy", "(r, q) policy", "rq policy",
               "reorder point for fill rate"], ask="reorder point for a fill rate",
     examples=[{"mean_ltd": 500, "sd_ltd": 100, "order_quantity": 1000, "fill_rate": 0.99},
               {"mean_ltd": 200, "sd_ltd": 40, "order_quantity": 150, "fill_rate": 0.95}],
     invalid=[{"mean_ltd": 500, "sd_ltd": 100, "order_quantity": 1000, "fill_rate": 1.5}])


def service_from_rop(mean_ltd, sd_ltd, reorder_point, order_quantity=None):
    mu, s, R = _fin("mean_ltd", mean_ltd), _pos("sd_ltd", sd_ltd), _fin("reorder_point", reorder_point)
    z = (R - mu) / s
    esc = s * _loss(z)
    out = {"cycle_service_level": _cdf(z), "z": z, "safety_stock": R - mu, "expected_shortage_per_cycle": esc}
    if order_quantity is not None:
        Q = _pos("order_quantity", order_quantity)
        out["fill_rate"] = max(0.0, 1 - esc / Q)
    return _rd(out)


def _ref_service_rop(mean_ltd, sd_ltd, reorder_point, order_quantity=None):
    from scipy.stats import norm
    f = norm(mean_ltd, sd_ltd).pdf
    esc = _quad(lambda x: (x - reorder_point) * f(x), reorder_point, mean_ltd + 40 * sd_ltd)
    out = {"cycle_service_level": float(norm.cdf(reorder_point, mean_ltd, sd_ltd)), "expected_shortage_per_cycle": esc}
    if order_quantity is not None:
        out["fill_rate"] = max(0.0, 1 - esc / order_quantity)
    return out


_add("ie_service_level_from_reorder_point", name="Service level of a reorder point", topic="inventory / continuous review",
     level="advanced",
     purpose="Cycle service level, expected shortage per cycle and fill rate achieved by a given reorder point with "
             "normal lead-time demand.",
     inputs=[_i("mean_ltd", "mean demand during the lead time", "mean lead time demand", "lead time demand", "mu"),
             _i("sd_ltd", "standard deviation of lead-time demand", "standard deviation of lead time demand", "sigma"),
             _i("reorder_point", "reorder point R", "r", "rop"),
             _i("order_quantity", "order quantity Q (optional; gives the fill rate)", "q", "lot size", required=False)],
     outputs=[_o("cycle_service_level", "P(no stock-out in a cycle) = Phi(z)"), _o("z", "service factor (R - mu)/sigma"),
              _o("safety_stock", "safety stock R - mu"), _o("expected_shortage_per_cycle", "sigma L(z)"),
              _o("fill_rate", "fill rate 1 - ESC/Q (when Q is given)")],
     primary="expected_shortage_per_cycle", conditions=["sd_ltd > 0", "order_quantity > 0 when given"],
     method="z = (R - mu)/sigma, CSL = Phi(z), ESC = sigma L(z), fill rate = 1 - ESC/Q",
     latex=r"CSL = \Phi(z),\quad ESC = \sigma_L\,L(z),\quad \beta = 1 - \frac{ESC}{Q}",
     run=service_from_rop, reference=_ref_service_rop,
     wrong=lambda mean_ltd, sd_ltd, reorder_point, order_quantity=None: {
         "expected_shortage_per_cycle": sd_ltd * _pdf((reorder_point - mean_ltd) / sd_ltd)},
     rand=lambda rng: (lambda mu, s: _drop_none({"mean_ltd": mu, "sd_ltd": s, "reorder_point": round(mu + s * rng.uniform(-1, 3), 1),
                                      "order_quantity": rng.choice([None, round(s * rng.uniform(1, 20), 0)])}))(
                                          _u(rng, 50, 5000, 0), _u(rng, 10, 400, 1)),
     triggers=["service level of a reorder point", "cycle service level", "expected shortage per cycle",
               "service level from reorder point"], ask="service level of a reorder point",
     examples=[{"mean_ltd": 500, "sd_ltd": 100, "reorder_point": 650, "order_quantity": 1000},
               {"mean_ltd": 240, "sd_ltd": 30, "reorder_point": 260}],
     invalid=[{"mean_ltd": 500, "sd_ltd": 0, "reorder_point": 650}])


def order_up_to(mean_demand, sd_demand, review_period, lead_time, service_level, inventory_position=None):
    d, s = _pos("mean_demand", mean_demand), _pos("sd_demand", sd_demand)
    T, L = _pos("review_period", review_period), _nonneg("lead_time", lead_time)
    csl = _prob("service_level", service_level, 0.5, 1.0)
    z = _zinv(csl)
    sig = s * math.sqrt(T + L)
    S = d * (T + L) + z * sig
    out = {"order_up_to_level": S, "safety_stock": z * sig, "z": z, "sd_protection_demand": sig}
    if inventory_position is not None:
        ip = _fin("inventory_position", inventory_position)
        out["order_quantity"] = max(0.0, S - ip)
    return _rd(out)


def _ref_order_up_to(mean_demand, sd_demand, review_period, lead_time, service_level, inventory_position=None):
    from scipy.stats import norm
    n = review_period + lead_time
    S = float(norm.ppf(service_level, mean_demand * n, sd_demand * math.sqrt(n)))
    out = {"order_up_to_level": S, "safety_stock": S - mean_demand * n}
    if inventory_position is not None:
        out["order_quantity"] = max(0.0, S - inventory_position)
    return out


_add("ie_order_up_to_level", name="Periodic review order-up-to level", topic="inventory / periodic review",
     level="intermediate",
     purpose="Order-up-to level (and order size) of a periodic-review policy that protects against demand over the "
             "review period plus the lead time.",
     inputs=[_MD_IN(), _SD_IN(), _i("review_period", "periods between reviews", "t", "review interval", "review time"),
             _i("lead_time", "lead time in periods", "l", "lead time in days"), _SL_IN(),
             _i("inventory_position", "current inventory position: on hand + on order - backorders (optional)",
                "on hand", "current inventory", "ip", required=False)],
     outputs=[_o("order_up_to_level", "order-up-to level S = d(T+L) + z sigma sqrt(T+L)"),
              _o("safety_stock", "safety stock"), _o("z", "service factor"),
              _o("sd_protection_demand", "standard deviation of demand over T + L"),
              _o("order_quantity", "order size S - inventory position (when given)")],
     primary="order_up_to_level", conditions=["mean_demand, sd_demand, review_period > 0", "lead_time >= 0",
                                              "0.5 < service_level < 1"],
     method="S = d (T + L) + z sigma_d sqrt(T + L); order S - IP", latex=r"S = \bar d\,(T+L) + z\,\sigma_d\sqrt{T+L}",
     run=order_up_to, reference=_ref_order_up_to,
     wrong=lambda mean_demand, sd_demand, review_period, lead_time, service_level, inventory_position=None: {
         "order_up_to_level": mean_demand * (review_period + lead_time) + _zinv(service_level) * sd_demand * math.sqrt(lead_time)},
     rand=lambda rng: _drop_none({"mean_demand": _u(rng, 5, 800, 1), "sd_demand": _u(rng, 1, 150, 1), "review_period": rng.randint(1, 30),
                       "lead_time": rng.randint(1, 20), "service_level": _u(rng, 0.8, 0.995, 3),
                       "inventory_position": rng.choice([None, _u(rng, 0, 3000, 0)])}),
     triggers=["order up to level", "order-up-to level", "periodic review", "fixed period", "fixed-period"],
     ask="order-up-to level",
     examples=[{"mean_demand": 20, "sd_demand": 4, "review_period": 14, "lead_time": 7, "service_level": 0.95,
                "inventory_position": 150},
               {"mean_demand": 100, "sd_demand": 25, "review_period": 7, "lead_time": 3, "service_level": 0.98}],
     invalid=[{"mean_demand": 20, "sd_demand": 4, "review_period": 0, "lead_time": 7, "service_level": 0.95}])


def abc_classification(annual_values, a_share=0.8, b_share=0.95):
    v = _vec("annual_values", annual_values, 2, 1000, nonneg=True)
    a, b = _prob("a_share", a_share), _prob("b_share", b_share)
    _need(a < b, "a_share must be below b_share")
    tot = sum(v)
    _need(tot > 0, "the annual values must not all be 0")
    order = sorted(range(len(v)), key=lambda k: (-v[k], k))
    cls = ["C"] * len(v)
    before = 0.0
    for k in order:
        share = before / tot
        cls[k] = "A" if share < a else ("B" if share < b else "C")
        before += v[k]
    na, nb = cls.count("A"), cls.count("B")
    share_a = sum(v[k] for k in range(len(v)) if cls[k] == "A") / tot
    return _rd({"classes": cls, "count_a": na, "count_b": nb, "count_c": len(v) - na - nb, "share_a": share_a,
                "share_b": sum(v[k] for k in range(len(v)) if cls[k] == "B") / tot})


def _ref_abc(annual_values, a_share=0.8, b_share=0.95):
    import pandas as pd
    s = pd.Series(annual_values, dtype=float)
    ranked = s.reset_index().sort_values([0, "index"], ascending=[False, True])
    before = ranked[0].cumsum().shift(1, fill_value=0.0) / s.sum()
    lab = pd.Series(["C"] * len(s))
    for idx, sh in zip(ranked["index"], before):
        lab[idx] = "A" if sh < a_share else ("B" if sh < b_share else "C")
    return {"classes": list(lab), "count_a": int((lab == "A").sum()), "share_a": float(s[lab == "A"].sum() / s.sum())}


_add("ie_abc_classification", name="ABC inventory classification", topic="inventory / classification", level="beginner",
     purpose="Classify items by annual usage value: A items make up the first ~80 % of value, B the next ~15 %, C the rest.",
     inputs=[_i("annual_values", "annual usage value of each item (annual demand x unit cost)", "values",
                "annual usage values", "annual dollar usage", "usage values", type="vector"),
             _i("a_share", "cumulative value share closing class A (default 80%)", "a cutoff", "a threshold", required=False),
             _i("b_share", "cumulative value share closing class B (default 95%)", "b cutoff", "b threshold", required=False)],
     outputs=[_o("classes", "class of each item, in input order"), _o("count_a", "number of A items"),
              _o("count_b", "number of B items"), _o("count_c", "number of C items"),
              _o("share_a", "share of total value in class A"), _o("share_b", "share of total value in class B")],
     primary="share_a",
     conditions=["values >= 0, not all 0", "0 < a_share < b_share < 1",
                 "items ranked by value (ties by item number); an item belongs to A while the value ranked above it is "
                 "below a_share of the total (so the item crossing the threshold is still A), likewise B"],
     method="rank by value; cumulative share before each item decides its class",
     latex=r"\text{class}(k) = A \text{ if } \frac{\sum_{j \prec k} v_j}{\sum v} < 0.8,\ B \text{ if } < 0.95,\ C",
     run=abc_classification, reference=_ref_abc,
     wrong=lambda annual_values, a_share=0.8, b_share=0.95: {"share_a": sum(sorted(annual_values, reverse=True)[:max(1, len(annual_values) // 5)]) / sum(annual_values) + 0.5},
     rand=lambda rng: {"annual_values": [round(rng.lognormvariate(7, 1.5), 2) for _ in range(rng.randint(5, 25))]},
     triggers=["abc classification", "abc analysis", "abc inventory"], ask="ABC classification",
     examples=[{"annual_values": [90000, 15000, 4000, 60000, 2500, 12000, 1500, 800, 30000, 6000]},
               {"annual_values": [500, 12000, 300, 8000, 200], "a_share": 0.7, "b_share": 0.9}],
     invalid=[{"annual_values": [0, 0, 0]}],
     weak="the reference ranks with pandas and the same cumulative-share rule")


def inventory_turnover(cost_of_goods_sold, average_inventory, days_per_year=365):
    cogs, inv = _pos("cost_of_goods_sold", cost_of_goods_sold), _pos("average_inventory", average_inventory)
    dpy = _pos("days_per_year", days_per_year)
    t = cogs / inv
    return _rd({"turnover": t, "days_of_supply": dpy / t, "weeks_of_supply": 52 * inv / cogs})


def _ref_turnover(cost_of_goods_sold, average_inventory, days_per_year=365):
    from fractions import Fraction
    t = Fraction(repr(float(cost_of_goods_sold))) / Fraction(repr(float(average_inventory)))
    return {"turnover": float(t), "days_of_supply": float(Fraction(repr(float(days_per_year))) / t),
            "weeks_of_supply": float(52 / t)}


_add("ie_inventory_turnover", name="Inventory turnover and days of supply", topic="inventory / performance",
     level="beginner",
     purpose="How many times a year inventory turns over and how many days (weeks) of supply it holds.",
     inputs=[_i("cost_of_goods_sold", "annual cost of goods sold", "cogs", "annual cogs", "cost of sales"),
             _i("average_inventory", "average inventory value (at cost)", "inventory", "average inventory value"),
             _i("days_per_year", "days in the year used (default 365)", "days", "working days", required=False)],
     outputs=[_o("turnover", "inventory turns per year COGS / average inventory"), _o("days_of_supply", "days of supply"),
              _o("weeks_of_supply", "weeks of supply")],
     primary="turnover", conditions=["all inputs > 0"],
     method="turnover = COGS / average inventory; days = days per year / turnover",
     latex=r"\text{turns} = \frac{\text{COGS}}{\bar I},\quad \text{days of supply} = \frac{365}{\text{turns}}",
     run=inventory_turnover, reference=_ref_turnover,
     wrong=lambda cost_of_goods_sold, average_inventory, days_per_year=365: {"turnover": average_inventory / cost_of_goods_sold},
     rand=lambda rng: {"cost_of_goods_sold": _u(rng, 1e4, 1e8, 0), "average_inventory": _u(rng, 1e3, 1e7, 0),
                       "days_per_year": rng.choice([365, 360, 250])},
     triggers=["inventory turnover", "inventory turns", "days of supply", "stock turnover"], ask="inventory turnover",
     examples=[{"cost_of_goods_sold": 1200000, "average_inventory": 150000},
               {"cost_of_goods_sold": 860000, "average_inventory": 215000, "days_per_year": 250}],
     invalid=[{"cost_of_goods_sold": 1200000, "average_inventory": 0}],
     weak="a definition: the reference recomputes it exactly with fractions")


def kanban_count(demand_rate, lead_time, container_size, safety_factor=0.0):
    d, L = _pos("demand_rate", demand_rate), _pos("lead_time", lead_time)
    c, a = _pos("container_size", container_size), _nonneg("safety_factor", safety_factor)
    x = d * L * (1 + a) / c
    return _rd({"kanbans": _ceil(x), "exact": x, "max_inventory": _ceil(x) * c})


def _ref_kanban(demand_rate, lead_time, container_size, safety_factor=0.0):
    from fractions import Fraction
    F = lambda v: Fraction(repr(float(v)))
    x = F(demand_rate) * F(lead_time) * (1 + F(safety_factor)) / F(container_size)
    k = -((-x.numerator) // x.denominator)
    return {"kanbans": k, "exact": float(x), "max_inventory": float(k * F(container_size))}


_add("ie_kanban_count", name="Number of kanbans", topic="inventory / lean", level="beginner",
     purpose="Number of kanban containers needed to cover demand during the replenishment lead time plus a safety factor.",
     inputs=[_i("demand_rate", "demand per time unit (per hour, say)", "demand", "usage rate", "d", "demand per hour"),
             _i("lead_time", "replenishment lead time in the same time unit (waiting + processing)", "l",
                "replenishment time"),
             _i("container_size", "parts per container", "c", "container capacity", "parts per container"),
             _i("safety_factor", "safety factor (decimal; default 0)", "alpha", "safety stock factor", required=False)],
     outputs=[_o("kanbans", "number of kanbans (rounded up)"), _o("exact", "d L (1 + alpha) / C before rounding"),
              _o("max_inventory", "maximum inventory N C")],
     primary="exact", conditions=["demand_rate, lead_time, container_size > 0", "safety_factor >= 0"],
     method="N = ceil(d L (1 + alpha) / C)", latex=r"N = \left\lceil \frac{d\,L\,(1+\alpha)}{C} \right\rceil",
     run=kanban_count, reference=_ref_kanban,
     wrong=lambda demand_rate, lead_time, container_size, safety_factor=0.0: {"exact": demand_rate * lead_time * (1 + safety_factor) / container_size + 1},
     rand=lambda rng: {"demand_rate": _u(rng, 5, 2000, 1), "lead_time": _u(rng, 0.1, 10, 2),
                       "container_size": rng.choice([10, 20, 25, 50, 100, 200]), "safety_factor": rng.choice([0, 0.1, 0.15, 0.2, 0.25])},
     triggers=["kanban", "number of kanbans", "kanban containers"], ask="number of kanbans",
     examples=[{"demand_rate": 100, "lead_time": 2, "container_size": 20, "safety_factor": 0.1},
               {"demand_rate": 350, "lead_time": 0.5, "container_size": 25}],
     invalid=[{"demand_rate": 100, "lead_time": 2, "container_size": 0}],
     weak="the reference recomputes the definition exactly with fractions (rounding up included)")


# =====================================================================================================================
# STATISTICAL PROCESS CONTROL
# =====================================================================================================================
_A2 = {2: 1.880, 3: 1.023, 4: 0.729, 5: 0.577, 6: 0.483, 7: 0.419, 8: 0.373, 9: 0.337, 10: 0.308}
_D3 = {2: 0.0, 3: 0.0, 4: 0.0, 5: 0.0, 6: 0.0, 7: 0.076, 8: 0.136, 9: 0.184, 10: 0.223}
_D4 = {2: 3.267, 3: 2.574, 4: 2.282, 5: 2.114, 6: 2.004, 7: 1.924, 8: 1.864, 9: 1.816, 10: 1.777}
_A3 = {2: 2.659, 3: 1.954, 4: 1.628, 5: 1.427, 6: 1.287, 7: 1.182, 8: 1.099, 9: 1.032, 10: 0.975}
_B3 = {2: 0.0, 3: 0.0, 4: 0.0, 5: 0.0, 6: 0.030, 7: 0.118, 8: 0.185, 9: 0.239, 10: 0.284}
_B4 = {2: 3.267, 3: 2.568, 4: 2.266, 5: 2.089, 6: 1.970, 7: 1.882, 8: 1.815, 9: 1.761, 10: 1.716}
_D2 = {2: 1.128, 3: 1.693, 4: 2.059, 5: 2.326, 6: 2.534, 7: 2.704, 8: 2.847, 9: 2.970, 10: 3.078}
_CONST_CACHE: dict = {}


def _exact_constants(n):
    """d2, d3 (mean and sd of the range of n standard normals, numerical integration) and c4 (gamma functions):
    the build-time check that the 3-decimal table constants are right."""
    if n not in _CONST_CACHE:
        import numpy as np
        from scipy.special import gammaln
        from scipy.stats import norm
        x = np.linspace(-9.0, 9.0, 1801)[:, None]
        r = np.linspace(0.0, 14.0, 1401)[None, :]
        dens = n * (n - 1) * norm.pdf(x) * norm.pdf(x + r) * (norm.cdf(x + r) - norm.cdf(x)) ** (n - 2)
        m1 = float(np.trapezoid(np.trapezoid(r * dens, x[:, 0], axis=0), r[0]))
        m2 = float(np.trapezoid(np.trapezoid(r * r * dens, x[:, 0], axis=0), r[0]))
        c4 = math.sqrt(2 / (n - 1)) * math.exp(gammaln(n / 2) - gammaln((n - 1) / 2))
        _CONST_CACHE[n] = (m1, math.sqrt(m2 - m1 * m1), c4)
    return _CONST_CACHE[n]


def _check_table(n):
    d2, d3, c4 = _exact_constants(n)
    want = {"A2": (_A2[n], 3 / (d2 * math.sqrt(n))), "D3": (_D3[n], max(0.0, 1 - 3 * d3 / d2)),
            "D4": (_D4[n], 1 + 3 * d3 / d2), "d2": (_D2[n], d2), "A3": (_A3[n], 3 / (c4 * math.sqrt(n))),
            "B3": (_B3[n], max(0.0, 1 - 3 * math.sqrt(1 - c4 * c4) / c4)), "B4": (_B4[n], 1 + 3 * math.sqrt(1 - c4 * c4) / c4)}
    for name, (table, exact) in want.items():
        if abs(table - exact) > 0.0011:
            raise AssertionError(f"table constant {name}(n={n}) = {table} but the exact value is {exact}")


_N_IN = lambda: _i("subgroup_size", "observations per subgroup (2 to 10)", "sample size", "n", "subgroup size",
                   "observations per subgroup", type="integer")
_SPC = "statistical process control"


def xbar_r_limits(grand_mean, mean_range, subgroup_size):
    xb, rb = _fin("grand_mean", grand_mean), _pos("mean_range", mean_range)
    n = _int("subgroup_size", subgroup_size, 2, 10)
    return _rd({"xbar_ucl": xb + _A2[n] * rb, "xbar_lcl": xb - _A2[n] * rb, "center": xb, "r_ucl": _D4[n] * rb,
                "r_lcl": _D3[n] * rb, "A2": _A2[n], "D3": _D3[n], "D4": _D4[n]})


def _ref_xbar_r(grand_mean, mean_range, subgroup_size):
    n = int(subgroup_size)
    _check_table(n)
    return {"xbar_ucl": grand_mean + _A2[n] * mean_range, "xbar_lcl": grand_mean - _A2[n] * mean_range,
            "r_ucl": _D4[n] * mean_range, "r_lcl": _D3[n] * mean_range}


_add("ie_xbar_r_limits", name="X-bar and R chart control limits", topic=f"{_SPC} / variables charts",
     level="intermediate",
     purpose="3-sigma control limits of the X-bar and R charts from the grand mean, the average range and the subgroup size.",
     inputs=[_i("grand_mean", "grand mean of the subgroup means (x-double-bar)", "x double bar", "overall mean",
                "process mean", "mean of means"),
             _i("mean_range", "average subgroup range (R-bar)", "r bar", "average range", "rbar"), _N_IN()],
     outputs=[_o("xbar_ucl", "X-bar chart UCL"), _o("xbar_lcl", "X-bar chart LCL"), _o("center", "X-bar chart centre line"),
              _o("r_ucl", "R chart UCL D4 R-bar"), _o("r_lcl", "R chart LCL D3 R-bar"), _o("A2", "A2"),
              _o("D3", "D3"), _o("D4", "D4")],
     primary="xbar_ucl", conditions=["mean_range > 0", "subgroup size 2 to 10", "standard 3-decimal constants"],
     method="UCL/LCL = x-double-bar +/- A2 R-bar; R chart D3 R-bar, D4 R-bar",
     latex=r"UCL_{\bar x} = \bar{\bar x} + A_2\bar R,\quad UCL_R = D_4\bar R,\quad LCL_R = D_3\bar R",
     run=xbar_r_limits, reference=_ref_xbar_r,
     wrong=lambda grand_mean, mean_range, subgroup_size: {
         "xbar_ucl": grand_mean + 3 * mean_range / _D2[int(subgroup_size)], "r_ucl": 3 * mean_range},
     rand=lambda rng: {"grand_mean": _u(rng, 1, 500, 3), "mean_range": _u(rng, 0.05, 20, 3), "subgroup_size": rng.randint(2, 10)},
     triggers=["x bar and r chart", "x-bar and r chart", "xbar r chart", "x bar r chart", "x-bar r chart",
               "xbar and r chart"], ask="X-bar and R chart limits",
     examples=[{"grand_mean": 50, "mean_range": 2, "subgroup_size": 5},
               {"grand_mean": 10.2, "mean_range": 0.35, "subgroup_size": 4}],
     invalid=[{"grand_mean": 50, "mean_range": 2, "subgroup_size": 12}],
     weak="the factors are published table constants (checked against their exact integrals); the limits are their "
          "definition")


def xbar_s_limits(grand_mean, mean_sd, subgroup_size):
    xb, sb = _fin("grand_mean", grand_mean), _pos("mean_sd", mean_sd)
    n = _int("subgroup_size", subgroup_size, 2, 10)
    return _rd({"xbar_ucl": xb + _A3[n] * sb, "xbar_lcl": xb - _A3[n] * sb, "center": xb, "s_ucl": _B4[n] * sb,
                "s_lcl": _B3[n] * sb, "A3": _A3[n], "B3": _B3[n], "B4": _B4[n]})


def _ref_xbar_s(grand_mean, mean_sd, subgroup_size):
    n = int(subgroup_size)
    _check_table(n)
    return {"xbar_ucl": grand_mean + _A3[n] * mean_sd, "xbar_lcl": grand_mean - _A3[n] * mean_sd,
            "s_ucl": _B4[n] * mean_sd, "s_lcl": _B3[n] * mean_sd}


_add("ie_xbar_s_limits", name="X-bar and S chart control limits", topic=f"{_SPC} / variables charts",
     level="intermediate",
     purpose="3-sigma control limits of the X-bar and S charts from the grand mean, the average subgroup standard "
             "deviation and the subgroup size.",
     inputs=[_i("grand_mean", "grand mean of the subgroup means", "x double bar", "overall mean", "process mean"),
             _i("mean_sd", "average subgroup standard deviation (s-bar)", "s bar", "average standard deviation", "sbar"),
             _N_IN()],
     outputs=[_o("xbar_ucl", "X-bar chart UCL"), _o("xbar_lcl", "X-bar chart LCL"), _o("center", "centre line"),
              _o("s_ucl", "S chart UCL B4 s-bar"), _o("s_lcl", "S chart LCL B3 s-bar"), _o("A3", "A3"), _o("B3", "B3"),
              _o("B4", "B4")],
     primary="xbar_ucl", conditions=["mean_sd > 0", "subgroup size 2 to 10", "standard 3-decimal constants"],
     method="x-double-bar +/- A3 s-bar; S chart B3 s-bar, B4 s-bar",
     latex=r"UCL_{\bar x} = \bar{\bar x} + A_3\bar s,\quad UCL_s = B_4\bar s,\quad LCL_s = B_3\bar s",
     run=xbar_s_limits, reference=_ref_xbar_s,
     wrong=lambda grand_mean, mean_sd, subgroup_size: {"xbar_ucl": grand_mean + 3 * mean_sd, "s_ucl": 3 * mean_sd},
     rand=lambda rng: {"grand_mean": _u(rng, 1, 500, 3), "mean_sd": _u(rng, 0.05, 20, 3), "subgroup_size": rng.randint(2, 10)},
     triggers=["x bar and s chart", "x-bar and s chart", "xbar s chart", "x bar s chart", "x-bar s chart",
               "xbar and s chart"], ask="X-bar and S chart limits",
     examples=[{"grand_mean": 50, "mean_sd": 0.9, "subgroup_size": 5},
               {"grand_mean": 74.0, "mean_sd": 0.0094, "subgroup_size": 5}],
     invalid=[{"grand_mean": 50, "mean_sd": -1, "subgroup_size": 5}],
     weak="the factors are published table constants (checked against their exact integrals); the limits are their "
          "definition")


def sigma_from_range(mean_range, subgroup_size):
    rb, n = _pos("mean_range", mean_range), _int("subgroup_size", subgroup_size, 2, 10)
    s = rb / _D2[n]
    return _rd({"sigma": s, "d2": _D2[n], "natural_tolerance": 6 * s})


def _ref_sigma_range(mean_range, subgroup_size):
    n = int(subgroup_size)
    _check_table(n)
    return {"sigma": mean_range / _D2[n], "natural_tolerance": 6 * mean_range / _D2[n]}


_add("ie_sigma_from_range", name="Process standard deviation from the average range", topic=f"{_SPC} / variables charts",
     level="intermediate",
     purpose="Estimate of the process standard deviation sigma = R-bar / d2 and the natural tolerance 6 sigma.",
     inputs=[_i("mean_range", "average subgroup range (R-bar)", "r bar", "average range", "rbar"), _N_IN()],
     outputs=[_o("sigma", "estimated process standard deviation R-bar/d2"), _o("d2", "d2 constant"),
              _o("natural_tolerance", "natural tolerance 6 sigma")],
     primary="sigma", conditions=["mean_range > 0", "subgroup size 2 to 10"],
     method="sigma = R-bar / d2", latex=r"\hat\sigma = \frac{\bar R}{d_2}", run=sigma_from_range, reference=_ref_sigma_range,
     wrong=lambda mean_range, subgroup_size: {"sigma": mean_range / math.sqrt(int(subgroup_size))},
     rand=lambda rng: {"mean_range": _u(rng, 0.05, 20, 3), "subgroup_size": rng.randint(2, 10)},
     triggers=["sigma from the average range", "standard deviation from average range", "sigma from range",
               "process sigma from r bar", "estimate sigma from range"], ask="process sigma from the average range",
     examples=[{"mean_range": 2, "subgroup_size": 5}, {"mean_range": 0.35, "subgroup_size": 4}],
     invalid=[{"mean_range": 2, "subgroup_size": 1}],
     weak="d2 is a published table constant (checked against its exact integral); the estimate is its definition")


def p_chart_limits(p_bar, sample_size):
    p, n = _prob("p_bar", p_bar), _int("sample_size", sample_size, 1)
    s = math.sqrt(p * (1 - p) / n)
    return _rd({"ucl": min(1.0, p + 3 * s), "lcl": max(0.0, p - 3 * s), "center": p, "sigma_p": s})


def _ref_p_chart(p_bar, sample_size):
    from scipy.stats import binom
    s = float(binom(int(sample_size), p_bar).std()) / sample_size
    return {"ucl": min(1.0, p_bar + 3 * s), "lcl": max(0.0, p_bar - 3 * s), "sigma_p": s}


_add("ie_p_chart_limits", name="p chart control limits", topic=f"{_SPC} / attribute charts", level="beginner",
     purpose="3-sigma limits of the fraction-defective (p) chart: p-bar +/- 3 sqrt(p-bar (1 - p-bar) / n).",
     inputs=[_i("p_bar", "average fraction defective (decimal)", "average fraction defective", "pbar",
                "fraction defective", "proportion defective", "average proportion defective"),
             _i("sample_size", "items per sample", "n", "sample size", "items per sample", type="integer")],
     outputs=[_o("ucl", "upper control limit"), _o("lcl", "lower control limit (not below 0)"), _o("center", "centre line"),
              _o("sigma_p", "standard deviation of the fraction defective")],
     primary="ucl", conditions=["0 < p_bar < 1", "sample_size > 0 (whole number of items)"],
     method="sigma_p = sqrt(p (1 - p)/n); limits p +/- 3 sigma_p, clipped to [0, 1]",
     latex=r"UCL, LCL = \bar p \pm 3\sqrt{\frac{\bar p(1-\bar p)}{n}}", run=p_chart_limits, reference=_ref_p_chart,
     wrong=lambda p_bar, sample_size: {"ucl": min(1.0, p_bar + 3 * math.sqrt(p_bar * (1 - p_bar)))},
     rand=lambda rng: {"p_bar": _u(rng, 0.005, 0.3, 4), "sample_size": rng.randint(20, 500)},
     triggers=["p chart", "p-chart", "fraction defective chart", "proportion defective chart"], ask="p chart limits",
     examples=[{"p_bar": 0.04, "sample_size": 100}, {"p_bar": 0.15, "sample_size": 50}],
     invalid=[{"p_bar": 1.5, "sample_size": 100}])


def np_chart_limits(p_bar, sample_size):
    p, n = _prob("p_bar", p_bar), _int("sample_size", sample_size, 1)
    s = math.sqrt(n * p * (1 - p))
    return _rd({"ucl": min(n, n * p + 3 * s), "lcl": max(0.0, n * p - 3 * s), "center": n * p, "sigma_np": s})


def _ref_np_chart(p_bar, sample_size):
    from scipy.stats import binom
    d = binom(int(sample_size), p_bar)
    m, s = float(d.mean()), float(d.std())
    return {"ucl": min(sample_size, m + 3 * s), "lcl": max(0.0, m - 3 * s), "center": m, "sigma_np": s}


_add("ie_np_chart_limits", name="np chart control limits", topic=f"{_SPC} / attribute charts", level="beginner",
     purpose="3-sigma limits of the number-defective (np) chart: n p +/- 3 sqrt(n p (1 - p)).",
     inputs=[_i("p_bar", "average fraction defective (decimal)", "average fraction defective", "pbar", "fraction defective",
                "proportion defective"),
             _i("sample_size", "items per sample", "n", "sample size", "items per sample", type="integer")],
     outputs=[_o("ucl", "upper control limit"), _o("lcl", "lower control limit (not below 0)"),
              _o("center", "centre line n p"), _o("sigma_np", "standard deviation of the number defective")],
     primary="ucl", conditions=["0 < p_bar < 1", "constant sample size"],
     method="limits n p +/- 3 sqrt(n p (1 - p))", latex=r"n\bar p \pm 3\sqrt{n\bar p(1-\bar p)}",
     run=np_chart_limits, reference=_ref_np_chart,
     wrong=lambda p_bar, sample_size: {"ucl": sample_size * p_bar + 3 * math.sqrt(sample_size * p_bar)},
     rand=lambda rng: {"p_bar": _u(rng, 0.02, 0.4, 4), "sample_size": rng.randint(20, 500)},
     triggers=["np chart", "np-chart", "number defective chart"], ask="np chart limits",
     examples=[{"p_bar": 0.04, "sample_size": 100}, {"p_bar": 0.1, "sample_size": 200}],
     invalid=[{"p_bar": 0.04, "sample_size": 0}])


def c_chart_limits(c_bar):
    c = _pos("c_bar", c_bar)
    s = math.sqrt(c)
    return _rd({"ucl": c + 3 * s, "lcl": max(0.0, c - 3 * s), "center": c, "sigma_c": s})


def _ref_c_chart(c_bar):
    from scipy.stats import poisson
    s = float(poisson(c_bar).std())
    return {"ucl": c_bar + 3 * s, "lcl": max(0.0, c_bar - 3 * s), "sigma_c": s}


_add("ie_c_chart_limits", name="c chart control limits", topic=f"{_SPC} / attribute charts", level="beginner",
     purpose="3-sigma limits of the count-of-defects (c) chart: c-bar +/- 3 sqrt(c-bar).",
     inputs=[_i("c_bar", "average number of defects per unit (inspection unit of constant size)",
                "average number of defects", "cbar", "mean defects", "average defects", "defects per unit")],
     outputs=[_o("ucl", "upper control limit"), _o("lcl", "lower control limit (not below 0)"), _o("center", "centre line"),
              _o("sigma_c", "sqrt(c-bar)")],
     primary="ucl", conditions=["c_bar > 0", "defects Poisson, constant inspection unit"],
     method="limits c +/- 3 sqrt(c)", latex=r"\bar c \pm 3\sqrt{\bar c}", run=c_chart_limits, reference=_ref_c_chart,
     wrong=lambda c_bar: {"ucl": c_bar + 3 * c_bar},
     rand=lambda rng: {"c_bar": _u(rng, 0.5, 60, 3)},
     triggers=["c chart", "c-chart", "count of defects chart"], ask="c chart limits",
     examples=[{"c_bar": 9}, {"c_bar": 2.5}],
     invalid=[{"c_bar": -3}])


def u_chart_limits(u_bar, units):
    u, n = _pos("u_bar", u_bar), _pos("units", units)
    s = math.sqrt(u / n)
    return _rd({"ucl": u + 3 * s, "lcl": max(0.0, u - 3 * s), "center": u, "sigma_u": s})


def _ref_u_chart(u_bar, units):
    from scipy.stats import poisson
    s = float(poisson(u_bar * units).std()) / units
    return {"ucl": u_bar + 3 * s, "lcl": max(0.0, u_bar - 3 * s), "sigma_u": s}


_add("ie_u_chart_limits", name="u chart control limits", topic=f"{_SPC} / attribute charts", level="intermediate",
     purpose="3-sigma limits of the defects-per-unit (u) chart for samples of n units: u-bar +/- 3 sqrt(u-bar / n).",
     inputs=[_i("u_bar", "average defects per unit", "average defects per unit", "ubar", "mean defects per unit"),
             _i("units", "inspection units in the sample", "n", "sample size", "units per sample", "number of units")],
     outputs=[_o("ucl", "upper control limit"), _o("lcl", "lower control limit (not below 0)"), _o("center", "centre line"),
              _o("sigma_u", "sqrt(u-bar / n)")],
     primary="ucl", conditions=["u_bar > 0", "units > 0"],
     method="limits u +/- 3 sqrt(u/n)", latex=r"\bar u \pm 3\sqrt{\bar u / n}", run=u_chart_limits, reference=_ref_u_chart,
     wrong=lambda u_bar, units: {"ucl": u_bar + 3 * math.sqrt(u_bar)},
     rand=lambda rng: {"u_bar": _u(rng, 0.05, 20, 3), "units": rng.randint(2, 200)},
     triggers=["u chart", "u-chart", "defects per unit chart"], ask="u chart limits",
     examples=[{"u_bar": 1.5, "units": 10}, {"u_bar": 0.32, "units": 25}],
     invalid=[{"u_bar": 1.5, "units": 0}])


def p_chart_counts(defectives, sample_size):
    d = _vec("defectives", defectives, 2, 500, nonneg=True)
    n = _int("sample_size", sample_size, 1)
    for x in d:
        _need(x == int(x) and x <= n, "each count of defectives must be a whole number no larger than the sample size")
    p = sum(d) / (n * len(d))
    _need(0 < p < 1, "the average fraction defective must lie strictly between 0 and 1")
    s = math.sqrt(p * (1 - p) / n)
    ucl, lcl = min(1.0, p + 3 * s), max(0.0, p - 3 * s)
    out = []
    for k in range(len(d)):
        f = d[k] / n
        if f > ucl or f < lcl:
            out.append(k + 1)
    return _rd({"p_bar": p, "ucl": ucl, "lcl": lcl, "out_of_control": out, "count_out": len(out)})


def _ref_p_counts(defectives, sample_size):
    import numpy as np
    from scipy.stats import binom
    d = np.asarray(defectives, float)
    p = float(d.sum() / (sample_size * len(d)))
    s = float(binom(int(sample_size), p).std()) / sample_size
    ucl, lcl = min(1.0, p + 3 * s), max(0.0, p - 3 * s)
    f = d / sample_size
    out = [int(k) + 1 for k in np.nonzero((f > ucl) | (f < lcl))[0]]
    return {"p_bar": p, "ucl": ucl, "lcl": lcl, "out_of_control": out, "count_out": len(out)}


def _rand_p_counts(rng):
    n, p = rng.randint(30, 300), rng.uniform(0.02, 0.2)
    m = rng.randint(8, 30)
    d = [sum(1 for _ in range(n) if rng.random() < p) for _ in range(m)]
    if rng.random() < 0.5:
        d[rng.randrange(m)] = min(n, int(n * p * 3) + 3)
    return {"defectives": d, "sample_size": n}


_add("ie_p_chart_from_samples", name="p chart from sample counts", topic=f"{_SPC} / attribute charts",
     level="intermediate",
     purpose="Centre line and 3-sigma limits of a p chart computed from the defectives found in each sample, and the "
             "samples that fall outside them.",
     inputs=[_i("defectives", "number of defective items in each sample", "defective counts", "defectives per sample",
                "number defective", type="vector"),
             _i("sample_size", "items in every sample", "n", "sample size", "items per sample", type="integer")],
     outputs=[_o("p_bar", "average fraction defective"), _o("ucl", "upper control limit"), _o("lcl", "lower control limit"),
              _o("out_of_control", "sample numbers outside the limits"), _o("count_out", "how many samples are outside")],
     primary="p_bar", conditions=["counts are whole numbers <= sample size", "0 < p-bar < 1", "constant sample size"],
     method="p-bar = total defectives / total inspected; limits p-bar +/- 3 sqrt(p-bar (1 - p-bar)/n); points beyond "
            "the limits flagged",
     latex=r"\bar p = \frac{\sum d_i}{m\,n},\quad \bar p \pm 3\sqrt{\bar p(1-\bar p)/n}",
     run=p_chart_counts, reference=_ref_p_counts,
     wrong=lambda defectives, sample_size: {"p_bar": sum(defectives) / sample_size},
     rand=_rand_p_counts,
     triggers=["p chart from samples", "p chart from sample counts", "p chart from counts", "p chart for the samples"],
     ask="p chart from samples",
     examples=[{"defectives": [3, 5, 2, 4, 12, 3, 4, 2, 5, 3], "sample_size": 100},
               {"defectives": [8, 6, 9, 7, 5, 10, 6, 8], "sample_size": 150}],
     invalid=[{"defectives": [3, 5, 2, 400], "sample_size": 100}])


_USL = lambda: _i("usl", "upper specification limit", "upper spec", "upper specification limit", "upper limit")
_LSL = lambda: _i("lsl", "lower specification limit", "lower spec", "lower specification limit", "lower limit")
_MU = lambda: _i("mean", "process mean", "process mean", "mu", "average")
_SIG = lambda: _i("sd", "process standard deviation", "standard deviation", "sigma", "process standard deviation", "std")


def capability(usl, lsl, mean, sd):
    u, l, m, s = _fin("usl", usl), _fin("lsl", lsl), _fin("mean", mean), _pos("sd", sd)
    _need(u > l, "the upper specification limit must exceed the lower one")
    cpu, cpl = (u - m) / (3 * s), (m - l) / (3 * s)
    return _rd({"cpk": min(cpu, cpl), "cp": (u - l) / (6 * s), "cpu": cpu, "cpl": cpl})


def _ref_capability(usl, lsl, mean, sd):
    from scipy.stats import norm
    zu = float(norm.isf(norm.sf(usl, mean, sd)))          # distances to the limits read back from the tail areas
    zl = float(norm.isf(norm.cdf(lsl, mean, sd)))
    return {"cpk": min(zu, zl) / 3, "cpu": zu / 3, "cpl": zl / 3, "cp": (zu + zl) / 6}


def _rand_cap(rng):
    m, s = _u(rng, 10, 100, 3), _u(rng, 0.1, 3, 3)
    return {"usl": round(m + s * rng.uniform(1.5, 5), 3), "lsl": round(m - s * rng.uniform(1.5, 5), 3), "mean": m, "sd": s}


_add("ie_process_capability", name="Process capability Cp and Cpk", topic=f"{_SPC} / capability", level="beginner",
     purpose="Capability indices of a process with known mean and standard deviation against two specification limits.",
     inputs=[_USL(), _LSL(), _MU(), _SIG()],
     outputs=[_o("cpk", "Cpk = min(Cpu, Cpl)"), _o("cp", "Cp = (USL - LSL)/(6 sigma)"), _o("cpu", "(USL - mean)/(3 sigma)"),
              _o("cpl", "(mean - LSL)/(3 sigma)")],
     primary="cpk", conditions=["usl > lsl", "sd > 0", "process normal and in control"],
     method="Cp = (USL - LSL)/6 sigma, Cpk = min(USL - mu, mu - LSL)/3 sigma",
     latex=r"C_p = \frac{USL-LSL}{6\sigma},\quad C_{pk} = \min\!\left(\frac{USL-\mu}{3\sigma},\frac{\mu-LSL}{3\sigma}\right)",
     run=capability, reference=_ref_capability,
     wrong=lambda usl, lsl, mean, sd: {"cpk": max((usl - mean) / (3 * sd), (mean - lsl) / (3 * sd)), "cp": (usl - lsl) / (3 * sd)},
     rand=_rand_cap,
     triggers=["process capability", "cpk", "capability index", "capability indices"], ask="process capability",
     examples=[{"usl": 10.5, "lsl": 9.5, "mean": 10.1, "sd": 0.12}, {"usl": 230, "lsl": 170, "mean": 205, "sd": 8}],
     invalid=[{"usl": 9.5, "lsl": 10.5, "mean": 10.1, "sd": 0.12}])


def performance(data, usl, lsl):
    x = _vec("data", data, 2, 10000)
    u, l = _fin("usl", usl), _fin("lsl", lsl)
    _need(u > l, "the upper specification limit must exceed the lower one")
    m = sum(x) / len(x)
    s = math.sqrt(sum((v - m) ** 2 for v in x) / (len(x) - 1))
    _need(s > 0, "the data must vary")
    ppu, ppl = (u - m) / (3 * s), (m - l) / (3 * s)
    return _rd({"ppk": min(ppu, ppl), "pp": (u - l) / (6 * s), "mean": m, "sd": s})


def _ref_performance(data, usl, lsl):
    import numpy as np
    x = np.asarray(data, float)
    m, s = float(x.mean()), float(x.std(ddof=1))
    return {"ppk": min(usl - m, m - lsl) / (3 * s), "pp": (usl - lsl) / (6 * s), "mean": m, "sd": s}


def _rand_perf(rng):
    m, s = _u(rng, 10, 100, 2), _u(rng, 0.2, 3, 2)
    return {"data": [round(rng.gauss(m, s), 2) for _ in range(rng.randint(10, 40))],
            "usl": round(m + s * rng.uniform(2, 5), 2), "lsl": round(m - s * rng.uniform(2, 5), 2)}


_add("ie_process_performance", name="Process performance Pp and Ppk from data", topic=f"{_SPC} / capability",
     level="intermediate",
     purpose="Performance indices Pp and Ppk from measured data, using the overall sample standard deviation (n - 1).",
     inputs=[_i("data", "measurements", "measurements", "observations", "sample", "values", type="vector"), _USL(), _LSL()],
     outputs=[_o("ppk", "Ppk"), _o("pp", "Pp"), _o("mean", "sample mean"), _o("sd", "overall sample standard deviation")],
     primary="ppk", conditions=["at least 2 measurements that vary", "usl > lsl"],
     method="s = sqrt(sum (x - mean)^2/(n - 1)); Pp = (USL - LSL)/6s, Ppk = min(USL - mean, mean - LSL)/3s",
     latex=r"P_p = \frac{USL-LSL}{6s},\quad P_{pk} = \min\!\left(\frac{USL-\bar x}{3s},\frac{\bar x-LSL}{3s}\right)",
     run=performance, reference=_ref_performance,
     wrong=lambda data, usl, lsl: {"ppk": min(usl - sum(data) / len(data), sum(data) / len(data) - lsl)
                                   / (3 * math.sqrt(sum((v - sum(data) / len(data)) ** 2 for v in data) / len(data)))},
     rand=_rand_perf,
     triggers=["process performance", "ppk", "pp and ppk", "performance index", "performance indices"],
     ask="process performance",
     examples=[{"data": [10.1, 9.9, 10.2, 10.0, 10.3, 9.8, 10.1, 10.0], "usl": 10.6, "lsl": 9.4},
               {"data": [52, 48, 50, 51, 49, 53, 47, 50, 50, 52], "usl": 58, "lsl": 44}],
     invalid=[{"data": [10, 10, 10], "usl": 10.6, "lsl": 9.4}])


def cpm_index(usl, lsl, mean, sd, target):
    u, l, m, s, t = _fin("usl", usl), _fin("lsl", lsl), _fin("mean", mean), _pos("sd", sd), _fin("target", target)
    _need(u > l, "the upper specification limit must exceed the lower one")
    _need(l < t < u, "the target must lie between the specification limits")
    tau = math.sqrt(s * s + (m - t) ** 2)
    return _rd({"cpm": (u - l) / (6 * tau), "cp": (u - l) / (6 * s), "tau": tau})


def _ref_cpm(usl, lsl, mean, sd, target):
    from scipy.stats import norm
    f = norm(mean, sd).pdf
    msd = _quad(lambda x: (x - target) ** 2 * f(x), mean - 40 * sd, mean + 40 * sd)
    return {"cpm": (usl - lsl) / (6 * math.sqrt(msd)), "tau": math.sqrt(msd)}


_add("ie_cpm_taguchi", name="Taguchi capability index Cpm", topic=f"{_SPC} / capability", level="advanced",
     purpose="Capability index that penalises distance from the target: Cpm = (USL - LSL) / (6 sqrt(sigma^2 + (mu - T)^2)).",
     inputs=[_USL(), _LSL(), _MU(), _SIG(), _i("target", "target value T", "t", "nominal", "target value")],
     outputs=[_o("cpm", "Cpm"), _o("cp", "Cp"), _o("tau", "root mean square deviation from target")],
     primary="cpm", conditions=["usl > lsl", "lsl < target < usl", "sd > 0"],
     method="tau = sqrt(sigma^2 + (mu - T)^2), Cpm = (USL - LSL)/(6 tau)",
     latex=r"C_{pm} = \frac{USL-LSL}{6\sqrt{\sigma^2+(\mu-T)^2}}", run=cpm_index, reference=_ref_cpm,
     wrong=lambda usl, lsl, mean, sd, target: {"cpm": (usl - lsl) / (6 * (sd + abs(mean - target)))},
     rand=lambda rng: (lambda d: dict(d, mean=round(d["target"] + d["sd"] * rng.uniform(-1.5, 1.5), 3)))(
         (lambda t, s: {"target": t, "sd": s, "usl": round(t + s * rng.uniform(3, 6), 3), "lsl": round(t - s * rng.uniform(3, 6), 3)})(
             _u(rng, 10, 100, 3), _u(rng, 0.1, 3, 3))),
     triggers=["cpm", "taguchi capability", "taguchi index"], ask="Taguchi capability index Cpm",
     examples=[{"usl": 10.6, "lsl": 9.4, "mean": 10.1, "sd": 0.1, "target": 10.0},
               {"usl": 60, "lsl": 40, "mean": 52, "sd": 2.5, "target": 50}],
     invalid=[{"usl": 10.6, "lsl": 9.4, "mean": 10.1, "sd": 0.1, "target": 11}])


def fraction_nonconforming(mean, sd, lsl=None, usl=None):
    m, s = _fin("mean", mean), _pos("sd", sd)
    _need(lsl is not None or usl is not None, "give at least one specification limit")
    below = _cdf((_fin("lsl", lsl) - m) / s) if lsl is not None else 0.0
    above = _sf((_fin("usl", usl) - m) / s) if usl is not None else 0.0
    if lsl is not None and usl is not None:
        _need(usl > lsl, "the upper specification limit must exceed the lower one")
    tot = below + above
    return _rd({"fraction_out": tot, "below_lsl": below, "above_usl": above, "ppm": tot * 1e6, "yield": 1 - tot})


def _ref_fraction(mean, sd, lsl=None, usl=None):
    from scipy.stats import norm
    b = float(norm.cdf(lsl, mean, sd)) if lsl is not None else 0.0
    a = float(norm.sf(usl, mean, sd)) if usl is not None else 0.0
    return {"fraction_out": a + b, "below_lsl": b, "above_usl": a, "ppm": (a + b) * 1e6}


def _rand_frac(rng):
    m, s = _u(rng, 10, 100, 3), _u(rng, 0.1, 3, 3)
    d = {"mean": m, "sd": s, "lsl": round(m - s * rng.uniform(1, 5), 3), "usl": round(m + s * rng.uniform(1, 5), 3)}
    k = rng.random()
    if k < 0.2:
        del d["lsl"]
    elif k < 0.4:
        del d["usl"]
    return d


_add("ie_fraction_nonconforming", name="Fraction nonconforming (normal process)", topic=f"{_SPC} / capability",
     level="intermediate",
     purpose="Fraction (and parts per million) of output outside the specification limits for a normal process.",
     inputs=[_MU(), _SIG(), _i("lsl", "lower specification limit (optional)", "lower spec", "lower specification limit",
                               required=False),
             _i("usl", "upper specification limit (optional)", "upper spec", "upper specification limit", required=False)],
     outputs=[_o("fraction_out", "fraction outside the limits"), _o("below_lsl", "fraction below LSL"),
              _o("above_usl", "fraction above USL"), _o("ppm", "parts per million out of specification"),
              _o("yield", "fraction within specification")],
     primary="fraction_out", conditions=["sd > 0", "at least one limit", "usl > lsl", "process normal"],
     method="P(X < LSL) + P(X > USL) for X ~ N(mu, sigma^2)",
     latex=r"p = \Phi\!\left(\frac{LSL-\mu}{\sigma}\right) + 1 - \Phi\!\left(\frac{USL-\mu}{\sigma}\right)",
     run=fraction_nonconforming, reference=_ref_fraction,
     wrong=lambda mean, sd, lsl=None, usl=None: {"fraction_out": (_cdf((lsl - mean) / sd) if lsl is not None else 0.0) * 2 + 1e-3},
     rand=_rand_frac,
     triggers=["fraction nonconforming", "fraction non-conforming", "fraction out of specification",
               "percent defective outside specification", "parts per million defective", "ppm defective"],
     ask="fraction nonconforming",
     examples=[{"mean": 10.1, "sd": 0.12, "lsl": 9.7, "usl": 10.5}, {"mean": 205, "sd": 8, "usl": 230}],
     invalid=[{"mean": 205, "sd": 0, "usl": 230}])


def arl_shewhart(shift, subgroup_size=1, limit_sigma=3.0):
    d = _nonneg("shift", shift)
    n = _int("subgroup_size", subgroup_size, 1, 1000)
    L = _pos("limit_sigma", limit_sigma)
    _need(L <= 6, "limit_sigma must be at most 6")
    a = d * math.sqrt(n)
    beta = _cdf(L - a) - _cdf(-L - a)
    alpha = 2 * _sf(L)
    return _rd({"arl": 1 / (1 - beta), "beta": beta, "detection_probability": 1 - beta, "arl_in_control": 1 / alpha})


def _ref_arl(shift, subgroup_size=1, limit_sigma=3.0):
    from scipy.stats import norm
    a = shift * math.sqrt(subgroup_size)
    p = float(norm.sf(limit_sigma, loc=a) + norm.cdf(-limit_sigma, loc=a))
    return {"arl": 1 / p, "detection_probability": p, "arl_in_control": 1 / float(2 * norm.sf(limit_sigma))}


_add("ie_arl_shewhart", name="Average run length of a Shewhart X-bar chart", topic=f"{_SPC} / chart performance",
     level="advanced",
     purpose="Probability that a Shewhart chart signals on a sample after the mean shifts by k sigma, and the average "
             "run length (ARL) until it does.",
     inputs=[_i("shift", "mean shift in units of the process standard deviation (k)", "k", "mean shift",
                "shift in sigma", "shift size"),
             _i("subgroup_size", "observations per subgroup (default 1)", "n", "sample size", "subgroup size",
                type="integer", required=False),
             _i("limit_sigma", "control limits at +/- L sigma of the mean (default 3)", "l", "limit width",
                "sigma limits", required=False)],
     outputs=[_o("arl", "average run length after the shift 1/(1 - beta)"), _o("beta", "probability of no signal per sample"),
              _o("detection_probability", "probability of a signal per sample"),
              _o("arl_in_control", "in-control ARL 1/alpha")],
     primary="arl", conditions=["shift >= 0", "1 <= subgroup_size <= 1000", "0 < limit_sigma <= 6"],
     method="beta = Phi(L - k sqrt(n)) - Phi(-L - k sqrt(n)); ARL = 1/(1 - beta)",
     latex=r"\beta = \Phi(L - k\sqrt n) - \Phi(-L - k\sqrt n),\quad ARL_1 = \frac{1}{1-\beta}",
     run=arl_shewhart, reference=_ref_arl,
     wrong=lambda shift, subgroup_size=1, limit_sigma=3.0: {
         "arl": 1 / (1 - (_cdf(limit_sigma - shift) - _cdf(-limit_sigma - shift))) * (1.5 if int(subgroup_size) == 1 else 1)},
     rand=lambda rng: {"shift": _u(rng, 0.25, 3, 2), "subgroup_size": rng.randint(1, 10), "limit_sigma": rng.choice([3, 3, 2.5, 3.5])},
     triggers=["average run length", "arl"], ask="average run length",
     examples=[{"shift": 1, "subgroup_size": 5}, {"shift": 1.5, "subgroup_size": 4, "limit_sigma": 3}],
     invalid=[{"shift": -1, "subgroup_size": 5}])


def ewma_limits(target, sigma, smoothing, limit_sigma, sample_number):
    m, s = _fin("target", target), _pos("sigma", sigma)
    lam, L = _prob("smoothing", smoothing, 0.0, 1.0 + 1e-12), _pos("limit_sigma", limit_sigma)
    i = _int("sample_number", sample_number, 1, 100000)
    f = lam / (2 - lam)
    w = L * s * math.sqrt(f * (1 - (1 - lam) ** (2 * i)))
    wi = L * s * math.sqrt(f)
    return _rd({"ucl": m + w, "lcl": m - w, "ucl_steady": m + wi, "lcl_steady": m - wi, "sd_ewma": w / L})


def _ref_ewma(target, sigma, smoothing, limit_sigma, sample_number):
    import numpy as np
    j = np.arange(int(sample_number))
    var = float(np.sum((smoothing * (1 - smoothing) ** j) ** 2)) * sigma ** 2   # z_i = sum lam (1-lam)^j x_(i-j)
    w = limit_sigma * math.sqrt(var)
    jj = np.arange(4000)
    vinf = float(np.sum((smoothing * (1 - smoothing) ** jj) ** 2)) * sigma ** 2
    return {"ucl": target + w, "lcl": target - w, "ucl_steady": target + limit_sigma * math.sqrt(vinf)}


_add("ie_ewma_limits", name="EWMA chart control limits", topic=f"{_SPC} / advanced charts", level="expert",
     purpose="Control limits of an exponentially weighted moving average chart at sample i and in the steady state.",
     inputs=[_i("target", "in-control mean mu0", "mu0", "centre line", "target mean"),
             _i("sigma", "standard deviation of the plotted observations", "standard deviation", "sd"),
             _i("smoothing", "smoothing constant lambda (0 < lambda <= 1)", "lambda", "smoothing constant", "weight"),
             _i("limit_sigma", "limit width L (often 2.7 to 3)", "l", "limit width", "sigma limits"),
             _i("sample_number", "sample number i", "i", "sample", "sample index", type="integer")],
     outputs=[_o("ucl", "UCL at sample i"), _o("lcl", "LCL at sample i"), _o("ucl_steady", "steady-state UCL"),
              _o("lcl_steady", "steady-state LCL"), _o("sd_ewma", "standard deviation of the EWMA at sample i")],
     primary="ucl", conditions=["sigma > 0", "0 < lambda <= 1", "L > 0", "sample_number >= 1"],
     method="limits mu0 +/- L sigma sqrt(lambda/(2 - lambda) (1 - (1 - lambda)^(2i)))",
     latex=r"\mu_0 \pm L\sigma\sqrt{\frac{\lambda}{2-\lambda}\left[1-(1-\lambda)^{2i}\right]}",
     run=ewma_limits, reference=_ref_ewma,
     wrong=lambda target, sigma, smoothing, limit_sigma, sample_number: {
         "ucl": target + limit_sigma * sigma * math.sqrt(smoothing / (2 - smoothing)) * 1.0001 + limit_sigma * sigma * (1 - smoothing) ** int(sample_number)},
     rand=lambda rng: {"target": _u(rng, 0, 100, 2), "sigma": _u(rng, 0.1, 5, 2), "smoothing": rng.choice([0.05, 0.1, 0.2, 0.25, 0.4]),
                       "limit_sigma": rng.choice([2.7, 2.86, 3]), "sample_number": rng.randint(1, 30)},
     triggers=["ewma chart", "ewma control chart", "ewma limits", "exponentially weighted moving average chart"],
     ask="EWMA chart limits",
     examples=[{"target": 10, "sigma": 1, "smoothing": 0.1, "limit_sigma": 2.7, "sample_number": 1},
               {"target": 50, "sigma": 2, "smoothing": 0.2, "limit_sigma": 3, "sample_number": 5}],
     invalid=[{"target": 10, "sigma": 1, "smoothing": 1.5, "limit_sigma": 2.7, "sample_number": 1}])


def tabular_cusum(data, target, k, h):
    x = _vec("data", data, 1, 10000)
    m, kk, hh = _fin("target", target), _nonneg("k", k), _pos("h", h)
    cp, cm, sig = 0.0, 0.0, 0
    ups, downs = [], []
    for i in range(len(x)):
        cp = max(0.0, x[i] - (m + kk) + cp)
        cm = max(0.0, (m - kk) - x[i] + cm)
        ups.append(cp)
        downs.append(cm)
        if sig == 0 and (cp > hh or cm > hh):
            sig = i + 1
    return _rd({"first_signal": sig, "c_plus": ups, "c_minus": downs, "final_c_plus": cp, "final_c_minus": cm})


def _ref_cusum(data, target, k, h):
    from fractions import Fraction
    import pandas as pd
    F = lambda v: Fraction(repr(float(v)))
    s = pd.Series([F(v) - F(target) - F(k) for v in data]).cumsum()
    t = pd.Series([F(target) - F(k) - F(v) for v in data]).cumsum()
    run_min = lambda c: [c[i] - min([Fraction(0)] + list(c[:i + 1])) for i in range(len(c))]   # C_i = S_i - min_j<=i S_j
    up, dn = run_min(list(s)), run_min(list(t))
    sig = next((i + 1 for i in range(len(up)) if up[i] > F(h) or dn[i] > F(h)), 0)
    return {"first_signal": sig, "c_plus": [float(v) for v in up], "c_minus": [float(v) for v in dn]}


def _rand_cusum(rng):
    m, s = _u(rng, 5, 50, 1), _u(rng, 0.5, 3, 2)
    n = rng.randint(8, 30)
    sh = rng.randint(3, n)
    x = [round(rng.gauss(m + (s * rng.uniform(0.5, 2) if i >= sh else 0), s), 3) for i in range(n)]
    return {"data": x, "target": m, "k": round(0.5 * s, 4), "h": round(rng.uniform(4, 5) * s, 4)}


_add("ie_tabular_cusum", name="Tabular CUSUM chart", topic=f"{_SPC} / advanced charts", level="expert",
     purpose="Upper and lower cumulative sums C+ and C- of a tabular CUSUM chart and the first sample that signals "
             "(C+ or C- above h).",
     inputs=[_i("data", "observations in time order", "observations", "measurements", "values", type="vector"),
             _i("target", "target mean mu0", "mu0", "target mean"),
             _i("k", "reference value K (often half the shift to detect)", "reference value", "allowance", "slack value"),
             _i("h", "decision interval H", "decision interval", "decision limit", "threshold")],
     outputs=[_o("first_signal", "first sample number with C+ or C- above H (0 = none)"),
              _o("c_plus", "C+ after each sample"), _o("c_minus", "C- after each sample"),
              _o("final_c_plus", "last C+"), _o("final_c_minus", "last C-")],
     primary="final_c_plus", conditions=["k >= 0", "h > 0", "C+ and C- start at 0"],
     method="C+_i = max(0, x_i - (mu0 + K) + C+_(i-1)); C-_i = max(0, (mu0 - K) - x_i + C-_(i-1)); signal when either "
            "exceeds H",
     latex=r"C_i^+ = \max(0,\,x_i-(\mu_0+K)+C_{i-1}^+),\quad C_i^- = \max(0,\,(\mu_0-K)-x_i+C_{i-1}^-)",
     run=tabular_cusum, reference=_ref_cusum,
     wrong=lambda data, target, k, h: {"final_c_plus": sum(v - target - k for v in data) + 1.0},
     rand=_rand_cusum,
     triggers=["cusum", "tabular cusum", "cumulative sum chart"], ask="tabular CUSUM",
     examples=[{"data": [9.45, 7.99, 9.29, 11.66, 12.16, 10.18, 8.04, 11.46, 9.2, 10.34, 11.8, 12.2, 11.9, 12.5],
                "target": 10, "k": 0.5, "h": 5},
               {"data": [50.2, 49.1, 51.0, 52.3, 53.1, 52.8, 54.0], "target": 50, "k": 1, "h": 4}],
     invalid=[{"data": [9.45, 7.99], "target": 10, "k": 0.5, "h": 0}],
     weak="the reference uses the closed form C_i = S_i - min S_j in exact fractions (an independent formulation of "
          "the same recursion)")


def dpmo_sigma(defects, units, opportunities):
    d, u = _nonneg("defects", defects), _pos("units", units)
    o = _pos("opportunities", opportunities)
    dpo = d / (u * o)
    _need(0 < dpo < 1, "defects per opportunity must lie strictly between 0 and 1")
    return _rd({"dpmo": dpo * 1e6, "dpu": d / u, "dpo": dpo, "yield": 1 - dpo, "sigma_level": _zinv(1 - dpo) + 1.5})


def _ref_dpmo(defects, units, opportunities):
    from scipy.stats import norm
    dpo = defects / (units * opportunities)
    return {"dpmo": dpo * 1e6, "dpu": defects / units, "sigma_level": float(norm.isf(dpo)) + 1.5}


_add("ie_dpmo_sigma_level", name="DPMO and sigma level", topic=f"{_SPC} / six sigma", level="intermediate",
     purpose="Defects per unit, per opportunity and per million opportunities, and the sigma level (with the "
             "conventional 1.5 sigma shift).",
     inputs=[_i("defects", "number of defects found", "number of defects", "defects found", "total defects"),
             _i("units", "number of units inspected", "number of units", "units inspected", "sample size"),
             _i("opportunities", "defect opportunities per unit", "opportunities per unit", "opportunities for defects")],
     outputs=[_o("dpmo", "defects per million opportunities"), _o("dpu", "defects per unit"),
              _o("dpo", "defects per opportunity"), _o("yield", "1 - DPO"),
              _o("sigma_level", "short-term sigma level Phi^-1(1 - DPO) + 1.5")],
     primary="dpmo", conditions=["defects >= 0", "units, opportunities > 0", "0 < DPO < 1"],
     method="DPO = defects/(units opportunities), DPMO = 1e6 DPO, sigma = Phi^-1(1 - DPO) + 1.5",
     latex=r"DPMO = \frac{10^6\,D}{U\,O},\quad \text{sigma} = \Phi^{-1}(1 - DPO) + 1.5",
     run=dpmo_sigma, reference=_ref_dpmo,
     wrong=lambda defects, units, opportunities: {"dpmo": defects / units * 1e6},
     rand=lambda rng: {"defects": rng.randint(1, 500), "units": rng.randint(100, 20000), "opportunities": rng.randint(2, 50)},
     triggers=["dpmo", "defects per million opportunities", "sigma level"], ask="DPMO and sigma level",
     examples=[{"defects": 21, "units": 500, "opportunities": 8}, {"defects": 3, "units": 1000, "opportunities": 10}],
     invalid=[{"defects": 21, "units": 0, "opportunities": 8}])


def rolled_yield(yields):
    y = _vec("yields", yields, 1, 200)
    for v in y:
        _need(0 < v <= 1, "every yield must lie in (0, 1]")
    p = 1.0
    for v in y:
        p *= v
    return _rd({"rty": p, "normalized_yield": p ** (1.0 / len(y)), "total_dpu": -math.log(p) + 0.0})


def _ref_rty(yields):
    from scipy.stats import gmean
    g = float(gmean(yields))
    return {"rty": g ** len(yields), "normalized_yield": g, "total_dpu": -len(yields) * math.log(g)}


_add("ie_rolled_throughput_yield", name="Rolled throughput yield", topic=f"{_SPC} / six sigma", level="beginner",
     purpose="Probability that a unit passes every process step defect-free (product of step yields), its geometric "
             "mean per step and the equivalent total defects per unit -ln(RTY).",
     inputs=[_i("yields", "first-pass yield of each step (decimals)", "step yields", "first pass yields",
                "process yields", type="vector")],
     outputs=[_o("rty", "rolled throughput yield"), _o("normalized_yield", "normalized yield RTY^(1/k)"),
              _o("total_dpu", "equivalent total defects per unit -ln RTY")],
     primary="rty", conditions=["each yield in (0, 1]"],
     method="RTY = product of yields; normalized = RTY^(1/k); DPU = -ln RTY",
     latex=r"RTY = \prod_i Y_i,\quad Y_{norm} = RTY^{1/k}", run=rolled_yield, reference=_ref_rty,
     wrong=lambda yields: {"rty": sum(yields) / len(yields)},
     rand=lambda rng: {"yields": [_u(rng, 0.8, 0.999, 3) for _ in range(rng.randint(2, 8))]},
     triggers=["rolled throughput yield", "throughput yield", "rty"], ask="rolled throughput yield",
     examples=[{"yields": [0.95, 0.98, 0.9, 0.99]}, {"yields": [0.92, 0.97]}],
     invalid=[{"yields": [0.95, 1.2]}],
     weak="the reference forms the product through SciPy's geometric mean (the same definition)")


def acceptance_sampling(sample_size, acceptance_number, fraction_defective, lot_size):
    n, c = _int("sample_size", sample_size, 1, 2000), _int("acceptance_number", acceptance_number, 0)
    p, N = _prob("fraction_defective", fraction_defective), _int("lot_size", lot_size, 1)
    _need(c < n, "the acceptance number must be below the sample size")
    _need(N >= n, "the lot must be at least as large as the sample")
    pa = _binom_cdf(c, n, p)
    return _rd({"probability_accept": pa, "aoq": pa * p * (N - n) / N, "ati": n + (1 - pa) * (N - n),
                "probability_reject": 1 - pa})


def _ref_acceptance(sample_size, acceptance_number, fraction_defective, lot_size):
    from scipy.stats import binom
    n, N = int(sample_size), int(lot_size)
    pa = float(binom.cdf(int(acceptance_number), n, fraction_defective))
    return {"probability_accept": pa, "aoq": pa * fraction_defective * (N - n) / N, "ati": n + (1 - pa) * (N - n)}


def _rand_accept(rng):
    n = rng.randint(10, 200)
    c = rng.randint(0, max(0, n // 20))
    p = _u(rng, 0.2 / n, min(0.3, 4 * (c + 1) / n), 4)
    return {"sample_size": n, "acceptance_number": c, "fraction_defective": p, "lot_size": n * rng.randint(2, 50)}


_add("ie_acceptance_sampling", name="Single sampling plan: OC, AOQ and ATI", topic=f"{_SPC} / acceptance sampling",
     level="expert",
     purpose="Probability of accepting a lot under a single sampling plan (n, c), the average outgoing quality with "
             "rectifying inspection and the average total inspection.",
     inputs=[_i("sample_size", "items sampled n", "n", "sample size", "items sampled", type="integer"),
             _i("acceptance_number", "acceptance number c", "c", "acceptance number", "accept number", type="integer"),
             _i("fraction_defective", "lot fraction defective p (decimal)", "p", "lot fraction defective",
                "incoming quality", "proportion defective"),
             _i("lot_size", "lot size N", "n lot", "lot size", "lot", type="integer")],
     outputs=[_o("probability_accept", "P(accept) = P(X <= c), X ~ Binomial(n, p)"),
              _o("aoq", "average outgoing quality Pa p (N - n)/N"), _o("ati", "average total inspection n + (1 - Pa)(N - n)"),
              _o("probability_reject", "1 - Pa")],
     primary="probability_accept", conditions=["0 <= c < n <= N", "0 < p < 1", "binomial sampling", "n <= 2000"],
     method="Pa = sum_{k<=c} C(n,k) p^k (1-p)^(n-k); AOQ = Pa p (N-n)/N; ATI = n + (1 - Pa)(N - n)",
     latex=r"P_a = \sum_{k=0}^{c}\binom{n}{k}p^k(1-p)^{n-k},\quad AOQ = \frac{P_a\,p\,(N-n)}{N}",
     run=acceptance_sampling, reference=_ref_acceptance,
     wrong=lambda sample_size, acceptance_number, fraction_defective, lot_size: {
         "probability_accept": _binom_cdf(int(acceptance_number), int(sample_size), fraction_defective)
         - math.comb(int(sample_size), int(acceptance_number)) * fraction_defective ** int(acceptance_number)
         * (1 - fraction_defective) ** (int(sample_size) - int(acceptance_number))},
     rand=_rand_accept,
     triggers=["single sampling plan", "acceptance sampling", "operating characteristic", "oc curve"],
     ask="single sampling plan",
     examples=[{"sample_size": 50, "acceptance_number": 1, "fraction_defective": 0.02, "lot_size": 2000},
               {"sample_size": 89, "acceptance_number": 2, "fraction_defective": 0.01, "lot_size": 10000}],
     invalid=[{"sample_size": 50, "acceptance_number": 60, "fraction_defective": 0.02, "lot_size": 2000}])


# =====================================================================================================================
# RELIABILITY AND AVAILABILITY
# =====================================================================================================================
_REL = "reliability"


def _enumerate_reliability(rs, works):
    """Exact system reliability by summing the probability of every component state in which the system works."""
    import itertools
    import numpy as np
    total = 0.0
    for state in itertools.product((0, 1), repeat=len(rs)):
        pr = float(np.prod([r if s else 1 - r for r, s in zip(rs, state)]))
        if works(state):
            total += pr
    return total


def _rel_list(name, rs):
    r = _vec(name, rs, 1, 500)
    for v in r:
        _need(0 <= v <= 1, f"every value of {name} must lie between 0 and 1")
    return r


def series_reliability(reliabilities):
    r = _rel_list("reliabilities", reliabilities)
    p = 1.0
    for v in r:
        p *= v
    return _rd({"reliability": p, "unreliability": 1 - p})


def _ref_series(reliabilities):
    R = _enumerate_reliability(reliabilities, lambda s: all(s))
    return {"reliability": R, "unreliability": 1 - R}


_RS_IN = lambda: _i("reliabilities", "reliability of each component (decimals)", "component reliabilities",
                    "reliability values", "component reliability", type="vector")

_add("ie_series_reliability", name="Reliability of components in series", topic=f"{_REL} / systems", level="beginner",
     purpose="Reliability of a system that works only if every component works (independent components).",
     inputs=[_RS_IN()],
     outputs=[_o("reliability", "system reliability product of R_i"), _o("unreliability", "1 - system reliability")],
     primary="reliability", conditions=["each reliability in [0, 1]", "independent failures"],
     method="R = product R_i", latex=r"R_s = \prod_{i=1}^{n} R_i", run=series_reliability, reference=_ref_series,
     wrong=lambda reliabilities: {"reliability": min(reliabilities)},
     rand=lambda rng: {"reliabilities": [_u(rng, 0.7, 0.999, 3) for _ in range(rng.randint(2, 10))]},
     triggers=["series reliability", "reliability of components in series", "series system reliability",
               "components in series"], ask="series system reliability",
     examples=[{"reliabilities": [0.95, 0.98, 0.99]}, {"reliabilities": [0.9, 0.9, 0.85, 0.97]}],
     invalid=[{"reliabilities": [0.95, 1.5]}])


def parallel_reliability(reliabilities):
    r = _rel_list("reliabilities", reliabilities)
    q = 1.0
    for v in r:
        q *= 1 - v
    return _rd({"reliability": 1 - q, "unreliability": q})


def _ref_parallel(reliabilities):
    R = _enumerate_reliability(reliabilities, lambda s: any(s))
    return {"reliability": R, "unreliability": 1 - R}


_add("ie_parallel_reliability", name="Reliability of components in parallel", topic=f"{_REL} / systems",
     level="beginner",
     purpose="Reliability of a redundant system that works if at least one component works (independent components).",
     inputs=[_RS_IN()],
     outputs=[_o("reliability", "1 - product (1 - R_i)"), _o("unreliability", "product (1 - R_i)")],
     primary="unreliability", conditions=["each reliability in [0, 1]", "independent failures", "active redundancy"],
     method="R = 1 - product (1 - R_i)", latex=r"R_p = 1 - \prod_{i=1}^{n}(1 - R_i)", run=parallel_reliability,
     reference=_ref_parallel,
     wrong=lambda reliabilities: {"unreliability": 1 - max(reliabilities)},
     rand=lambda rng: {"reliabilities": [_u(rng, 0.5, 0.99, 3) for _ in range(rng.randint(2, 10))]},
     triggers=["parallel reliability", "reliability of components in parallel", "parallel system reliability",
               "components in parallel", "redundant system reliability"], ask="parallel system reliability",
     examples=[{"reliabilities": [0.9, 0.8]}, {"reliabilities": [0.95, 0.9, 0.85]}],
     invalid=[{"reliabilities": [-0.1, 0.9]}])


def k_out_of_n(component_reliability, k, n):
    r = _fin("component_reliability", component_reliability)
    _need(0 <= r <= 1, "component_reliability must lie between 0 and 1")
    nn, kk = _int("n", n, 1, 1000), _int("k", k, 1)
    _need(kk <= nn, "k must not exceed n")
    s = 0.0
    for j in range(kk, nn + 1):
        s += math.comb(nn, j) * r ** j * (1 - r) ** (nn - j)
    return _rd({"reliability": min(1.0, s), "unreliability": max(0.0, 1 - s)})


def _ref_kofn(component_reliability, k, n):
    from scipy.stats import binom
    R = float(binom.sf(int(k) - 1, int(n), component_reliability))
    return {"reliability": R}


_add("ie_k_out_of_n_reliability", name="k-out-of-n system reliability", topic=f"{_REL} / systems",
     level="intermediate",
     purpose="Reliability of a system of n identical independent components that works when at least k of them work.",
     inputs=[_i("component_reliability", "reliability of each component", "component reliability", "r", "reliability"),
             _i("k", "components needed", "required components", "k components", type="integer"),
             _i("n", "components installed", "total components", "number of components", type="integer")],
     outputs=[_o("reliability", "P(at least k of n work)"), _o("unreliability", "1 - reliability")],
     primary="reliability", conditions=["0 <= r <= 1", "1 <= k <= n <= 1000", "identical independent components"],
     method="R = sum_{j=k}^{n} C(n, j) r^j (1 - r)^(n - j)",
     latex=r"R = \sum_{j=k}^{n}\binom{n}{j}r^j(1-r)^{n-j}", run=k_out_of_n, reference=_ref_kofn,
     wrong=lambda component_reliability, k, n: {"reliability": math.comb(int(n), int(k)) * component_reliability ** int(k)
                                               * (1 - component_reliability) ** (int(n) - int(k)) * 0.999},
     rand=lambda rng: (lambda n: {"component_reliability": _u(rng, 0.5, 0.99, 3), "n": n, "k": rng.randint(1, n)})(rng.randint(2, 12)),
     triggers=["k out of n", "k-out-of-n", "voting system reliability"], ask="k-out-of-n system reliability",
     examples=[{"component_reliability": 0.9, "k": 2, "n": 3}, {"component_reliability": 0.95, "k": 3, "n": 5}],
     invalid=[{"component_reliability": 0.9, "k": 4, "n": 3}])


def series_parallel(stage_reliabilities):
    m = _mat("stage_reliabilities", stage_reliabilities)
    R = 1.0
    for row in m:
        q = 1.0
        for v in row:
            _need(0 <= v <= 1, "every reliability must lie between 0 and 1")
            q *= 1 - v
        R *= 1 - q
    return _rd({"reliability": R, "unreliability": 1 - R})


def _ref_series_parallel(stage_reliabilities):
    flat = [v for row in stage_reliabilities for v in row]
    w = len(stage_reliabilities[0])
    def works(s):
        return all(any(s[i * w:(i + 1) * w]) for i in range(len(stage_reliabilities)))
    R = _enumerate_reliability(flat, works)
    return {"reliability": R, "unreliability": 1 - R}


_add("ie_series_parallel_reliability", name="Series system of parallel stages", topic=f"{_REL} / systems",
     level="intermediate",
     purpose="Reliability of stages in series where each stage has redundant components in parallel (each row of the "
             "matrix is one stage).",
     inputs=[_i("stage_reliabilities", "matrix: one row per stage, the reliabilities of its parallel components",
                "stages", "stage reliabilities", "reliability matrix", type="matrix")],
     outputs=[_o("reliability", "product over stages of 1 - product (1 - r)"), _o("unreliability", "1 - reliability")],
     primary="reliability", conditions=["every value in [0, 1]", "independent components", "rows of equal length"],
     method="R = prod_i [1 - prod_j (1 - r_ij)]", latex=r"R = \prod_i\Big[1 - \prod_j (1 - r_{ij})\Big]",
     run=series_parallel, reference=_ref_series_parallel,
     wrong=lambda stage_reliabilities: {"reliability": 1 - math.prod(1 - math.prod(r) for r in stage_reliabilities)},
     rand=lambda rng: (lambda s, w: {"stage_reliabilities": [[_u(rng, 0.6, 0.98, 3) for _ in range(w)] for _ in range(s)]})(
         rng.randint(2, 4), rng.randint(2, 3)),
     triggers=["series parallel system reliability", "series parallel system","parallel stages in series",
               "series of parallel stages", "series parallel reliability"], ask="series-parallel system reliability",
     examples=[{"stage_reliabilities": [[0.9, 0.9], [0.8, 0.8], [0.95, 0.95]]},
               {"stage_reliabilities": [[0.7, 0.8, 0.9], [0.85, 0.6, 0.75]]}],
     invalid=[{"stage_reliabilities": [[0.9, 1.9], [0.8, 0.8]]}])


def exponential_reliability(failure_rate, time):
    lam, t = _pos("failure_rate", failure_rate), _nonneg("time", time)
    R = math.exp(-lam * t)
    return _rd({"reliability": R, "unreliability": -math.expm1(-lam * t), "mtbf": 1 / lam, "hazard_rate": lam})


def _ref_exp_rel(failure_rate, time):
    from scipy.stats import expon
    d = expon(scale=1 / failure_rate)
    return {"reliability": float(d.sf(time)), "unreliability": float(d.cdf(time)), "mtbf": float(d.mean())}


_add("ie_exponential_reliability", name="Exponential (constant failure rate) reliability", topic=f"{_REL} / life models",
     level="beginner",
     purpose="Probability of surviving a mission time with a constant failure rate: R(t) = exp(-lambda t), MTBF = 1/lambda.",
     inputs=[_i("failure_rate", "constant failure rate lambda (failures per time unit)", "lambda", "failure rate",
                "hazard rate"),
             _i("time", "mission time in the same time unit", "t", "mission time", "operating time")],
     outputs=[_o("reliability", "R(t) = exp(-lambda t)"), _o("unreliability", "F(t) = 1 - R(t)"),
              _o("mtbf", "mean time between failures 1/lambda"), _o("hazard_rate", "lambda")],
     primary="reliability", conditions=["failure_rate > 0", "time >= 0"],
     method="R = exp(-lambda t)", latex=r"R(t) = e^{-\lambda t},\quad MTBF = \frac{1}{\lambda}",
     run=exponential_reliability, reference=_ref_exp_rel,
     wrong=lambda failure_rate, time: {"reliability": max(0.0, 1 - failure_rate * time * 1.5)},
     rand=lambda rng: {"failure_rate": _u(rng, 1e-5, 1e-2, 7), "time": _u(rng, 10, 2000, 1)},
     triggers=["exponential reliability", "constant failure rate reliability", "reliability with constant failure rate"],
     ask="exponential reliability",
     examples=[{"failure_rate": 0.0002, "time": 1000}, {"failure_rate": 0.001, "time": 500}],
     invalid=[{"failure_rate": -0.001, "time": 500}])


def availability(mtbf, mttr, hours_per_year=8760.0):
    up, down = _pos("mtbf", mtbf), _pos("mttr", mttr)
    hpy = _pos("hours_per_year", hours_per_year)
    A = up / (up + down)
    return _rd({"availability": A, "unavailability": down / (up + down), "downtime_per_year": hpy * down / (up + down),
                "failure_rate": 1 / up, "repair_rate": 1 / down})


def _ref_availability(mtbf, mttr, hours_per_year=8760.0):
    import numpy as np
    lam, mu = 1 / mtbf, 1 / mttr
    Q = np.array([[-lam, lam], [mu, -mu]])               # two-state Markov chain: up, down
    A = np.vstack([Q.T[:1], np.ones(2)])
    pi = np.linalg.solve(A, np.array([0.0, 1.0]))
    return {"availability": float(pi[0]), "unavailability": float(pi[1]), "downtime_per_year": hours_per_year * float(pi[1])}


_add("ie_availability", name="Inherent availability from MTBF and MTTR", topic=f"{_REL} / availability",
     level="beginner",
     purpose="Long-run fraction of time a repairable system is up, A = MTBF / (MTBF + MTTR), and the downtime it implies.",
     inputs=[_i("mtbf", "mean time between failures (hours)", "mean time between failures", "mean time to failure",
                "mttf", "uptime"),
             _i("mttr", "mean time to repair (hours)", "mean time to repair", "repair time", "downtime per failure"),
             _i("hours_per_year", "operating hours in a year (default 8760)", "hours", "operating hours", required=False)],
     outputs=[_o("availability", "MTBF/(MTBF + MTTR)"), _o("unavailability", "MTTR/(MTBF + MTTR)"),
              _o("downtime_per_year", "expected downtime hours per year"), _o("failure_rate", "1/MTBF"),
              _o("repair_rate", "1/MTTR")],
     primary="availability", conditions=["mtbf, mttr > 0"],
     method="A = MTBF/(MTBF + MTTR)", latex=r"A = \frac{MTBF}{MTBF + MTTR}", run=availability, reference=_ref_availability,
     wrong=lambda mtbf, mttr, hours_per_year=8760.0: {"availability": 1 - mttr / mtbf},
     rand=lambda rng: {"mtbf": _u(rng, 10, 5000, 1), "mttr": _u(rng, 0.5, 100, 2)},
     triggers=["availability", "inherent availability", "system availability"], ask="inherent availability",
     examples=[{"mtbf": 500, "mttr": 20}, {"mtbf": 1200, "mttr": 8, "hours_per_year": 6000}],
     invalid=[{"mtbf": 500, "mttr": 0}])


def weibull_reliability(shape, scale, time, location=0.0):
    b, eta = _pos("shape", shape), _pos("scale", scale)
    g = _nonneg("location", location)
    t = _fin("time", time)
    _need(t >= g, "time must not be before the location (failure-free time)")
    x = (t - g) / eta
    R = math.exp(-x ** b)
    h = b / eta * x ** (b - 1) if x > 0 else (0.0 if b > 1 else (1 / eta if b == 1 else math.inf))
    _need(math.isfinite(h), "the hazard rate is infinite at the location when shape < 1")
    return _rd({"reliability": R, "unreliability": -math.expm1(-x ** b), "hazard_rate": h,
                "mttf": g + eta * math.gamma(1 + 1 / b), "b10_life": g + eta * (-math.log(0.9)) ** (1 / b)})


def _ref_weibull(shape, scale, time, location=0.0):
    from scipy.stats import weibull_min
    d = weibull_min(shape, loc=location, scale=scale)
    out = {"reliability": float(d.sf(time)), "mttf": float(d.mean()), "b10_life": float(d.ppf(0.1))}
    if time > location:
        out["hazard_rate"] = float(d.pdf(time) / d.sf(time))
    return out


_add("ie_weibull_reliability", name="Weibull reliability, hazard and MTTF", topic=f"{_REL} / life models",
     level="intermediate",
     purpose="Reliability, hazard rate, mean life and B10 life of a Weibull life distribution (shape beta, scale eta, "
             "optional location gamma).",
     inputs=[_i("shape", "shape parameter beta", "beta", "shape parameter", "weibull slope"),
             _i("scale", "scale parameter eta (characteristic life)", "eta", "scale parameter", "characteristic life"),
             _i("time", "time t", "t", "mission time", "operating time"),
             _i("location", "location gamma, failure-free time (default 0)", "gamma", "location parameter",
                "failure free time", required=False)],
     outputs=[_o("reliability", "R(t) = exp(-((t - gamma)/eta)^beta)"), _o("unreliability", "1 - R(t)"),
              _o("hazard_rate", "h(t) = beta/eta ((t - gamma)/eta)^(beta - 1)"), _o("mttf", "gamma + eta Gamma(1 + 1/beta)"),
              _o("b10_life", "time by which 10 % have failed")],
     primary="reliability", conditions=["shape, scale > 0", "location >= 0", "time >= location"],
     method="R = exp(-((t-g)/eta)^b); MTTF = g + eta Gamma(1 + 1/b); B10 = g + eta (-ln 0.9)^(1/b)",
     latex=r"R(t) = \exp\!\left[-\left(\frac{t-\gamma}{\eta}\right)^{\beta}\right],\quad MTTF = \gamma + \eta\,\Gamma(1+1/\beta)",
     run=weibull_reliability, reference=_ref_weibull,
     wrong=lambda shape, scale, time, location=0.0: {"reliability": math.exp(-((time - location) / scale) * shape),
                                                    "mttf": scale / shape},
     rand=lambda rng: (lambda g, eta: {"shape": _u(rng, 0.5, 4, 3), "scale": eta, "location": g,
                      "time": round(g + eta * rng.uniform(0.05, 2.5), 1)})(rng.choice([0, 0, _u(rng, 10, 500, 1)]), _u(rng, 100, 10000, 1)),
     triggers=["weibull reliability", "weibull distribution reliability", "weibull life", "weibull hazard"],
     ask="Weibull reliability",
     examples=[{"shape": 2, "scale": 1000, "time": 500}, {"shape": 1.5, "scale": 5000, "time": 2000, "location": 200}],
     invalid=[{"shape": 2, "scale": 0, "time": 500}])


def weibull_regression(failure_times):
    t = _vec("failure_times", failure_times, 3, 1000, positive=True)
    t = sorted(t)
    n = len(t)
    xs, ys = [], []
    for i in range(n):
        F = (i + 1 - 0.3) / (n + 0.4)
        xs.append(math.log(t[i]))
        ys.append(math.log(-math.log(1 - F)))
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    _need(sxx > 0, "the failure times must not all be equal")
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    syy = sum((y - my) ** 2 for y in ys)
    b = sxy / sxx
    a = my - b * mx
    return _rd({"shape": b, "scale": math.exp(-a / b), "r_squared": sxy * sxy / (sxx * syy)})


def _ref_weibull_reg(failure_times):
    import numpy as np
    import statsmodels.api as sm
    t = np.sort(np.asarray(failure_times, float))
    n = len(t)
    F = (np.arange(1, n + 1) - 0.3) / (n + 0.4)
    y = np.log(-np.log(1 - F))
    fit = sm.OLS(y, sm.add_constant(np.log(t))).fit()
    a, b = fit.params
    return {"shape": float(b), "scale": float(np.exp(-a / b)), "r_squared": float(fit.rsquared)}


_add("ie_weibull_median_rank_regression", name="Weibull fit by median-rank regression", topic=f"{_REL} / life data",
     level="advanced",
     purpose="Weibull shape and scale estimated from complete failure times by regression on Bernard's median ranks "
             "(Weibull probability plot).",
     inputs=[_i("failure_times", "failure times (complete data, any order)", "failure times", "times to failure",
                "lifetimes", type="vector")],
     outputs=[_o("shape", "shape beta (slope of ln ln(1/(1-F)) on ln t)"), _o("scale", "scale eta"),
              _o("r_squared", "coefficient of determination of the plot")],
     primary="shape", conditions=["at least 3 positive failure times, not all equal", "complete (uncensored) data"],
     method="F_i = (i - 0.3)/(n + 0.4); regress y = ln(-ln(1 - F_i)) on x = ln t_(i); beta = slope, "
            "eta = exp(-intercept/beta)",
     latex=r"\ln\ln\frac{1}{1-F_i} = \beta\ln t_{(i)} - \beta\ln\eta,\quad F_i = \frac{i-0.3}{n+0.4}",
     run=weibull_regression, reference=_ref_weibull_reg,
     wrong=lambda failure_times: (lambda n: (lambda xs, ys: {"shape": (sum(ys) / n) / (sum(xs) / n)})(
         [math.log(v) for v in sorted(failure_times)],
         [math.log(-math.log(1 - (i + 1) / (n + 1))) for i in range(n)]))(len(failure_times)),
     rand=lambda rng: (lambda b, e: {"failure_times": [round(rng.weibullvariate(e, b), 1) + 0.1 for _ in range(rng.randint(5, 20))]})(
         _u(rng, 0.8, 4, 2), _u(rng, 100, 5000, 0)),
     triggers=["weibull fit", "median rank regression", "weibull probability plot", "fit a weibull"],
     ask="Weibull fit by median rank regression",
     examples=[{"failure_times": [16, 34, 53, 75, 93, 120]}, {"failure_times": [1200, 850, 2300, 1700, 3100, 640, 1950]}],
     invalid=[{"failure_times": [16, -34, 53]}])


def redundancy_needed(target_reliability, component_reliability):
    Rt = _prob("target_reliability", target_reliability)
    r = _prob("component_reliability", component_reliability)
    m, q = 1, 1 - r
    while 1 - q < Rt:
        m += 1
        q *= 1 - r
        _need(m <= 10000, "more than 10000 components would be needed")
    return _rd({"components": m, "achieved_reliability": 1 - q})


def _ref_redundancy(target_reliability, component_reliability):
    import numpy as np
    m = int(np.ceil(np.log(1 - target_reliability) / np.log(1 - component_reliability)))
    for c in (m - 1, m, m + 1):                            # guard the logarithm's last bit
        if c >= 1 and 1 - (1 - component_reliability) ** c >= target_reliability:
            m = c
            break
    return {"components": m, "achieved_reliability": 1 - (1 - component_reliability) ** m}


_add("ie_redundancy_needed", name="Parallel components needed for a target reliability", topic=f"{_REL} / systems",
     level="advanced",
     purpose="Smallest number of identical components in parallel that reaches a target system reliability.",
     inputs=[_i("target_reliability", "required system reliability", "required reliability", "target",
                "system reliability"),
             _i("component_reliability", "reliability of one component", "component reliability", "r")],
     outputs=[_o("components", "smallest m with 1 - (1 - r)^m >= target"),
              _o("achieved_reliability", "reliability with m components")],
     primary="achieved_reliability", conditions=["0 < target < 1", "0 < r < 1", "active parallel redundancy"],
     method="smallest m with 1 - (1 - r)^m >= R_target (m = ceil(ln(1 - R)/ln(1 - r)))",
     latex=r"m = \left\lceil \frac{\ln(1-R_{target})}{\ln(1-r)} \right\rceil", run=redundancy_needed,
     reference=_ref_redundancy,
     wrong=lambda target_reliability, component_reliability: {"achieved_reliability": 1 - (1 - component_reliability) ** max(1, int(math.log(1 - target_reliability) / math.log(1 - component_reliability))) + 0.5 * (1 - target_reliability)},
     rand=lambda rng: {"target_reliability": rng.choice([0.99, 0.999, 0.9999, _u(rng, 0.9, 0.99999, 5)]),
                       "component_reliability": _u(rng, 0.5, 0.98, 3)},
     triggers=["redundant components needed", "components needed in parallel", "parallel components needed",
               "redundancy needed"], ask="parallel components needed",
     examples=[{"target_reliability": 0.999, "component_reliability": 0.9},
               {"target_reliability": 0.9999, "component_reliability": 0.75}],
     invalid=[{"target_reliability": 1.2, "component_reliability": 0.9}])


def system_failure_rate(failure_rates, time=None):
    lam = _vec("failure_rates", failure_rates, 1, 1000, positive=True)
    L = sum(lam)
    out = {"system_failure_rate": L, "mtbf": 1 / L}
    if time is not None:
        t = _nonneg("time", time)
        out["reliability"] = math.exp(-L * t)
    return _rd(out)


def _ref_sys_lambda(failure_rates, time=None):
    import numpy as np
    lam = np.asarray(failure_rates, float)
    R = lambda t: float(np.exp(-lam * t).prod())
    mtbf = _quad(R, 0.0, 200.0 / lam.sum())                # MTBF = integral of R(t) dt
    out = {"mtbf": mtbf, "system_failure_rate": 1 / mtbf}
    if time is not None:
        out["reliability"] = R(time)
    return out


_add("ie_system_failure_rate", name="Failure rate and MTBF of a series system", topic=f"{_REL} / life models",
     level="intermediate",
     purpose="Failure rate, MTBF and mission reliability of a series system of components with constant failure rates.",
     inputs=[_i("failure_rates", "constant failure rate of each component (per hour, say)", "component failure rates",
                "lambdas", "failure rate of each component", type="vector"),
             _i("time", "mission time (optional)", "t", "mission time", required=False)],
     outputs=[_o("system_failure_rate", "sum of lambda_i"), _o("mtbf", "1 / sum lambda_i"),
              _o("reliability", "exp(-sum lambda_i t) (when a time is given)")],
     primary="mtbf", conditions=["failure rates > 0", "time >= 0", "series system, constant failure rates"],
     method="lambda_s = sum lambda_i, MTBF = 1/lambda_s, R(t) = exp(-lambda_s t)",
     latex=r"\lambda_s = \sum_i \lambda_i,\quad MTBF_s = \frac{1}{\lambda_s}", run=system_failure_rate,
     reference=_ref_sys_lambda,
     wrong=lambda failure_rates, time=None: {"mtbf": sum(1 / v for v in failure_rates)},
     rand=lambda rng: _drop_none({"failure_rates": [_u(rng, 1e-5, 1e-3, 7) for _ in range(rng.randint(2, 8))],
                                  "time": rng.choice([None, _u(rng, 10, 5000, 1)])}),
     triggers=["system failure rate", "failure rate of a series system", "series system mtbf", "mtbf of a series system"],
     ask="failure rate of a series system",
     examples=[{"failure_rates": [0.0001, 0.0002, 0.00005], "time": 1000}, {"failure_rates": [0.002, 0.001]}],
     invalid=[{"failure_rates": [0.0001, -0.0002]}])


def parallel_mttf(failure_rate, n):
    lam, nn = _pos("failure_rate", failure_rate), _int("n", n, 1, 1000)
    s = 0.0
    for i in range(1, nn + 1):
        s += 1.0 / i
    return _rd({"mttf": s / lam, "single_mttf": 1 / lam, "improvement_factor": s})


def _ref_parallel_mttf(failure_rate, n):
    nn = int(n)
    m = _quad(lambda t: -math.expm1(nn * math.log1p(-math.exp(-failure_rate * t))) if t > 0 else 1.0, 0.0, 60.0 * (1 + math.log(nn)) / failure_rate)
    return {"mttf": m, "improvement_factor": m * failure_rate}


_add("ie_parallel_mttf", name="MTTF of identical components in parallel", topic=f"{_REL} / life models",
     level="advanced",
     purpose="Mean time to failure of n identical exponential components in active parallel: (1/lambda) sum 1/i.",
     inputs=[_i("failure_rate", "failure rate lambda of each component", "lambda", "failure rate"),
             _i("n", "number of components in parallel", "components", "number of components", type="integer")],
     outputs=[_o("mttf", "system MTTF (1/lambda) sum_{i=1}^{n} 1/i"), _o("single_mttf", "1/lambda"),
              _o("improvement_factor", "sum 1/i")],
     primary="mttf", conditions=["failure_rate > 0", "1 <= n <= 1000", "active redundancy, constant failure rate"],
     method="MTTF = integral of 1 - (1 - e^(-lambda t))^n dt = (1/lambda) sum 1/i",
     latex=r"MTTF = \frac{1}{\lambda}\sum_{i=1}^{n}\frac{1}{i}", run=parallel_mttf, reference=_ref_parallel_mttf,
     wrong=lambda failure_rate, n: {"mttf": int(n) / failure_rate},
     rand=lambda rng: {"failure_rate": _u(rng, 1e-5, 1e-2, 7), "n": rng.randint(2, 10)},
     triggers=["mttf of parallel components", "parallel mttf", "mttf of a parallel system", "parallel system mttf"],
     ask="MTTF of a parallel system",
     examples=[{"failure_rate": 0.001, "n": 2}, {"failure_rate": 0.0005, "n": 3}],
     invalid=[{"failure_rate": 0.001, "n": 0}])


def standby_redundancy(failure_rate, time, units=2):
    lam, t, n = _pos("failure_rate", failure_rate), _nonneg("time", time), _int("units", units, 1, 200)
    x = lam * t
    term, s = 1.0, 1.0
    for k in range(1, n):
        term *= x / k
        s += term
    return _rd({"reliability": math.exp(-x) * s, "mttf": n / lam})


def _ref_standby(failure_rate, time, units=2):
    from scipy.stats import gamma
    d = gamma(int(units), scale=1 / failure_rate)          # system life = sum of n exponential lives (Erlang)
    return {"reliability": float(d.sf(time)), "mttf": float(d.mean())}


_add("ie_standby_redundancy", name="Cold standby redundancy", topic=f"{_REL} / systems", level="advanced",
     purpose="Reliability and MTTF of one operating unit backed by identical cold standby units with perfect switching.",
     inputs=[_i("failure_rate", "failure rate lambda of each unit", "lambda", "failure rate"),
             _i("time", "mission time", "t", "mission time"),
             _i("units", "total units including the operating one (default 2)", "n", "number of units", type="integer",
                required=False)],
     outputs=[_o("reliability", "exp(-lambda t) sum_{k<n} (lambda t)^k / k!"), _o("mttf", "n/lambda")],
     primary="reliability", conditions=["failure_rate > 0", "time >= 0", "1 <= units <= 200",
                                        "standby units do not fail, switching is perfect"],
     method="R(t) = e^(-lambda t) sum_{k=0}^{n-1} (lambda t)^k/k!",
     latex=r"R(t) = e^{-\lambda t}\sum_{k=0}^{n-1}\frac{(\lambda t)^k}{k!}", run=standby_redundancy, reference=_ref_standby,
     wrong=lambda failure_rate, time, units=2: {"reliability": 1 - (1 - math.exp(-failure_rate * time)) ** int(units)},
     rand=lambda rng: {"failure_rate": _u(rng, 1e-4, 1e-2, 6), "time": _u(rng, 50, 3000, 1), "units": rng.randint(2, 6)},
     triggers=["standby redundancy", "cold standby", "standby system", "standby units"], ask="cold standby redundancy",
     examples=[{"failure_rate": 0.001, "time": 1000}, {"failure_rate": 0.002, "time": 500, "units": 3}],
     invalid=[{"failure_rate": 0.001, "time": -5}])


def _chi2_even_cdf(x, m):
    """P(chi-square with 2m degrees of freedom <= x) = 1 - sum_{j<m} e^(-x/2) (x/2)^j / j!."""
    h = x / 2
    term, s = math.exp(-h), 0.0
    for j in range(m):
        if j > 0:
            term *= h / j
        s += term
    return 1.0 - s


def _chi2_even_ppf(p, m):
    hi = 2.0 * m + 10.0
    while _chi2_even_cdf(hi, m) < p:
        hi *= 2
    return _bisect_root(lambda x: _chi2_even_cdf(x, m) - p, 0.0, hi)


def mtbf_confidence(total_time, failures, confidence, test="time"):
    T, r = _pos("total_time", total_time), _int("failures", failures, 1, 10000)
    c = _prob("confidence", confidence)
    _need(test in ("time", "failure"), "test is 'time' (time-terminated) or 'failure' (failure-terminated)")
    dl = 2 * r + 2 if test == "time" else 2 * r
    a = 1 - c
    return _rd({"mtbf": T / r, "lower_one_sided": 2 * T / _chi2_even_ppf(c, dl // 2),
                "lower_two_sided": 2 * T / _chi2_even_ppf(1 - a / 2, dl // 2),
                "upper_two_sided": 2 * T / _chi2_even_ppf(a / 2, r)})


def _ref_mtbf_conf(total_time, failures, confidence, test="time"):
    from scipy.stats import chi2
    r = int(failures)
    dl = 2 * r + 2 if test == "time" else 2 * r
    a = 1 - confidence
    return {"mtbf": total_time / r, "lower_one_sided": 2 * total_time / float(chi2.ppf(confidence, dl)),
            "lower_two_sided": 2 * total_time / float(chi2.ppf(1 - a / 2, dl)),
            "upper_two_sided": 2 * total_time / float(chi2.ppf(a / 2, 2 * r))}


_add("ie_mtbf_confidence", name="Confidence bounds on MTBF (chi-square)", topic=f"{_REL} / life data",
     level="expert",
     purpose="Point estimate and chi-square confidence bounds of the MTBF from a life test with constant failure rate.",
     inputs=[_i("total_time", "total accumulated test time (all units)", "total test time", "accumulated time",
                "test time", "unit hours"),
             _i("failures", "number of failures observed", "number of failures", "r", type="integer"),
             _i("confidence", "confidence level (decimal)", "confidence level", "c"),
             _i("test", "time (test stopped at a set time) or failure (stopped at the r-th failure); default time",
                "test type", "termination", type="choice", choices=["time", "failure"], required=False,
                syn={"time-terminated": "time", "truncated": "time", "failure-terminated": "failure"})],
     outputs=[_o("mtbf", "point estimate T/r"), _o("lower_one_sided", "one-sided lower bound 2T/chi2(C; d)"),
              _o("lower_two_sided", "two-sided lower bound"), _o("upper_two_sided", "two-sided upper bound 2T/chi2(a/2; 2r)")],
     primary="lower_one_sided", conditions=["total_time > 0", "failures >= 1", "0 < confidence < 1",
                                            "d = 2r + 2 for a time-terminated test, 2r for a failure-terminated test"],
     method="MTBF_L = 2T / chi2_{C, d}; two-sided: 2T/chi2_{1-a/2, d} and 2T/chi2_{a/2, 2r}",
     latex=r"MTBF_L = \frac{2T}{\chi^2_{C,\,2r+2}},\quad MTBF_U = \frac{2T}{\chi^2_{\alpha/2,\,2r}}",
     run=mtbf_confidence, reference=_ref_mtbf_conf,
     wrong=lambda total_time, failures, confidence, test="time": {"lower_one_sided": 2 * total_time / _chi2_even_ppf(1 - confidence, int(failures) + 1)},
     rand=lambda rng: {"total_time": _u(rng, 500, 1e6, 0), "failures": rng.randint(1, 30), "confidence": rng.choice([0.8, 0.9, 0.95, 0.99]),
                       "test": rng.choice(["time", "failure"])},
     triggers=["mtbf confidence", "confidence bounds on mtbf", "mtbf lower bound", "confidence interval for mtbf",
               "lower confidence limit on mtbf"], ask="confidence bounds on MTBF",
     examples=[{"total_time": 10000, "failures": 4, "confidence": 0.9, "test": "time"},
               {"total_time": 5000, "failures": 2, "confidence": 0.95, "test": "failure"}],
     invalid=[{"total_time": 10000, "failures": 4, "confidence": 1.9}])


# =====================================================================================================================
# QUEUEING
# =====================================================================================================================
_QUE = "queueing"
_LAM = lambda: _i("arrival_rate", "mean arrival rate lambda (per time unit)", "lambda", "arrival rate", "arrivals per hour")
_MUI = lambda: _i("service_rate", "mean service rate mu of one server (per time unit)", "mu", "service rate",
                  "services per hour")


def _bd_steady(lam, mu, c, K):
    """Stationary distribution of a birth-death chain with c servers truncated at K customers (sparse solve)."""
    import numpy as np
    from scipy.sparse import lil_matrix
    from scipy.sparse.linalg import spsolve
    A = lil_matrix((K + 1, K + 1))
    for n in range(K + 1):
        if n < K:
            A[n, n] -= lam
            A[n, n + 1] += lam
        if n > 0:
            A[n, n] -= mu * min(n, c)
            A[n, n - 1] += mu * min(n, c)
    M = A.T.tolil()
    M[K, :] = np.ones(K + 1)
    b = np.zeros(K + 1)
    b[K] = 1.0
    return np.asarray(spsolve(M.tocsr(), b))


def _bd_metrics(pi, lam, c):
    import numpy as np
    n = np.arange(len(pi))
    L = float((n * pi).sum())
    Lq = float((np.maximum(n - c, 0) * pi).sum())
    return {"L": L, "Lq": Lq, "W": L / lam, "Wq": Lq / lam, "p_wait": float(pi[c:].sum()), "p0": float(pi[0])}


def mm1(arrival_rate, service_rate):
    lam, mu = _pos("arrival_rate", arrival_rate), _pos("service_rate", service_rate)
    _need(lam < mu, "the queue is stable only if the arrival rate is below the service rate")
    rho = lam / mu
    return _rd({"utilization": rho, "L": rho / (1 - rho), "Lq": rho * rho / (1 - rho), "W": 1 / (mu - lam),
                "Wq": rho / (mu - lam), "p0": 1 - rho})


def _ref_mm1(arrival_rate, service_rate):
    rho = arrival_rate / service_rate
    K = int(math.log(1e-20) / math.log(rho)) + 20
    m = _bd_metrics(_bd_steady(arrival_rate, service_rate, 1, K), arrival_rate, 1)
    return {"utilization": 1 - m["p0"], "L": m["L"], "Lq": m["Lq"], "W": m["W"], "Wq": m["Wq"], "p0": m["p0"]}


_add("ie_mm1_queue", name="M/M/1 queue", topic=f"{_QUE} / Markovian queues", level="beginner",
     purpose="Utilisation, mean numbers in system and in queue, and mean times of a single-server queue with Poisson "
             "arrivals and exponential service.",
     inputs=[_LAM(), _MUI()],
     outputs=[_o("utilization", "rho = lambda/mu"), _o("L", "mean number in system rho/(1 - rho)"),
              _o("Lq", "mean number waiting rho^2/(1 - rho)"), _o("W", "mean time in system 1/(mu - lambda)"),
              _o("Wq", "mean wait in queue rho/(mu - lambda)"), _o("p0", "probability the system is empty")],
     primary="L", conditions=["arrival_rate < service_rate", "Poisson arrivals, exponential service, FCFS, infinite queue"],
     method="rho = lambda/mu, L = rho/(1 - rho), W = 1/(mu - lambda), Lq = rho L, Wq = rho W",
     latex=r"\rho = \frac{\lambda}{\mu},\ L = \frac{\rho}{1-\rho},\ W = \frac{1}{\mu-\lambda},\ L_q = \frac{\rho^2}{1-\rho}",
     run=mm1, reference=_ref_mm1,
     wrong=lambda arrival_rate, service_rate: {"L": arrival_rate / service_rate / (1 - arrival_rate / service_rate) ** 2},
     rand=lambda rng: (lambda mu: {"service_rate": mu, "arrival_rate": round(mu * rng.uniform(0.1, 0.93), 3)})(_u(rng, 1, 60, 2)),
     triggers=["m/m/1", "mm1", "m m 1", "single server queue", "single-server queue"], ask="M/M/1 queue",
     examples=[{"arrival_rate": 2, "service_rate": 3}, {"arrival_rate": 18, "service_rate": 20}],
     invalid=[{"arrival_rate": 5, "service_rate": 3}])


def mmc(arrival_rate, service_rate, servers):
    lam, mu, c = _pos("arrival_rate", arrival_rate), _pos("service_rate", service_rate), _int("servers", servers, 1, 500)
    a = lam / mu
    rho = a / c
    _need(rho < 1, "the queue is stable only if the arrival rate is below servers x service rate")
    term, s = 1.0, 1.0
    for k in range(1, c):
        term *= a / k
        s += term
    last = term * a / c / (1 - rho)                        # a^c / (c! (1 - rho))
    p0 = 1 / (s + last)
    pw = last * p0
    Lq = pw * rho / (1 - rho)
    Wq = Lq / lam
    return _rd({"utilization": rho, "p_wait": pw, "Lq": Lq, "Wq": Wq, "W": Wq + 1 / mu, "L": Lq + a, "p0": p0})


def _ref_mmc(arrival_rate, service_rate, servers):
    from scipy.stats import poisson
    c, a = int(servers), arrival_rate / service_rate
    rho = a / c
    B = float(poisson.pmf(c, a) / poisson.cdf(c, a))      # Erlang B through the Poisson distribution
    C = B / (1 - rho * (1 - B))                            # Erlang C from Erlang B
    Lq = C * rho / (1 - rho)
    return {"p_wait": C, "Lq": Lq, "Wq": Lq / arrival_rate, "W": Lq / arrival_rate + 1 / service_rate, "L": Lq + a}


_add("ie_mmc_queue", name="M/M/c queue (Erlang C)", topic=f"{_QUE} / Markovian queues", level="intermediate",
     purpose="Probability of waiting (Erlang C), mean queue length and mean waits of a multi-server queue with Poisson "
             "arrivals and exponential service.",
     inputs=[_LAM(), _MUI(), _i("servers", "number of servers c", "c", "number of servers", "channels", type="integer")],
     outputs=[_o("utilization", "rho = lambda/(c mu)"), _o("p_wait", "Erlang C probability of waiting"),
              _o("Lq", "mean number waiting"), _o("Wq", "mean wait in queue"), _o("W", "mean time in system"),
              _o("L", "mean number in system"), _o("p0", "probability the system is empty")],
     primary="Lq", conditions=["arrival_rate < servers x service_rate", "1 <= servers <= 500"],
     method="a = lambda/mu, P0 = [sum_{k<c} a^k/k! + a^c/(c!(1-rho))]^-1, C = a^c P0/(c!(1-rho)), Lq = C rho/(1-rho)",
     latex=r"C(c,a) = \frac{\frac{a^c}{c!(1-\rho)}}{\sum_{k=0}^{c-1}\frac{a^k}{k!} + \frac{a^c}{c!(1-\rho)}},\quad L_q = C\frac{\rho}{1-\rho}",
     run=mmc, reference=_ref_mmc,
     wrong=lambda arrival_rate, service_rate, servers: {"Lq": (arrival_rate / (servers * service_rate)) ** 2 / (1 - arrival_rate / (servers * service_rate))},
     rand=lambda rng: (lambda mu, c: {"service_rate": mu, "servers": c, "arrival_rate": round(mu * c * rng.uniform(0.2, 0.93), 3)})(
         _u(rng, 1, 30, 2), rng.randint(2, 12)),
     triggers=["m/m/c", "mmc", "m m c", "m/m/s", "erlang c", "multi server queue", "multi-server queue",
               "multiple server queue"], ask="M/M/c queue",
     examples=[{"arrival_rate": 10, "service_rate": 4, "servers": 3}, {"arrival_rate": 45, "service_rate": 6, "servers": 9}],
     invalid=[{"arrival_rate": 20, "service_rate": 4, "servers": 3}])


def _mg1_values(lam, es, var):
    rho = lam * es
    _need(rho < 1, "the queue is stable only if arrival rate x mean service time is below 1")
    Lq = lam * lam * (var + es * es) / (2 * (1 - rho)) if var > 0 else lam * lam * es * es / (2 * (1 - rho))
    Wq = Lq / lam
    return {"utilization": rho, "Lq": Lq, "Wq": Wq, "W": Wq + es, "L": Lq + rho}


def md1(arrival_rate, service_rate):
    lam, mu = _pos("arrival_rate", arrival_rate), _pos("service_rate", service_rate)
    return _rd(_mg1_values(lam, 1 / mu, 0.0))


def _pk_transform_mean(lam, mean, sd):
    """Mean wait in queue of M/G/1: the service distribution's moments are read off its Laplace-Stieltjes transform
    (deterministic, or gamma with the given mean and standard deviation) by 40-digit numerical differentiation, then
    the mean of the Pollaczek-Khinchine transform W*(s) = (1 - rho) s / (s - lam + lam B*(s)) is
    -W*'(0) = lam E[S^2] / (2 (1 - rho))."""
    import mpmath
    with mpmath.workdps(40):
        m = mpmath.mpf(repr(float(mean)))
        if sd == 0:
            B = lambda s: mpmath.exp(-s * m)
        else:
            v = mpmath.mpf(repr(float(sd))) ** 2
            k, th = m * m / v, v / m
            B = lambda s: (1 + th * s) ** (-k)
        m1 = -mpmath.diff(B, 0)
        m2 = mpmath.diff(B, 0, 2)
        L = mpmath.mpf(repr(float(lam)))
        return float(L * m2 / (2 * (1 - L * m1)))


def _ref_md1(arrival_rate, service_rate):
    wq = _pk_transform_mean(arrival_rate, 1 / service_rate, 0)
    return {"Wq": wq, "Lq": arrival_rate * wq, "W": wq + 1 / service_rate, "L": arrival_rate * (wq + 1 / service_rate)}


_add("ie_md1_queue", name="M/D/1 queue (constant service time)", topic=f"{_QUE} / M/G/1 queues", level="intermediate",
     purpose="Mean queue length and waits of a single-server queue with Poisson arrivals and a constant service time.",
     inputs=[_LAM(), _MUI()],
     outputs=[_o("utilization", "rho = lambda/mu"), _o("Lq", "mean number waiting rho^2/(2(1 - rho))"),
              _o("Wq", "mean wait in queue Lq/lambda"), _o("W", "mean time in system Wq + 1/mu"),
              _o("L", "mean number in system Lq + rho")],
     primary="Lq", conditions=["arrival_rate < service_rate", "constant service time 1/mu"],
     method="Lq = rho^2 / (2 (1 - rho)), Wq = Lq/lambda", latex=r"L_q = \frac{\rho^2}{2(1-\rho)}",
     run=md1, reference=_ref_md1,
     wrong=lambda arrival_rate, service_rate: {"Lq": (arrival_rate / service_rate) ** 2 / (1 - arrival_rate / service_rate)},
     rand=lambda rng: (lambda mu: {"service_rate": mu, "arrival_rate": round(mu * rng.uniform(0.1, 0.93), 3)})(_u(rng, 1, 60, 2)),
     triggers=["m/d/1", "md1", "m d 1", "constant service time queue", "deterministic service queue"],
     ask="M/D/1 queue",
     examples=[{"arrival_rate": 2, "service_rate": 3}, {"arrival_rate": 8, "service_rate": 10}],
     invalid=[{"arrival_rate": 3, "service_rate": 3}])


def mg1(arrival_rate, mean_service_time, sd_service_time):
    lam, es, sd = _pos("arrival_rate", arrival_rate), _pos("mean_service_time", mean_service_time), _nonneg("sd_service_time", sd_service_time)
    return _rd(_mg1_values(lam, es, sd * sd))


def _ref_mg1(arrival_rate, mean_service_time, sd_service_time):
    wq = _pk_transform_mean(arrival_rate, mean_service_time, sd_service_time)
    return {"Wq": wq, "Lq": arrival_rate * wq, "W": wq + mean_service_time, "L": arrival_rate * (wq + mean_service_time)}


_add("ie_mg1_queue", name="M/G/1 queue (Pollaczek-Khinchine)", topic=f"{_QUE} / M/G/1 queues", level="advanced",
     purpose="Mean queue length and waits of a single-server queue with Poisson arrivals and a general service time "
             "of given mean and standard deviation.",
     inputs=[_LAM(), _i("mean_service_time", "mean service time E[S]", "mean service time", "average service time",
                        "service time"),
             _i("sd_service_time", "standard deviation of the service time", "standard deviation of service time",
                "service time standard deviation", "sigma s", "sd of service time")],
     outputs=[_o("utilization", "rho = lambda E[S]"), _o("Lq", "Lq = lambda^2 (sigma^2 + E[S]^2)/(2(1 - rho))"),
              _o("Wq", "mean wait in queue"), _o("W", "mean time in system"), _o("L", "mean number in system")],
     primary="Lq", conditions=["lambda E[S] < 1", "sd_service_time >= 0"],
     method="Pollaczek-Khinchine: Lq = (lambda^2 sigma^2 + rho^2)/(2(1 - rho)), Wq = Lq/lambda, W = Wq + E[S]",
     latex=r"L_q = \frac{\lambda^2\sigma^2 + \rho^2}{2(1-\rho)}", run=mg1, reference=_ref_mg1,
     wrong=lambda arrival_rate, mean_service_time, sd_service_time: {
         "Lq": (arrival_rate ** 2 * sd_service_time ** 2 + (arrival_rate * mean_service_time) ** 2) / (1 - arrival_rate * mean_service_time)},
     rand=lambda rng: (lambda es: {"mean_service_time": es, "arrival_rate": round(rng.uniform(0.1, 0.9) / es, 4),
                                   "sd_service_time": round(es * rng.uniform(0.1, 1.5), 4)})(_u(rng, 0.05, 10, 3)),
     triggers=["m/g/1", "mg1", "m g 1", "pollaczek khinchine", "pollaczek-khinchine", "general service time queue"],
     ask="M/G/1 queue",
     examples=[{"arrival_rate": 0.5, "mean_service_time": 1.5, "sd_service_time": 1},
               {"arrival_rate": 10, "mean_service_time": 0.08, "sd_service_time": 0.05}],
     invalid=[{"arrival_rate": 1, "mean_service_time": 1.5, "sd_service_time": 1}])


def mm1k(arrival_rate, service_rate, capacity):
    lam, mu, K = _pos("arrival_rate", arrival_rate), _pos("service_rate", service_rate), _int("capacity", capacity, 1, 1000)
    rho = lam / mu
    ps, w = [], 1.0
    for n in range(K + 1):
        ps.append(w)
        w *= rho
    tot = sum(ps)
    ps = [p / tot for p in ps]
    L = sum(n * ps[n] for n in range(K + 1))
    Lq = sum((n - 1) * ps[n] for n in range(1, K + 1))
    le = lam * (1 - ps[K])
    return _rd({"p0": ps[0], "p_block": ps[K], "L": L, "Lq": Lq, "effective_arrival_rate": le, "W": L / le, "Wq": Lq / le,
                "utilization": 1 - ps[0]})


def _ref_mm1k(arrival_rate, service_rate, capacity):
    import numpy as np
    K = int(capacity)
    pi = _bd_steady(arrival_rate, service_rate, 1, K)
    n = np.arange(K + 1)
    L, Lq = float((n * pi).sum()), float((np.maximum(n - 1, 0) * pi).sum())
    le = arrival_rate * (1 - float(pi[K]))
    return {"p0": float(pi[0]), "p_block": float(pi[K]), "L": L, "Lq": Lq, "effective_arrival_rate": le, "W": L / le}


_add("ie_mm1k_queue", name="M/M/1/K queue (finite capacity)", topic=f"{_QUE} / Markovian queues", level="advanced",
     purpose="Single-server queue with room for at most K customers in the system: blocking probability, effective "
             "arrival rate, mean numbers and times.",
     inputs=[_LAM(), _MUI(), _i("capacity", "maximum customers in the system K (including the one in service)", "k",
                                "system capacity", "maximum in system", type="integer")],
     outputs=[_o("p0", "probability empty"), _o("p_block", "probability an arrival is turned away P_K"),
              _o("L", "mean number in system"), _o("Lq", "mean number waiting"),
              _o("effective_arrival_rate", "lambda (1 - P_K)"), _o("W", "mean time in system L/lambda_eff"),
              _o("Wq", "mean wait Lq/lambda_eff"), _o("utilization", "1 - P0")],
     primary="L", conditions=["arrival_rate, service_rate > 0 (rho may exceed 1)", "1 <= capacity <= 1000"],
     method="P_n = rho^n / sum_{k=0}^{K} rho^k; lambda_eff = lambda (1 - P_K); Little's law",
     latex=r"P_n = \frac{\rho^n}{\sum_{k=0}^{K}\rho^k},\quad \lambda_{eff} = \lambda(1-P_K)", run=mm1k, reference=_ref_mm1k,
     wrong=lambda arrival_rate, service_rate, capacity: {"L": sum(n * (arrival_rate / service_rate) ** n for n in range(int(capacity) + 1))
                                                         / sum((arrival_rate / service_rate) ** n for n in range(int(capacity))) },
     rand=lambda rng: (lambda mu: {"service_rate": mu, "arrival_rate": round(mu * rng.uniform(0.2, 1.6), 3), "capacity": rng.randint(1, 30)})(
         _u(rng, 1, 40, 2)),
     triggers=["m/m/1/k", "mm1k", "m m 1 k", "finite capacity queue", "finite queue"], ask="M/M/1/K queue",
     examples=[{"arrival_rate": 4, "service_rate": 5, "capacity": 3}, {"arrival_rate": 12, "service_rate": 10, "capacity": 6}],
     invalid=[{"arrival_rate": 4, "service_rate": 5, "capacity": 0}])


def _erlang_c(lam, mu, c):
    a = lam / mu
    rho = a / c
    term, s = 1.0, 1.0
    for k in range(1, c):
        term *= a / k
        s += term
    last = term * a / c / (1 - rho)
    return last / (s + last)


def staffing(arrival_rate, service_rate, target_time, target_level):
    lam, mu = _pos("arrival_rate", arrival_rate), _pos("service_rate", service_rate)
    t, sl = _nonneg("target_time", target_time), _prob("target_level", target_level)
    c = int(lam / mu) + 1
    while True:
        _need(c <= 2000, "more than 2000 servers would be needed")
        C = _erlang_c(lam, mu, c)
        level = 1 - C * math.exp(-(c * mu - lam) * t)
        if level >= sl:
            break
        c += 1
    return _rd({"servers": c, "service_level": level, "p_wait": C, "utilization": lam / (c * mu),
                "mean_wait": C / (c * mu - lam)})


def _ref_staffing(arrival_rate, service_rate, target_time, target_level):
    from scipy.stats import poisson
    a = arrival_rate / service_rate
    c = int(a) + 1
    while True:
        B = float(poisson.pmf(c, a) / poisson.cdf(c, a))  # Erlang B through the Poisson distribution
        rho = a / c
        C = B / (1 - rho * (1 - B))                        # Erlang C from Erlang B
        lev = 1 - C * math.exp(-(c * service_rate - arrival_rate) * target_time)
        if lev >= target_level:
            return {"servers": c, "service_level": lev, "p_wait": C, "mean_wait": C / (c * service_rate - arrival_rate)}
        c += 1


_add("ie_erlang_c_staffing", name="Servers needed for a service level (Erlang C)", topic=f"{_QUE} / staffing",
     level="expert",
     purpose="Fewest servers of an M/M/c queue such that the probability of waiting no longer than a target time "
             "reaches a target service level (call-centre staffing).",
     inputs=[_LAM(), _MUI(),
             _i("target_time", "target answer time in the same time unit", "answer time", "target wait",
                "acceptable wait"),
             _i("target_level", "required fraction answered within the target time (decimal)", "service level target",
                "required service level", "service level goal")],
     outputs=[_o("servers", "fewest servers meeting the target"), _o("service_level", "P(wait <= t) achieved"),
              _o("p_wait", "Erlang C probability of waiting"), _o("utilization", "lambda/(c mu)"),
              _o("mean_wait", "mean wait in queue")],
     primary="service_level", conditions=["arrival_rate, service_rate > 0", "target_time >= 0", "0 < target_level < 1"],
     method="smallest c > lambda/mu with 1 - C(c, a) exp(-(c mu - lambda) t) >= target",
     latex=r"P(W_q \le t) = 1 - C(c,a)\,e^{-(c\mu-\lambda)t}", run=staffing, reference=_ref_staffing,
     wrong=lambda arrival_rate, service_rate, target_time, target_level: {"service_level": 1 - _erlang_c(arrival_rate, service_rate, int(arrival_rate / service_rate) + 1) * 0.5},
     rand=lambda rng: {"arrival_rate": _u(rng, 5, 400, 1), "service_rate": _u(rng, 2, 20, 2), "target_time": _u(rng, 0.005, 0.2, 3),
                       "target_level": rng.choice([0.8, 0.85, 0.9, 0.95])},
     triggers=["servers needed for a service level", "erlang c staffing", "call centre staffing", "call center staffing",
               "agents needed"], ask="servers needed for a service level",
     examples=[{"arrival_rate": 100, "service_rate": 12, "target_time": 0.0333, "target_level": 0.8},
               {"arrival_rate": 30, "service_rate": 10, "target_time": 0.05, "target_level": 0.9}],
     invalid=[{"arrival_rate": 100, "service_rate": 12, "target_time": 0.0333, "target_level": 1.8}])


def server_cost(arrival_rate, service_rate, server_cost_rate, waiting_cost_rate, max_servers=50):
    lam, mu = _pos("arrival_rate", arrival_rate), _pos("service_rate", service_rate)
    cs, cw = _nonneg("server_cost_rate", server_cost_rate), _nonneg("waiting_cost_rate", waiting_cost_rate)
    cmax = _int("max_servers", max_servers, 1, 500)
    best = None
    c = int(lam / mu) + 1
    while c <= cmax:
        C = _erlang_c(lam, mu, c)
        rho = lam / (c * mu)
        L = C * rho / (1 - rho) + lam / mu
        cost = c * cs + cw * L
        if best is None or cost < best[0] - 1e-12 * cost:
            best = (cost, c, L)
        c += 1
    _need(best is not None, "max_servers is too small for a stable queue")
    return _rd({"servers": best[1], "total_cost": best[0], "L": best[2]})


def _ref_server_cost(arrival_rate, service_rate, server_cost_rate, waiting_cost_rate, max_servers=50):
    from scipy.stats import poisson
    a = arrival_rate / service_rate
    rows = []
    for c in range(int(a) + 1, int(max_servers) + 1):
        B = float(poisson.pmf(c, a) / poisson.cdf(c, a))
        rho = a / c
        C = B / (1 - rho * (1 - B))
        L = C * rho / (1 - rho) + a
        rows.append((c * server_cost_rate + waiting_cost_rate * L, c, L))
    cost, c, L = min(rows)
    return {"servers": c, "total_cost": cost, "L": L}


_add("ie_queue_server_cost", name="Cost-optimal number of servers", topic=f"{_QUE} / staffing", level="intermediate",
     purpose="Number of servers of an M/M/c queue that minimises server cost plus the cost of customers waiting in "
             "the system per time unit.",
     inputs=[_LAM(), _MUI(),
             _i("server_cost_rate", "cost of one server per time unit", "server cost", "cost per server",
                "cost per server per hour"),
             _i("waiting_cost_rate", "cost of one customer in the system per time unit", "waiting cost",
                "cost of waiting", "waiting cost per customer"),
             _i("max_servers", "largest number of servers considered (default 50)", "maximum servers",
                type="integer", required=False)],
     outputs=[_o("servers", "cost-minimising number of servers"), _o("total_cost", "c C_s + C_w L at that number"),
              _o("L", "mean number in system at that number")],
     primary="total_cost", conditions=["arrival_rate, service_rate > 0", "costs >= 0", "ties go to fewer servers"],
     method="for each stable c: L from Erlang C; total cost c C_s + C_w L; the smallest wins",
     latex=r"TC(c) = c\,C_s + C_w\,L(c)", run=server_cost, reference=_ref_server_cost,
     wrong=lambda arrival_rate, service_rate, server_cost_rate, waiting_cost_rate, max_servers=50: {
         "total_cost": (int(arrival_rate / service_rate) + 1) * server_cost_rate},
     rand=lambda rng: {"arrival_rate": _u(rng, 2, 60, 2), "service_rate": _u(rng, 1, 15, 2),
                       "server_cost_rate": _u(rng, 5, 60, 2), "waiting_cost_rate": _u(rng, 5, 150, 2), "max_servers": 80},
     triggers=["cost optimal number of servers", "optimal number of servers", "minimum cost servers",
               "server cost and waiting cost"], ask="optimal number of servers",
     examples=[{"arrival_rate": 20, "service_rate": 6, "server_cost_rate": 25, "waiting_cost_rate": 40},
               {"arrival_rate": 8, "service_rate": 3, "server_cost_rate": 15, "waiting_cost_rate": 20}],
     invalid=[{"arrival_rate": 20, "service_rate": 6, "server_cost_rate": -25, "waiting_cost_rate": 40}])


def littles_law(arrival_rate=None, number_in_system=None, time_in_system=None):
    given = [v is not None for v in (arrival_rate, number_in_system, time_in_system)]
    _need(sum(given) == 2, "give exactly two of arrival rate, number in system and time in system")
    if arrival_rate is None:
        L, W = _nonneg("number_in_system", number_in_system), _pos("time_in_system", time_in_system)
        lam = L / W
    elif number_in_system is None:
        lam, W = _pos("arrival_rate", arrival_rate), _nonneg("time_in_system", time_in_system)
        L = lam * W
    else:
        lam, L = _pos("arrival_rate", arrival_rate), _nonneg("number_in_system", number_in_system)
        W = L / lam
    return _rd({"arrival_rate": lam, "number_in_system": L, "time_in_system": W})


def _ref_little(arrival_rate=None, number_in_system=None, time_in_system=None):
    import sympy as sp
    lam, L, W = sp.symbols("lam L W", positive=True)
    known = {k: sp.Rational(repr(float(v))) for k, v in ((lam, arrival_rate), (L, number_in_system), (W, time_in_system)) if v is not None}
    unknown = [s for s in (lam, L, W) if s not in known][0]
    val = float(sp.solve(sp.Eq(L, lam * W).subs(known), unknown)[0])
    vals = {**{k: float(v) for k, v in known.items()}, unknown: val}
    return {"arrival_rate": vals[lam], "number_in_system": vals[L], "time_in_system": vals[W]}


def _rand_little(rng):
    d = {"arrival_rate": _u(rng, 0.5, 100, 2), "number_in_system": _u(rng, 0.5, 200, 2), "time_in_system": _u(rng, 0.1, 50, 2)}
    del d[rng.choice(list(d))]
    return d


_add("ie_littles_law", name="Little's law", topic=f"{_QUE} / Little's law", level="beginner",
     purpose="L = lambda W: the average number in a system equals the arrival rate times the average time spent; give "
             "any two to get the third.",
     inputs=[_i("arrival_rate", "average arrival (throughput) rate", "lambda", "arrival rate", "throughput", required=False),
             _i("number_in_system", "average number in the system (WIP)", "l", "average number in system", "wip",
                "work in process", "inventory", required=False),
             _i("time_in_system", "average time in the system (flow time)", "w", "average time in system", "flow time",
                "cycle time", "lead time", required=False)],
     outputs=[_o("arrival_rate", "lambda"), _o("number_in_system", "L = lambda W"), _o("time_in_system", "W = L/lambda")],
     primary="number_in_system", conditions=["exactly two of the three given", "consistent time units", "steady state"],
     method="L = lambda W", latex=r"L = \lambda W", run=littles_law, reference=_ref_little,
     wrong=lambda arrival_rate=None, number_in_system=None, time_in_system=None: {
         "number_in_system": (arrival_rate / time_in_system) if number_in_system is None else number_in_system + 1,
         "time_in_system": (number_in_system * arrival_rate) if time_in_system is None else time_in_system,
         "arrival_rate": (number_in_system * time_in_system) if arrival_rate is None else arrival_rate},
     rand=_rand_little,
     triggers=["little's law", "littles law", "little law"], ask="Little's law",
     examples=[{"arrival_rate": 12, "time_in_system": 0.5}, {"number_in_system": 300, "time_in_system": 2.5}],
     invalid=[{"arrival_rate": 12, "time_in_system": 0.5, "number_in_system": 6}],
     weak="the reference re-solves the defining equation L = lambda W with SymPy")


# =====================================================================================================================
# SCHEDULING AND SEQUENCING
# =====================================================================================================================
_SCH = "scheduling"


def _metrics(order, p, due):
    """Single-machine measures of a job order (0-based indices), all jobs available at time 0."""
    t, comp = 0.0, []
    for j in order:
        t += p[j]
        comp.append(t)
    n = len(order)
    out = {"sequence": [j + 1 for j in order], "mean_flow_time": sum(comp) / n, "makespan": t,
           "average_jobs_in_system": sum(comp) / t, "utilization": t / sum(comp)}
    if due is not None:
        late = [comp[k] - due[order[k]] for k in range(n)]
        tard = [max(0.0, x) for x in late]
        out.update({"total_tardiness": sum(tard), "mean_tardiness": sum(tard) / n, "max_tardiness": max(tard),
                    "max_lateness": max(late), "tardy_jobs": sum(1 for x in tard if x > 0)})
    return out


def _jobs(processing_times, due_dates=None, n_max=500):
    p = _vec("processing_times", processing_times, 1, n_max, positive=True)
    d = None
    if due_dates is not None:
        d = _vec("due_dates", due_dates, 1, n_max, nonneg=True)
        _need(len(d) == len(p), "processing_times and due_dates must have the same number of values")
    return p, d


def _ref_metrics(seq, p, due):
    import numpy as np
    pp = np.asarray(p, float)[np.asarray(seq) - 1]
    comp = np.cumsum(pp)
    out = {"sequence": [int(s) for s in seq], "mean_flow_time": float(comp.mean()), "makespan": float(comp[-1]),
           "average_jobs_in_system": float(comp.sum() / comp[-1]), "utilization": float(comp[-1] / comp.sum())}
    if due is not None:
        late = comp - np.asarray(due, float)[np.asarray(seq) - 1]
        tard = np.maximum(late, 0)
        out.update({"total_tardiness": float(tard.sum()), "mean_tardiness": float(tard.mean()), "max_tardiness": float(tard.max()),
                    "max_lateness": float(late.max()), "tardy_jobs": int((tard > 0).sum())})
    return out


_PT_IN = lambda: _i("processing_times", "processing time of each job (jobs numbered 1, 2, ... in this order)",
                    "processing times", "job times", "durations", "times", type="vector")
_DD_IN = lambda req=True: _i("due_dates", "due date of each job (same order)", "due dates", "due", "deadlines",
                             type="vector", required=req)
_SEQ_OUT = [_o("sequence", "job numbers in processing order"), _o("mean_flow_time", "average completion (flow) time"),
            _o("makespan", "completion time of the last job"),
            _o("average_jobs_in_system", "sum of completion times / makespan"), _o("utilization", "sum of processing times / sum of completion times")]
_TARD_OUT = [_o("total_tardiness", "sum of tardiness"), _o("mean_tardiness", "average tardiness (job lateness)"),
             _o("max_tardiness", "largest tardiness"), _o("max_lateness", "largest lateness C - d"),
             _o("tardy_jobs", "number of late jobs")]


def spt_sequence(processing_times, due_dates=None):
    p, d = _jobs(processing_times, due_dates)
    order = sorted(range(len(p)), key=lambda j: (p[j], j))
    return _rd(_metrics(order, p, d))


def _ref_spt(processing_times, due_dates=None):
    import itertools
    from fractions import Fraction
    p = [Fraction(repr(float(v))) for v in processing_times]
    best, arg = None, None
    for perm in itertools.permutations(range(len(p))):    # exhaustive: least total flow time, first in lexicographic order
        t, tot = 0, 0
        for j in perm:
            t += p[j]
            tot += t
        if best is None or tot < best:
            best, arg = tot, perm
    return _ref_metrics([j + 1 for j in arg], processing_times, due_dates)


def _rand_jobs(rng, n_lo=3, n_hi=7, due=True, tight=1.0):
    n = rng.randint(n_lo, n_hi)
    p = [rng.randint(1, 20) for _ in range(n)]
    out = {"processing_times": p}
    if due:
        out["due_dates"] = [rng.randint(max(1, int(min(p))), max(2, int(sum(p) * tight))) for _ in range(n)]
    return out


_add("ie_spt_sequence", name="Shortest processing time (SPT) sequence", topic=f"{_SCH} / single machine",
     level="beginner",
     purpose="Order jobs on one machine by shortest processing time first (minimises mean flow time) and report the "
             "usual measures.",
     inputs=[_PT_IN(), _DD_IN(False)],
     outputs=_SEQ_OUT + _TARD_OUT, primary="mean_flow_time",
     conditions=["processing times > 0", "all jobs available at time 0", "ties go to the lower job number",
                 "tardiness measures only when due dates are given"],
     method="sort by processing time; completion C_k = sum of the first k times; flow = mean C",
     latex=r"p_{[1]} \le p_{[2]} \le \dots,\quad \bar F = \frac{1}{n}\sum_k C_{[k]}", run=spt_sequence, reference=_ref_spt,
     wrong=lambda processing_times, due_dates=None: _metrics(sorted(range(len(processing_times)), key=lambda j: (-processing_times[j], j)),
                                                            [float(v) for v in processing_times], None),
     rand=lambda rng: _rand_jobs(rng, due=rng.random() < 0.5),
     triggers=["shortest processing time", "spt sequence", "spt rule", "spt"], ask="SPT sequence",
     examples=[{"processing_times": [6, 2, 8, 3, 9]}, {"processing_times": [4, 7, 2, 5], "due_dates": [10, 15, 5, 12]}],
     invalid=[{"processing_times": [6, -2, 8]}])


def edd_sequence(processing_times, due_dates):
    p, d = _jobs(processing_times, due_dates)
    order = sorted(range(len(p)), key=lambda j: (d[j], j))
    return _rd(_metrics(order, p, d))


def _ref_edd(processing_times, due_dates):
    import itertools
    import pandas as pd
    df = pd.DataFrame({"p": processing_times, "d": due_dates, "job": range(1, len(due_dates) + 1)})
    seq = list(df.sort_values(["d", "job"], kind="stable")["job"])
    out = _ref_metrics(seq, processing_times, due_dates)
    best = None                                            # EDD's maximum lateness is the least possible
    for perm in itertools.permutations(range(len(processing_times))):
        t, worst = 0, None
        for j in perm:
            t += processing_times[j]
            worst = t - due_dates[j] if worst is None else max(worst, t - due_dates[j])
        best = worst if best is None else min(best, worst)
    if abs(best - out["max_lateness"]) > 1e-9 * max(1.0, abs(best)):
        out["max_lateness"] = float("nan")
    return out


_add("ie_edd_sequence", name="Earliest due date (EDD) sequence", topic=f"{_SCH} / single machine", level="beginner",
     purpose="Order jobs on one machine by earliest due date (minimises the maximum lateness) and report tardiness "
             "measures.",
     inputs=[_PT_IN(), _DD_IN()],
     outputs=_SEQ_OUT + _TARD_OUT, primary="max_lateness",
     conditions=["processing times > 0", "due dates >= 0", "ties go to the lower job number"],
     method="sort by due date; lateness C - d; tardiness max(0, C - d)",
     latex=r"d_{[1]} \le d_{[2]} \le \dots,\quad L_{max} = \max_k (C_{[k]} - d_{[k]})", run=edd_sequence, reference=_ref_edd,
     wrong=lambda processing_times, due_dates: {"max_lateness": max(t - d for t, d in zip(processing_times, due_dates)) - 1.0},
     rand=lambda rng: _rand_jobs(rng, tight=0.7),
     triggers=["earliest due date", "edd sequence", "edd rule", "edd"], ask="EDD sequence",
     examples=[{"processing_times": [4, 7, 2, 5], "due_dates": [10, 15, 5, 12]},
               {"processing_times": [3, 6, 2, 8, 4], "due_dates": [8, 20, 5, 18, 12]}],
     invalid=[{"processing_times": [4, 7, 2], "due_dates": [10, 15]}])


_RULES = ["fcfs", "spt", "lpt", "edd", "cr"]


def dispatch_rule(processing_times, due_dates, rule):
    p, d = _jobs(processing_times, due_dates)
    _need(rule in _RULES, "rule is one of fcfs, spt, lpt, edd, cr")
    n = len(p)
    if rule == "fcfs":
        order = list(range(n))
    elif rule == "spt":
        order = sorted(range(n), key=lambda j: (p[j], j))
    elif rule == "lpt":
        order = sorted(range(n), key=lambda j: (-p[j], j))
    elif rule == "edd":
        order = sorted(range(n), key=lambda j: (d[j], j))
    else:
        order = sorted(range(n), key=lambda j: (d[j] / p[j], j))
    return _rd(_metrics(order, p, d))


def _ref_dispatch(processing_times, due_dates, rule):
    import pandas as pd
    df = pd.DataFrame({"p": processing_times, "d": due_dates, "job": range(1, len(due_dates) + 1)})
    df["cr"] = df["d"] / df["p"]
    df["neg_p"] = -df["p"]
    key = {"fcfs": ["job"], "spt": ["p", "job"], "lpt": ["neg_p", "job"], "edd": ["d", "job"], "cr": ["cr", "job"]}[rule]
    seq = list(df.sort_values(key, kind="stable")["job"])
    return _ref_metrics(seq, processing_times, due_dates)


_add("ie_dispatching_rule", name="Sequencing by a dispatching rule (FCFS, SPT, LPT, EDD, CR)",
     topic=f"{_SCH} / single machine", level="intermediate",
     purpose="Sequence jobs at one work centre by a priority rule and compare average completion time, utilisation, "
             "average number of jobs and tardiness.",
     inputs=[_PT_IN(), _DD_IN(),
             _i("rule", "fcfs (given order), spt, lpt (longest first), edd or cr (critical ratio due date / processing "
                "time at time 0, smallest first)", "priority rule", "dispatching rule", "sequencing rule",
                type="choice", choices=_RULES,
                syn={"fifo": "fcfs", "first-come": "fcfs", "longest": "lpt", "shortest": "spt", "critical": "cr",
                     "due": "edd"})],
     outputs=_SEQ_OUT + _TARD_OUT, primary="mean_flow_time",
     conditions=["processing times > 0", "due dates >= 0", "ties go to the lower job number", "jobs available at time 0"],
     method="order the jobs by the rule; completion times are running sums; tardiness max(0, C - d)",
     latex=r"\bar F = \frac{\sum C_j}{n},\quad \text{utilization} = \frac{\sum p_j}{\sum C_j},\quad \bar N = \frac{\sum C_j}{C_{max}}",
     run=dispatch_rule, reference=_ref_dispatch,
     wrong=lambda processing_times, due_dates, rule: {"mean_flow_time": sum(processing_times) / len(processing_times),
                                                      "utilization": 1.0},
     rand=lambda rng: dict(_rand_jobs(rng, 3, 10), rule=rng.choice(_RULES)),
     triggers=["dispatching rule", "sequencing rule", "priority rule", "sequence the jobs by"],
     ask="sequence by dispatching rule",
     examples=[{"processing_times": [6, 2, 8, 3, 9], "due_dates": [8, 6, 18, 15, 23], "rule": "fcfs"},
               {"processing_times": [6, 2, 8, 3, 9], "due_dates": [8, 6, 18, 15, 23], "rule": "cr"}],
     invalid=[{"processing_times": [6, 2, 8], "due_dates": [8, 6, 18], "rule": "random"}])


def _johnson_order(a, b):
    first = sorted([j for j in range(len(a)) if a[j] <= b[j]], key=lambda j: (a[j], j))
    last = sorted([j for j in range(len(a)) if a[j] > b[j]], key=lambda j: (-b[j], j))
    return first + last


def _two_machine_span(order, a, b):
    t1, t2 = 0.0, 0.0
    for j in order:
        t1 += a[j]
        t2 = max(t2, t1) + b[j]
    return t2


def johnson_two(machine1_times, machine2_times):
    a = _vec("machine1_times", machine1_times, 1, 500, nonneg=True)
    b = _vec("machine2_times", machine2_times, 1, 500, nonneg=True)
    _need(len(a) == len(b), "both machines need a time for every job")
    order = _johnson_order(a, b)
    span = _two_machine_span(order, a, b)
    return _rd({"sequence": [j + 1 for j in order], "makespan": span, "idle_time_machine2": span - sum(b)})


def _ref_johnson2(machine1_times, machine2_times):
    import itertools
    import pandas as pd
    df = pd.DataFrame({"a": machine1_times, "b": machine2_times, "job": range(1, len(machine1_times) + 1)})
    df["grp"] = (df["a"] > df["b"]).astype(int)
    df["key"] = df["a"].where(df["grp"] == 0, -df["b"])
    seq = list(df.sort_values(["grp", "key", "job"], kind="stable")["job"])
    best = None                                            # exhaustive search for the least makespan
    for perm in itertools.permutations(range(len(machine1_times))):
        t1 = t2 = 0
        for j in perm:
            t1 += machine1_times[j]
            t2 = max(t2, t1) + machine2_times[j]
        best = t2 if best is None else min(best, t2)
    return {"sequence": seq, "makespan": float(best), "idle_time_machine2": float(best - sum(machine2_times))}


_add("ie_johnson_two_machine", name="Johnson's rule (two machines)", topic=f"{_SCH} / flow shop", level="intermediate",
     purpose="Sequence jobs through two machines in the same order so that the makespan is minimal (Johnson's rule).",
     inputs=[_i("machine1_times", "time of each job on machine 1 (first)", "machine 1 times", "times on machine 1",
                "first machine times", "m1", type="vector"),
             _i("machine2_times", "time of each job on machine 2 (second)", "machine 2 times", "times on machine 2",
                "second machine times", "m2", type="vector")],
     outputs=[_o("sequence", "job order"), _o("makespan", "time all jobs finish on machine 2"),
              _o("idle_time_machine2", "idle time of machine 2 (makespan - its work)")],
     primary="makespan",
     conditions=["times >= 0", "jobs with a1 <= a2 go first by increasing a1, the others last by decreasing a2; ties "
                 "by the lower job number"],
     method="Johnson: front set (a1 <= a2) by a1 ascending, back set by a2 descending",
     latex=r"\min(a_{1i}, a_{2j}) \le \min(a_{1j}, a_{2i}) \Rightarrow i \text{ before } j",
     run=johnson_two, reference=_ref_johnson2,
     wrong=lambda machine1_times, machine2_times: {"makespan": sum(machine1_times) + sum(machine2_times)},
     rand=lambda rng: (lambda n: {"machine1_times": [rng.randint(1, 15) for _ in range(n)],
                                  "machine2_times": [rng.randint(1, 15) for _ in range(n)]})(rng.randint(3, 7)),
     triggers=["johnson's rule", "johnsons rule", "johnson rule", "two machine flow shop", "two-machine flow shop"],
     ask="Johnson's rule",
     examples=[{"machine1_times": [5, 2, 1, 7, 6], "machine2_times": [2, 6, 2, 5, 3]},
               {"machine1_times": [3, 6, 4, 8], "machine2_times": [7, 2, 5, 4]}],
     invalid=[{"machine1_times": [5, 2, 1], "machine2_times": [2, 6]}])


def _three_machine_span(order, a, b, c):
    t1 = t2 = t3 = 0.0
    for j in order:
        t1 += a[j]
        t2 = max(t2, t1) + b[j]
        t3 = max(t3, t2) + c[j]
    return t3


def johnson_three(machine1_times, machine2_times, machine3_times):
    a = _vec("machine1_times", machine1_times, 1, 500, nonneg=True)
    b = _vec("machine2_times", machine2_times, 1, 500, nonneg=True)
    c = _vec("machine3_times", machine3_times, 1, 500, nonneg=True)
    _need(len(a) == len(b) == len(c), "every machine needs a time for every job")
    _need(min(a) >= max(b) or min(c) >= max(b),
          "Johnson's extension needs min(machine 1) >= max(machine 2) or min(machine 3) >= max(machine 2)")
    order = _johnson_order([a[j] + b[j] for j in range(len(a))], [b[j] + c[j] for j in range(len(a))])
    return _rd({"sequence": [j + 1 for j in order], "makespan": _three_machine_span(order, a, b, c)})


def _ref_johnson3(machine1_times, machine2_times, machine3_times):
    import itertools
    a, b, c = machine1_times, machine2_times, machine3_times
    best = None
    for perm in itertools.permutations(range(len(a))):
        t1 = t2 = t3 = 0
        for j in perm:
            t1 += a[j]
            t2 = max(t2, t1) + b[j]
            t3 = max(t3, t2) + c[j]
        best = t3 if best is None else min(best, t3)
    return {"makespan": float(best)}


def _rand_j3(rng):
    n = rng.randint(3, 7)
    b = [rng.randint(1, 6) for _ in range(n)]
    hi = max(b)
    a = [rng.randint(hi, hi + 10) for _ in range(n)]
    c = [rng.randint(1, hi + 10) for _ in range(n)]
    if rng.random() < 0.5:
        a, c = [rng.randint(1, hi + 10) for _ in range(n)], [rng.randint(hi, hi + 10) for _ in range(n)]
    return {"machine1_times": a, "machine2_times": b, "machine3_times": c}


_add("ie_johnson_three_machine", name="Johnson's rule extended to three machines", topic=f"{_SCH} / flow shop",
     level="advanced",
     purpose="Optimal job order through three machines when machine 2 is dominated (Johnson's extension with pseudo "
             "times a1 + a2 and a2 + a3).",
     inputs=[_i("machine1_times", "time of each job on machine 1", "machine 1 times", "times on machine 1", type="vector"),
             _i("machine2_times", "time of each job on machine 2", "machine 2 times", "times on machine 2", type="vector"),
             _i("machine3_times", "time of each job on machine 3", "machine 3 times", "times on machine 3", type="vector")],
     outputs=[_o("sequence", "job order"), _o("makespan", "time all jobs finish on machine 3")],
     primary="makespan",
     conditions=["min(machine 1) >= max(machine 2) or min(machine 3) >= max(machine 2) (else refused)",
                 "ties by the lower job number"],
     method="Johnson's rule on pseudo machines (a1 + a2, a2 + a3); makespan by the three-machine recursion",
     latex=r"a_i' = a_{1i} + a_{2i},\quad a_i'' = a_{2i} + a_{3i}", run=johnson_three, reference=_ref_johnson3,
     wrong=lambda machine1_times, machine2_times, machine3_times: {"makespan": sum(machine1_times) + sum(machine3_times) - 0.5},
     rand=_rand_j3,
     triggers=["johnson's rule for three machines", "three machine flow shop", "three-machine flow shop",
               "johnson three machines", "johnson's rule three machines"], ask="Johnson's rule for three machines",
     examples=[{"machine1_times": [8, 10, 6, 7, 11], "machine2_times": [5, 6, 2, 3, 4], "machine3_times": [4, 9, 8, 6, 5]},
               {"machine1_times": [3, 8, 7, 4], "machine2_times": [4, 3, 2, 5], "machine3_times": [6, 7, 5, 11]}],
     invalid=[{"machine1_times": [1, 2], "machine2_times": [5, 6], "machine3_times": [1, 2]}])


def moore_hodgson(processing_times, due_dates):
    p, d = _jobs(processing_times, due_dates)
    order = sorted(range(len(p)), key=lambda j: (d[j], j))
    on, t, late = [], 0.0, []
    for j in order:
        on.append(j)
        t += p[j]
        if t > d[j]:
            k = on[0]
            for x in on:
                if p[x] > p[k] or (p[x] == p[k] and x < k):
                    k = x
            on.remove(k)
            t -= p[k]
            late.append(k)
    seq = [j + 1 for j in on] + sorted([j + 1 for j in late])
    return _rd({"number_late": len(late), "late_jobs": sorted([j + 1 for j in late]), "on_time_jobs": [j + 1 for j in on],
                "sequence": seq})


def _ref_moore(processing_times, due_dates):
    import itertools
    n = len(processing_times)
    for size in range(n, -1, -1):                          # the largest set of jobs that can all be on time
        for sub in itertools.combinations(range(n), size):
            t, ok = 0, True
            for j in sorted(sub, key=lambda j: (due_dates[j], j)):
                t += processing_times[j]
                if t > due_dates[j]:
                    ok = False
                    break
            if ok:
                return {"number_late": n - size}


def _check_moore(processing_times, due_dates):
    out = moore_hodgson(processing_times, due_dates)
    return out


_add("ie_moore_hodgson", name="Minimise the number of late jobs (Moore-Hodgson)", topic=f"{_SCH} / single machine",
     level="expert",
     purpose="Sequence on one machine that minimises the number of late jobs: on-time jobs in EDD order, then the late "
             "ones (Moore-Hodgson algorithm).",
     inputs=[_PT_IN(), _DD_IN()],
     outputs=[_o("number_late", "least possible number of late jobs"), _o("late_jobs", "the jobs scheduled late"),
              _o("on_time_jobs", "the on-time jobs in processing (EDD) order"), _o("sequence", "full sequence")],
     primary="number_late",
     conditions=["processing times > 0", "due dates >= 0", "when a job is late, the longest scheduled job is removed "
                 "(ties: the lower job number)"],
     method="add jobs in EDD order; whenever the current job finishes late, drop the longest job scheduled so far",
     latex=r"\min \sum_j U_j,\quad U_j = [C_j > d_j]", run=moore_hodgson, reference=_ref_moore,
     wrong=lambda processing_times, due_dates: {"number_late": _metrics(sorted(range(len(processing_times)), key=lambda j: (due_dates[j], j)),
                                                                        [float(v) for v in processing_times], [float(v) for v in due_dates])["tardy_jobs"]},
     rand=lambda rng: _rand_jobs(rng, 5, 10, tight=0.45),
     triggers=["moore hodgson", "moore-hodgson", "minimise the number of late jobs", "minimize the number of late jobs",
               "minimum number of tardy jobs", "minimise the number of tardy jobs"], ask="Moore-Hodgson",
     examples=[{"processing_times": [4, 3, 2, 5, 6], "due_dates": [5, 6, 8, 10, 12]},
               {"processing_times": [2, 6, 3, 4, 5, 1], "due_dates": [3, 8, 9, 10, 12, 14]}],
     invalid=[{"processing_times": [4, 3, 0], "due_dates": [5, 6, 8]}])


def flow_shop_makespan(times):
    m = _mat("times", times)
    for row in m:
        for v in row:
            _need(v >= 0, "processing times must not be negative")
    n, k = len(m), len(m[0])
    prev = [0.0] * k
    last = []
    for i in range(n):
        cur = []
        for j in range(k):
            ready = max(prev[j], cur[j - 1] if j > 0 else 0.0)
            cur.append(ready + m[i][j])
        last.append(cur[k - 1])
        prev = cur
    return _rd({"makespan": last[-1], "completion_times": last,
                "idle_time_last_machine": last[-1] - sum(m[i][k - 1] for i in range(n))})


def _ref_flow_shop(times):
    import itertools
    n, k = len(times), len(times[0])
    def longest(i):                                        # longest monotone path (1,1) -> (i, k) through the grid
        best = 0.0
        for downs in itertools.combinations(range(i + k - 2), i - 1):
            r = c = 0
            tot = times[0][0]
            for step in range(i + k - 2):
                if step in downs:
                    r += 1
                else:
                    c += 1
                tot += times[r][c]
            best = max(best, tot)
        return best
    last = [longest(i) for i in range(1, n + 1)]
    return {"makespan": last[-1], "completion_times": last}


_add("ie_flow_shop_makespan", name="Permutation flow shop makespan", topic=f"{_SCH} / flow shop", level="advanced",
     purpose="Makespan and completion times of jobs processed in a given order through machines in series (each row "
             "a job in processing order, each column a machine).",
     inputs=[_i("times", "matrix: row = job (in processing order), column = machine (in route order)",
                "processing time matrix", "time matrix", "job times", type="matrix")],
     outputs=[_o("makespan", "completion time of the last job on the last machine"),
              _o("completion_times", "completion time of each job on the last machine"),
              _o("idle_time_last_machine", "idle time of the last machine")],
     primary="makespan", conditions=["times >= 0", "the same job order on every machine, no passing"],
     method="C(i, j) = max(C(i-1, j), C(i, j-1)) + t(i, j)",
     latex=r"C_{i,j} = \max(C_{i-1,j},\,C_{i,j-1}) + t_{i,j}", run=flow_shop_makespan, reference=_ref_flow_shop,
     wrong=lambda times: {"makespan": max(sum(r) for r in times) + sum(r[-1] for r in times[1:])
                          + (0.5 if len(times) == 1 else 0)},
     rand=lambda rng: (lambda n, k: {"times": [[rng.randint(1, 12) for _ in range(k)] for _ in range(n)]})(rng.randint(2, 7), rng.randint(2, 4)),
     triggers=["flow shop makespan", "flow-shop makespan", "permutation flow shop", "makespan of the flow shop"],
     ask="flow shop makespan",
     examples=[{"times": [[5, 3, 6], [2, 7, 4], [4, 2, 5], [6, 4, 3]]}, {"times": [[3, 6], [5, 2], [1, 2]]}],
     invalid=[{"times": [[5, -3], [2, 7]]}])


# =====================================================================================================================
# FORECASTING AND FORECAST ACCURACY
# =====================================================================================================================
_FC = "forecasting"
_DATA = lambda: _i("data", "observations in time order (oldest first)", "demand", "history", "observations", "series",
                   "actual demand", type="vector")


def moving_average(data, periods):
    x = _vec("data", data, 1, 10000)
    n = _int("periods", periods, 1)
    _need(n <= len(x), "periods must not exceed the number of observations")
    fc = []
    for t in range(n, len(x) + 1):
        fc.append(sum(x[t - n:t]) / n)
    return _rd({"forecast": fc[-1], "forecasts": fc})


def _ref_ma(data, periods):
    import pandas as pd
    r = pd.Series(data, dtype=float).rolling(int(periods)).mean().dropna()
    return {"forecast": float(r.iloc[-1]), "forecasts": [float(v) for v in r]}


_add("ie_moving_average_forecast", name="Moving average forecast", topic=f"{_FC} / time series methods",
     level="beginner",
     purpose="Next-period forecast as the average of the last n observations (and the forecast each period would "
             "have had).",
     inputs=[_DATA(), _i("periods", "number of periods averaged n", "n", "window", "number of periods", type="integer")],
     outputs=[_o("forecast", "forecast for the next period"),
              _o("forecasts", "moving averages: forecast for periods n+1, ..., (last period + 1)")],
     primary="forecast", conditions=["1 <= periods <= number of observations"],
     method="F_(t+1) = (A_t + ... + A_(t-n+1)) / n", latex=r"F_{t+1} = \frac{1}{n}\sum_{i=0}^{n-1} A_{t-i}",
     run=moving_average, reference=_ref_ma,
     wrong=lambda data, periods: {"forecast": sum(data[-int(periods):]) / (int(periods) + 1)},
     rand=lambda rng: (lambda x: {"data": x, "periods": rng.randint(1, len(x))})([_u(rng, 20, 500, 1) for _ in range(rng.randint(3, 15))]),
     triggers=["moving average forecast", "simple moving average", "moving average"], ask="moving average forecast",
     examples=[{"data": [120, 135, 128, 142, 150, 147], "periods": 3}, {"data": [40, 44, 39, 47], "periods": 2}],
     invalid=[{"data": [40, 44, 39], "periods": 5}])


def weighted_moving_average(data, weights):
    x = _vec("data", data, 1, 10000)
    w = _vec("weights", weights, 1, 1000, nonneg=True)
    _need(len(w) <= len(x), "there must be at least as many observations as weights")
    tw = sum(w)
    _need(tw > 0, "the weights must not all be 0")
    k = len(w)
    return _rd({"forecast": sum(w[i] * x[len(x) - k + i] for i in range(k)) / tw, "weight_sum": tw})


def _ref_wma(data, weights):
    import numpy as np
    k = len(weights)
    return {"forecast": float(np.average(np.asarray(data[-k:], float), weights=np.asarray(weights, float)))}


_add("ie_weighted_moving_average", name="Weighted moving average forecast", topic=f"{_FC} / time series methods",
     level="beginner",
     purpose="Next-period forecast as a weighted average of the most recent observations (weights listed oldest to "
             "newest, divided by their sum).",
     inputs=[_DATA(), _i("weights", "weights for the last k observations, oldest first", "weights", "weighting", type="vector")],
     outputs=[_o("forecast", "sum w_i A_i / sum w_i"), _o("weight_sum", "sum of the weights")],
     primary="forecast", conditions=["weights >= 0, not all 0", "at least as many observations as weights",
                                     "the last weight applies to the most recent observation"],
     method="F = sum w_i A_i / sum w_i over the last k observations",
     latex=r"F_{t+1} = \frac{\sum_{i=1}^{k} w_i A_{t-k+i}}{\sum w_i}", run=weighted_moving_average, reference=_ref_wma,
     wrong=lambda data, weights: {"forecast": sum(w * a for w, a in zip(weights, reversed(data[-len(weights):]))) / sum(weights)},
     rand=lambda rng: (lambda k: {"data": [_u(rng, 20, 500, 1) for _ in range(rng.randint(k, 15))],
                                  "weights": sorted(rng.sample(range(1, 10), k))})(rng.randint(2, 5)),
     triggers=["weighted moving average forecast", "weighted moving average"], ask="weighted moving average forecast",
     examples=[{"data": [100, 110, 105, 120], "weights": [1, 2, 3]}, {"data": [52, 48, 55, 60, 58], "weights": [0.2, 0.3, 0.5]}],
     invalid=[{"data": [100, 110], "weights": [1, 2, 3]}])


def exp_smoothing(data, alpha, initial_forecast=None):
    x = _vec("data", data, 1, 10000)
    a = _prob("alpha", alpha, 0.0, 1.0 + 1e-12)
    f = x[0] if initial_forecast is None else _fin("initial_forecast", initial_forecast)
    fc = [f]
    for v in x:
        f = f + a * (v - f)
        fc.append(f)
    return _rd({"forecast": fc[-1], "forecasts": fc})


def _ref_ses(data, alpha, initial_forecast=None):
    import numpy as np
    from statsmodels.tsa.holtwinters import SimpleExpSmoothing
    l0 = data[0] if initial_forecast is None else initial_forecast
    fit = SimpleExpSmoothing(np.asarray(data, float), initialization_method="known", initial_level=l0).fit(
        smoothing_level=alpha, optimized=False)
    return {"forecast": float(fit.forecast(1)[0]), "forecasts": [float(l0)] + [float(v) for v in fit.level]}


_add("ie_exponential_smoothing", name="Simple exponential smoothing forecast", topic=f"{_FC} / time series methods",
     level="intermediate",
     purpose="Exponentially smoothed forecasts F_(t+1) = F_t + alpha (A_t - F_t), starting from a given first forecast "
             "(default: the first observation).",
     inputs=[_DATA(), _i("alpha", "smoothing constant (0 < alpha <= 1)", "smoothing constant", "a"),
             _i("initial_forecast", "forecast for period 1 (default: the first observation)", "first forecast",
                "starting forecast", "f1", required=False)],
     outputs=[_o("forecast", "forecast for the next period"), _o("forecasts", "forecasts for periods 1, ..., n + 1")],
     primary="forecast", conditions=["0 < alpha <= 1"],
     method="F_1 given (or A_1); F_(t+1) = alpha A_t + (1 - alpha) F_t",
     latex=r"F_{t+1} = F_t + \alpha\,(A_t - F_t)", run=exp_smoothing, reference=_ref_ses,
     wrong=lambda data, alpha, initial_forecast=None: {"forecast": (lambda f: [f := f + (1 - alpha) * (v - f) for v in data][-1])(
         data[0] if initial_forecast is None else initial_forecast)},
     rand=lambda rng: _drop_none({"data": [_u(rng, 20, 500, 1) for _ in range(rng.randint(3, 15))], "alpha": _u(rng, 0.05, 0.6, 2),
                                  "initial_forecast": rng.choice([None, _u(rng, 20, 500, 1)])}),
     triggers=["exponential smoothing", "simple exponential smoothing", "exponentially smoothed forecast"],
     ask="exponential smoothing forecast",
     examples=[{"data": [142, 153, 136, 150], "alpha": 0.2, "initial_forecast": 140},
               {"data": [20, 24, 22, 27, 25], "alpha": 0.4}],
     invalid=[{"data": [142, 153], "alpha": 1.4}])


def holt(data, alpha, beta, initial_level, initial_trend, horizon=1):
    x = _vec("data", data, 1, 10000)
    a, b = _prob("alpha", alpha, 0.0, 1.0 + 1e-12), _prob("beta", beta, 0.0, 1.0 + 1e-12)
    lv, tr = _fin("initial_level", initial_level), _fin("initial_trend", initial_trend)
    h = _int("horizon", horizon, 1, 1000)
    for v in x:
        prev = lv
        lv = a * v + (1 - a) * (lv + tr)
        tr = b * (lv - prev) + (1 - b) * tr
    return _rd({"forecast": lv + h * tr, "level": lv, "trend": tr})


def _ref_holt(data, alpha, beta, initial_level, initial_trend, horizon=1):
    import numpy as np
    from statsmodels.tsa.holtwinters import Holt
    fit = Holt(np.asarray(data, float), initialization_method="known", initial_level=initial_level,
               initial_trend=initial_trend).fit(smoothing_level=alpha, smoothing_trend=beta, optimized=False)
    return {"forecast": float(fit.forecast(int(horizon))[-1]), "level": float(fit.level[-1]), "trend": float(fit.trend[-1])}


_add("ie_holt_trend", name="Holt's linear trend (double exponential smoothing)", topic=f"{_FC} / time series methods",
     level="advanced",
     purpose="Trend-adjusted exponential smoothing: level and trend updated each period; the forecast h periods ahead "
             "is level + h x trend.",
     inputs=[_DATA(), _i("alpha", "level smoothing constant", "level smoothing", "a"),
             _i("beta", "trend smoothing constant", "trend smoothing", "b"),
             _i("initial_level", "level before the first observation", "starting level", "level", "l0"),
             _i("initial_trend", "trend before the first observation", "starting trend", "trend", "b0"),
             _i("horizon", "periods ahead (default 1)", "h", "periods ahead", "steps ahead", type="integer", required=False)],
     outputs=[_o("forecast", "level + horizon x trend"), _o("level", "final level"), _o("trend", "final trend")],
     primary="forecast", conditions=["0 < alpha, beta <= 1", "1 <= horizon <= 1000"],
     method="L_t = alpha A_t + (1 - alpha)(L_(t-1) + T_(t-1)); T_t = beta (L_t - L_(t-1)) + (1 - beta) T_(t-1); "
            "F_(t+h) = L_t + h T_t",
     latex=r"L_t = \alpha A_t + (1-\alpha)(L_{t-1}+T_{t-1}),\quad T_t = \beta(L_t - L_{t-1}) + (1-\beta)T_{t-1}",
     run=holt, reference=_ref_holt,
     wrong=lambda data, alpha, beta, initial_level, initial_trend, horizon=1: (
         lambda r: {"forecast": r["level"] + r["trend"]})(holt(data, alpha, beta, initial_level, initial_trend, 1)) if int(horizon) > 1
     else {"forecast": holt(data, alpha, beta, initial_level, initial_trend, 1)["level"]},
     rand=lambda rng: (lambda x: {"data": x, "alpha": _u(rng, 0.1, 0.7, 2), "beta": _u(rng, 0.05, 0.5, 2),
                                  "initial_level": round(x[0] + rng.uniform(-10, 10), 1), "initial_trend": round(rng.uniform(-5, 10), 2),
                                  "horizon": rng.randint(1, 6)})([round(100 + 4 * t + rng.gauss(0, 6), 1) for t in range(rng.randint(4, 14))]),
     triggers=["holt", "holt's method", "holt's linear trend", "double exponential smoothing", "trend adjusted exponential smoothing",
               "trend-adjusted exponential smoothing"], ask="Holt's linear trend forecast",
     examples=[{"data": [12, 17, 20, 19, 24, 27], "alpha": 0.2, "beta": 0.4, "initial_level": 11, "initial_trend": 2},
               {"data": [100, 108, 115, 118, 126], "alpha": 0.5, "beta": 0.3, "initial_level": 95, "initial_trend": 5, "horizon": 3}],
     invalid=[{"data": [12, 17], "alpha": 0.2, "beta": 1.4, "initial_level": 11, "initial_trend": 2}])


_ACT = lambda: _i("actual", "actual values", "actual", "actuals", "observed", "actual demand", type="vector")
_FCS = lambda: _i("forecast", "forecast values (same periods)", "forecasts", "forecasted", "predicted", type="vector")


def _pair(actual, forecast):
    a = _vec("actual", actual, 1, 100000)
    f = _vec("forecast", forecast, 1, 100000)
    _need(len(a) == len(f), "actual and forecast must have the same number of values")
    return a, f


def forecast_errors(actual, forecast):
    a, f = _pair(actual, forecast)
    n = len(a)
    e = [a[i] - f[i] for i in range(n)]
    mad = sum(abs(v) for v in e) / n
    mse = sum(v * v for v in e) / n
    out = {"mad": mad, "mse": mse, "rmse": math.sqrt(mse), "bias": sum(e) / n, "running_sum_of_errors": sum(e)}
    if all(v != 0 for v in a):
        out["mape"] = 100 * sum(abs(e[i] / a[i]) for i in range(n)) / n
    if mad > 0:
        out["tracking_signal"] = sum(e) / mad
    return _rd(out)


def _ref_errors(actual, forecast):
    import numpy as np
    from statsmodels.tools import eval_measures as em
    a, f = np.asarray(actual, float), np.asarray(forecast, float)
    out = {"mad": float(em.meanabs(a, f)), "mse": float(em.mse(a, f)), "rmse": float(em.rmse(a, f)),
           "bias": float(np.mean(a - f))}
    if np.all(a != 0):
        out["mape"] = float(100 * np.mean(np.abs((a - f) / a)))
    if out["mad"] > 0:
        out["tracking_signal"] = float(np.sum(a - f)) / out["mad"]
    return out


def _rand_af(rng):
    n = rng.randint(3, 15)
    a = [_u(rng, 20, 500, 1) for _ in range(n)]
    return {"actual": a, "forecast": [round(v + rng.gauss(0, 20) + 5, 1) for v in a]}


_add("ie_forecast_errors", name="Forecast accuracy: MAD, MSE, RMSE, MAPE, bias, tracking signal",
     topic=f"{_FC} / forecast accuracy", level="beginner",
     purpose="Standard accuracy measures of a forecast against actual values (error = actual - forecast).",
     inputs=[_ACT(), _FCS()],
     outputs=[_o("mad", "mean absolute deviation"), _o("mse", "mean squared error"), _o("rmse", "root mean squared error"),
              _o("mape", "mean absolute percentage error, in percent (when no actual is 0)"),
              _o("bias", "mean error (positive: forecasts too low)"), _o("running_sum_of_errors", "sum of errors"),
              _o("tracking_signal", "running sum of errors / MAD")],
     primary="mad", conditions=["equal lengths", "error = actual - forecast", "MAPE only if no actual value is 0"],
     method="e_t = A_t - F_t; MAD = mean |e|; MSE = mean e^2; MAPE = 100 mean |e/A|; TS = sum e / MAD",
     latex=r"MAD = \frac{\sum|e_t|}{n},\ MSE = \frac{\sum e_t^2}{n},\ MAPE = \frac{100}{n}\sum\left|\frac{e_t}{A_t}\right|,\ TS = \frac{\sum e_t}{MAD}",
     run=forecast_errors, reference=_ref_errors,
     wrong=lambda actual, forecast: {"mad": abs(sum(a - f for a, f in zip(actual, forecast))) / len(actual),
                                     "mse": sum((a - f) ** 2 for a, f in zip(actual, forecast)) / (len(actual) - 1 if len(actual) > 1 else 0.5)},
     rand=_rand_af,
     triggers=["forecast accuracy", "forecast errors", "forecast error measures", "mad mse mape", "mean absolute deviation",
               "mean absolute percentage error", "mape", "mse"], ask="forecast accuracy",
     examples=[{"actual": [100, 110, 120, 130], "forecast": [105, 108, 125, 126]},
               {"actual": [52, 48, 60, 55, 62], "forecast": [50, 50, 54, 58, 58]}],
     invalid=[{"actual": [100, 110, 120], "forecast": [105, 108]}])


def tracking_signal(actual, forecast, limit=4.0):
    a, f = _pair(actual, forecast)
    lim = _pos("limit", limit)
    rs, sa, ts, first = 0.0, 0.0, [], 0
    for i in range(len(a)):
        e = a[i] - f[i]
        rs += e
        sa += abs(e)
        mad = sa / (i + 1)
        _need(mad > 0 or rs == 0, "the tracking signal is undefined")
        v = rs / mad if mad > 0 else 0.0
        ts.append(v)
        if first == 0 and abs(v) > lim:
            first = i + 1
    return _rd({"final_signal": ts[-1], "signals": ts, "first_out_of_control": first})


def _ref_tracking(actual, forecast, limit=4.0):
    import pandas as pd
    e = pd.Series(actual, dtype=float) - pd.Series(forecast, dtype=float)
    mad = e.abs().expanding().mean()
    ts = (e.cumsum() / mad).fillna(0.0)
    over = [i + 1 for i, v in enumerate(ts) if abs(v) > limit]
    return {"final_signal": float(ts.iloc[-1]), "signals": [float(v) for v in ts], "first_out_of_control": over[0] if over else 0}


def _rand_ts(rng):
    d = _rand_af(rng)
    if rng.random() < 0.5:
        d["forecast"] = [round(v - abs(rng.gauss(10, 8)) - 2, 1) for v in d["actual"]]
    return dict(d, limit=rng.choice([3, 4, 4, 5]))


_add("ie_tracking_signal", name="Tracking signal over time", topic=f"{_FC} / forecast accuracy", level="intermediate",
     purpose="Tracking signal each period (running sum of errors / running MAD) and the first period it leaves the "
             "+/- limit.",
     inputs=[_ACT(), _FCS(), _i("limit", "control limit in MADs (default 4)", "control limit", "limits", required=False)],
     outputs=[_o("final_signal", "tracking signal in the last period"), _o("signals", "tracking signal of each period"),
              _o("first_out_of_control", "first period with |TS| > limit (0 = never)")],
     primary="final_signal", conditions=["equal lengths", "running MAD uses all periods so far"],
     method="TS_t = (sum_{i<=t} e_i) / (sum_{i<=t} |e_i| / t)", latex=r"TS_t = \frac{\sum_{i\le t} e_i}{MAD_t}",
     run=tracking_signal, reference=_ref_tracking,
     wrong=lambda actual, forecast, limit=4.0: {"final_signal": sum(a - f for a, f in zip(actual, forecast))
                                                / (sum(abs(a - f) for a, f in zip(actual, forecast)) or 1.0) + 0.5},
     rand=_rand_ts,
     triggers=["tracking signal", "tracking signals"], ask="tracking signal",
     examples=[{"actual": [90, 95, 115, 100, 125, 140], "forecast": [100, 100, 100, 110, 110, 110]},
               {"actual": [52, 48, 60, 55, 62], "forecast": [50, 50, 54, 58, 58], "limit": 3}],
     invalid=[{"actual": [90, 95], "forecast": [100, 100], "limit": -4}])


def linear_trend(data, horizon=1):
    y = _vec("data", data, 2, 10000)
    h = _int("horizon", horizon, 1, 10000)
    n = len(y)
    xs = list(range(1, n + 1))
    mx, my = sum(xs) / n, sum(y) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    b = sum((x - mx) * (v - my) for x, v in zip(xs, y)) / sxx
    if abs(b) <= 1e-12 * max(abs(v) for v in y):
        b = 0.0                                            # a flat series: the slope is exactly 0, not rounding residue
    a = my - b * mx
    return _rd({"forecast": a + b * (n + h), "intercept": a, "slope": b})


def _ref_trend(data, horizon=1):
    import numpy as np
    import statsmodels.api as sm
    n = len(data)
    t = np.arange(1, n + 1, dtype=float)
    fit = sm.OLS(np.asarray(data, float), sm.add_constant(t)).fit()
    a, b = fit.params
    if abs(b) <= 1e-12 * max(abs(v) for v in data):        # the stated convention: a flat series has slope exactly 0
        b = 0.0
    return {"forecast": float(fit.predict([[1.0, n + int(horizon)]])[0]), "intercept": float(a), "slope": float(b)}


_add("ie_linear_trend_forecast", name="Linear trend projection (least squares)", topic=f"{_FC} / time series methods",
     level="intermediate",
     purpose="Least-squares trend line y = a + b t through the data at periods t = 1, ..., n, projected h periods ahead.",
     inputs=[_DATA(), _i("horizon", "periods beyond the last observation (default 1)", "h", "periods ahead", "steps ahead",
                         type="integer", required=False)],
     outputs=[_o("forecast", "a + b (n + h)"), _o("intercept", "a"), _o("slope", "b, change per period")],
     primary="forecast", conditions=["at least 2 observations", "periods numbered 1, 2, ..., n"],
     method="b = sum (t - t_bar)(y - y_bar) / sum (t - t_bar)^2, a = y_bar - b t_bar",
     latex=r"b = \frac{\sum (t-\bar t)(y-\bar y)}{\sum (t-\bar t)^2},\quad a = \bar y - b\bar t", run=linear_trend,
     reference=_ref_trend,
     wrong=lambda data, horizon=1: {"forecast": data[-1] + (data[-1] - data[0]) / len(data) * int(horizon)},
     rand=lambda rng: {"data": [round(50 + 3 * t + rng.gauss(0, 5), 1) for t in range(rng.randint(3, 15))], "horizon": rng.randint(1, 5)},
     triggers=["linear trend", "trend projection", "least squares trend", "trend line forecast"],
     ask="linear trend projection",
     examples=[{"data": [74, 79, 80, 90, 105, 142, 122]}, {"data": [20, 24, 27, 31], "horizon": 3}],
     invalid=[{"data": [20], "horizon": 1}])


def seasonal_index(data, next_year_total=None):
    m = _mat("data", data, 1, cols_min=2, cols_max=366)
    years, s = len(m), len(m[0])
    season = [sum(m[y][k] for y in range(years)) / years for k in range(s)]
    overall = sum(season) / s
    _need(overall > 0, "the average demand must be positive")
    idx = [v / overall for v in season]
    out = {"indices": idx, "season_averages": season, "overall_average": overall}
    if next_year_total is not None:
        T = _nonneg("next_year_total", next_year_total)
        out["forecasts"] = [T / s * v for v in idx]
    return _rd(out)


def _ref_seasonal(data, next_year_total=None):
    import pandas as pd
    df = pd.DataFrame(data, dtype=float)
    avg = df.mean(axis=0)
    idx = avg / df.values.mean()
    out = {"indices": [float(v) for v in idx], "overall_average": float(df.values.mean())}
    if next_year_total is not None:
        out["forecasts"] = [float(next_year_total / df.shape[1] * v) for v in idx]
    return out


_add("ie_seasonal_index", name="Seasonal indices (average method)", topic=f"{_FC} / seasonality", level="intermediate",
     purpose="Seasonal index of each season = average demand in that season / average demand over all seasons; with an "
             "expected annual total, the seasonal forecasts.",
     inputs=[_i("data", "matrix: one row per year, one column per season (month, quarter)", "demand table",
                "seasonal data", "history", type="matrix"),
             _i("next_year_total", "expected total demand next year (optional)", "annual forecast", "next year demand",
                "expected annual demand", required=False)],
     outputs=[_o("indices", "seasonal index of each season"), _o("season_averages", "average of each season"),
              _o("overall_average", "average over all seasons and years"),
              _o("forecasts", "next-year forecast of each season (annual total / seasons x index)")],
     primary="overall_average", conditions=["rows of equal length (at least 2 seasons)", "positive average"],
     method="index_k = mean of season k / overall mean; forecast_k = (annual total / s) index_k",
     latex=r"S_k = \frac{\bar A_k}{\bar A},\quad F_k = \frac{T}{s}\,S_k", run=seasonal_index, reference=_ref_seasonal,
     wrong=lambda data, next_year_total=None: {"overall_average": sum(sum(r) for r in data) / len(data)},
     rand=lambda rng: _drop_none({"data": [[_u(rng, 50, 300, 0) for _ in range(4)] for _ in range(rng.randint(2, 4))],
                                  "next_year_total": rng.choice([None, _u(rng, 500, 1500, 0)])}),
     triggers=["seasonal index", "seasonal indices", "seasonal indexes", "seasonal forecast"], ask="seasonal indices",
     examples=[{"data": [[80, 85, 105, 110], [70, 85, 85, 120], [80, 93, 82, 98]], "next_year_total": 1200},
               {"data": [[200, 350, 150], [220, 330, 170]]}],
     invalid=[{"data": [[0, 0], [0, 0]]}])


def regression_forecast(x, y, x_new=None):
    xs, ys = _vec("x", x, 3, 100000), _vec("y", y, 3, 100000)
    _need(len(xs) == len(ys), "x and y must have the same number of values")
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((v - mx) ** 2 for v in xs)
    syy = sum((v - my) ** 2 for v in ys)
    _need(sxx > 0 and syy > 0, "x and y must both vary")
    sxy = sum((a - mx) * (b - my) for a, b in zip(xs, ys))
    b = sxy / sxx
    a = my - b * mx
    sse = sum((ys[i] - a - b * xs[i]) ** 2 for i in range(n))
    out = {"intercept": a, "slope": b, "r": sxy / math.sqrt(sxx * syy), "r_squared": sxy * sxy / (sxx * syy),
           "standard_error": math.sqrt(sse / (n - 2))}
    if x_new is not None:
        out["forecast"] = a + b * _fin("x_new", x_new)
    return _rd(out)


def _ref_regression(x, y, x_new=None):
    import numpy as np
    import statsmodels.api as sm
    fit = sm.OLS(np.asarray(y, float), sm.add_constant(np.asarray(x, float))).fit()
    a, b = fit.params
    out = {"intercept": float(a), "slope": float(b), "r_squared": float(fit.rsquared),
           "standard_error": float(np.sqrt(fit.mse_resid))}
    if x_new is not None:
        out["forecast"] = float(a + b * x_new)
    return out


_add("ie_regression_forecast", name="Associative forecast by simple linear regression", topic=f"{_FC} / causal methods",
     level="intermediate",
     purpose="Least-squares line y = a + b x of demand on a driver, its correlation and standard error of estimate, and "
             "the forecast at a new x.",
     inputs=[_i("x", "values of the driver (independent variable)", "driver", "independent variable", "predictor", type="vector"),
             _i("y", "values of demand (dependent variable)", "dependent variable", "response", "sales", type="vector"),
             _i("x_new", "driver value to forecast at (optional)", "new x", "forecast at", "x value", required=False)],
     outputs=[_o("intercept", "a"), _o("slope", "b"), _o("r", "correlation coefficient"), _o("r_squared", "coefficient of determination"),
              _o("standard_error", "standard error of the estimate sqrt(SSE/(n - 2))"), _o("forecast", "a + b x_new")],
     primary="slope", conditions=["at least 3 pairs", "x and y vary"],
     method="ordinary least squares; S_yx = sqrt(sum (y - y_hat)^2 / (n - 2))",
     latex=r"\hat y = a + bx,\quad S_{y,x} = \sqrt{\frac{\sum (y-\hat y)^2}{n-2}}", run=regression_forecast,
     reference=_ref_regression,
     wrong=lambda x, y, x_new=None: {"slope": (sum(a * b for a, b in zip(x, y)) / sum(a * a for a in x)),
                                     "standard_error": 1.0},
     rand=lambda rng: (lambda xs: _drop_none({"x": xs, "y": [round(20 + 3 * v + rng.gauss(0, 4), 2) for v in xs],
                                              "x_new": rng.choice([None, _u(rng, 1, 20, 1)])}))(
                                                  [_u(rng, 1, 20, 1) for _ in range(rng.randint(4, 15))]),
     triggers=["regression forecast", "associative forecast", "causal forecast", "linear regression forecast"],
     ask="regression forecast",
     examples=[{"x": [1, 3, 4, 2, 1, 7], "y": [2.0, 3.0, 2.5, 2.0, 2.0, 3.5], "x_new": 6},
               {"x": [10, 12, 15, 18, 20], "y": [110, 125, 140, 170, 182]}],
     invalid=[{"x": [1, 1, 1], "y": [2, 3, 4]}])


# =====================================================================================================================
# LINE BALANCING
# =====================================================================================================================
_LB = "line balancing"


def takt_time(available_time, demand, total_task_time=None):
    T, D = _pos("available_time", available_time), _pos("demand", demand)
    c = T / D
    out = {"cycle_time": c, "output_rate": D / T}
    if total_task_time is not None:
        w = _pos("total_task_time", total_task_time)
        out["minimum_stations"] = _ceil(w / c)
        out["exact_stations"] = w / c
    return _rd(out)


def _ref_takt(available_time, demand, total_task_time=None):
    from fractions import Fraction
    F = lambda v: Fraction(repr(float(v)))
    c = F(available_time) / F(demand)
    out = {"cycle_time": float(c), "output_rate": float(1 / c)}
    if total_task_time is not None:
        x = F(total_task_time) / c
        out["minimum_stations"] = -((-x.numerator) // x.denominator)
        out["exact_stations"] = float(x)
    return out


_add("ie_takt_time", name="Takt (cycle) time and minimum stations", topic=f"{_LB} / cycle time", level="beginner",
     purpose="Cycle time that meets demand (available production time / demand) and, with the total task time, the "
             "theoretical minimum number of stations.",
     inputs=[_i("available_time", "production time available per period (minutes, say)", "available production time",
                "production time", "time available", "available time per day"),
             _i("demand", "units required per period", "required output", "daily demand", "units required", "output"),
             _i("total_task_time", "sum of all task times (same time unit), optional", "sum of task times",
                "work content", "total work content", required=False)],
     outputs=[_o("cycle_time", "takt time = available time / demand"), _o("output_rate", "units per time unit"),
              _o("minimum_stations", "theoretical minimum stations ceil(total task time / cycle time)"),
              _o("exact_stations", "total task time / cycle time")],
     primary="cycle_time", conditions=["inputs > 0", "consistent time units"],
     method="C = available time / demand; N_min = ceil(sum t / C)",
     latex=r"C = \frac{\text{available time}}{\text{demand}},\quad N_{min} = \left\lceil\frac{\sum t_i}{C}\right\rceil",
     run=takt_time, reference=_ref_takt,
     wrong=lambda available_time, demand, total_task_time=None: {"cycle_time": demand / available_time},
     rand=lambda rng: _drop_none({"available_time": rng.choice([420, 450, 480, 960, _u(rng, 100, 2000, 1)]),
                                  "demand": rng.randint(20, 1000), "total_task_time": rng.choice([None, _u(rng, 1, 60, 2)])}),
     triggers=["takt time", "cycle time required", "required cycle time", "cycle time to meet demand"], ask="takt time",
     examples=[{"available_time": 480, "demand": 60, "total_task_time": 30}, {"available_time": 450, "demand": 900}],
     invalid=[{"available_time": 480, "demand": 0}],
     weak="a definition: the reference recomputes it exactly with fractions")


def line_efficiency(task_times, cycle_time, stations):
    t = _vec("task_times", task_times, 1, 1000, positive=True)
    c, n = _pos("cycle_time", cycle_time), _int("stations", stations, 1, 10000)
    w = sum(t)
    _need(max(t) <= c * (1 + 1e-12), "every task must fit within the cycle time")
    _need(w <= n * c * (1 + 1e-12), "the stations cannot hold the total work at this cycle time")
    eff = w / (n * c)
    return _rd({"efficiency": eff, "balance_delay": 1 - eff, "idle_time": n * c - w, "total_task_time": w,
                "minimum_stations": _ceil(w / c)})


def _ref_line_eff(task_times, cycle_time, stations):
    from fractions import Fraction
    F = lambda v: Fraction(repr(float(v)))
    w = sum(F(v) for v in task_times)
    cap = int(stations) * F(cycle_time)
    x = w / F(cycle_time)
    return {"efficiency": float(w / cap), "balance_delay": float(1 - w / cap), "idle_time": float(cap - w),
            "total_task_time": float(w), "minimum_stations": -((-x.numerator) // x.denominator)}


def _rand_line_eff(rng):
    t = [_u(rng, 0.2, 3, 2) for _ in range(rng.randint(4, 12))]
    c = round(max(t) * rng.uniform(1, 2), 2)
    return {"task_times": t, "cycle_time": c, "stations": _ceil(sum(t) / c) + rng.randint(0, 2)}


_add("ie_line_efficiency", name="Line balance efficiency, balance delay and idle time", topic=f"{_LB} / measures",
     level="beginner",
     purpose="Efficiency of a balanced line: total task time / (stations x cycle time), the balance delay and the idle "
             "time per cycle.",
     inputs=[_i("task_times", "time of each task", "task times", "element times", "times", type="vector"),
             _i("cycle_time", "cycle time", "c", "cycle time", "takt time"),
             _i("stations", "number of workstations used", "n", "workstations", "number of stations", type="integer")],
     outputs=[_o("efficiency", "sum t / (N C)"), _o("balance_delay", "1 - efficiency"),
              _o("idle_time", "N C - sum t per cycle"), _o("total_task_time", "sum of task times"),
              _o("minimum_stations", "theoretical minimum stations")],
     primary="efficiency", conditions=["task times > 0", "each task <= cycle time", "sum t <= N C"],
     method="E = sum t / (N C); idle = N C - sum t", latex=r"E = \frac{\sum t_i}{N\,C},\quad \text{balance delay} = 1 - E",
     run=line_efficiency, reference=_ref_line_eff,
     wrong=lambda task_times, cycle_time, stations: {"efficiency": sum(task_times) / (int(stations) * max(task_times))},
     rand=_rand_line_eff,
     triggers=["line efficiency", "balance delay", "line balance efficiency", "efficiency of the line",
               "efficiency of the balance"], ask="line efficiency",
     examples=[{"task_times": [0.5, 1.2, 0.8, 0.4, 1.0, 0.6], "cycle_time": 1.5, "stations": 4},
               {"task_times": [45, 30, 50, 20, 35, 40], "cycle_time": 60, "stations": 5}],
     invalid=[{"task_times": [0.5, 2.2, 0.8], "cycle_time": 1.5, "stations": 4}],
     weak="a definition: the reference recomputes it exactly with fractions")


def _precedence(n, pairs):
    m = _mat("precedence", pairs, 1, cols=2)
    preds = [[] for _ in range(n)]
    for a, b in m:
        _need(a == int(a) and b == int(b) and 1 <= a <= n and 1 <= b <= n and a != b,
              "each precedence row [a, b] names two different task numbers (a before b)")
        if int(a) - 1 not in preds[int(b) - 1]:
            preds[int(b) - 1].append(int(a) - 1)
    state = [0] * n                                        # cycle check by repeated removal of free tasks
    left = n
    done = [False] * n
    while left:
        free = [j for j in range(n) if not done[j] and all(done[p] for p in preds[j])]
        _need(len(free) > 0, "the precedence relations contain a cycle")
        for j in free:
            done[j] = True
            left -= 1
    return preds


def _balance(t, preds, c, priority):
    n = len(t)
    station = [0] * n
    loads = []
    assigned = [False] * n
    k = 0
    while k < n:
        loads.append(0.0)
        s = len(loads)
        while True:
            best = -1
            for j in range(n):
                if assigned[j] or loads[-1] + t[j] > c * (1 + 1e-12):
                    continue
                if not all(assigned[p] for p in preds[j]):
                    continue
                if best < 0 or priority[j] > priority[best]:
                    best = j
            if best < 0:
                break
            assigned[best] = True
            station[best] = s
            loads[-1] += t[best]
            k += 1
        _need(loads[-1] > 0, "a task does not fit in the cycle time")
    N = len(loads)
    w = sum(t)
    return {"stations": N, "assignment": station, "station_times": loads, "efficiency": w / (N * c),
            "idle_time": N * c - w, "minimum_stations": _ceil(w / c)}


def largest_candidate(task_times, precedence, cycle_time):
    t = _vec("task_times", task_times, 2, 300, positive=True)
    c = _pos("cycle_time", cycle_time)
    _need(max(t) <= c * (1 + 1e-12), "every task must fit within the cycle time")
    preds = _precedence(len(t), precedence)
    n = len(t)
    pr = [t[j] - j * 1e-9 * 0 for j in range(n)]
    order = sorted(range(n), key=lambda j: (-t[j], j))
    rank = [0] * n
    for pos in range(n):
        rank[order[pos]] = n - pos                         # larger time first, ties by the lower task number
    return _rd(_balance(t, preds, c, rank))


def _ref_balance(task_times, precedence, cycle_time, weights):
    import pandas as pd
    n = len(task_times)
    pre = {j: {int(a) - 1 for a, b in precedence if int(b) - 1 == j} for j in range(n)}
    df = pd.DataFrame({"t": task_times, "w": weights, "job": range(n)}).sort_values(["w", "job"], ascending=[False, True])
    ranked = list(df["job"])
    done, station, loads = set(), {}, []
    while len(done) < n:
        loads.append(0.0)
        moved = True
        while moved:
            moved = False
            for j in ranked:                               # the first eligible task in priority order that fits
                if j not in done and pre[j] <= done and loads[-1] + task_times[j] <= cycle_time * (1 + 1e-12):
                    done.add(j)
                    station[j] = len(loads)
                    loads[-1] += task_times[j]
                    moved = True
                    break
    for j in range(n):                                     # feasibility of the result
        assert all(station[p] <= station[j] for p in pre[j])
    N = len(loads)
    return {"stations": N, "assignment": [station[j] for j in range(n)], "station_times": loads,
            "efficiency": sum(task_times) / (N * cycle_time)}


def _ref_lcr(task_times, precedence, cycle_time):
    return _ref_balance(task_times, precedence, cycle_time, list(task_times))


def _rand_line(rng):
    n = rng.randint(5, 10)
    t = [rng.randint(1, 9) for _ in range(n)]
    pairs = []
    for b in range(1, n):
        for a in rng.sample(range(b), min(b, rng.randint(1, 2))):
            pairs.append([a + 1, b + 1])
    return {"task_times": t, "precedence": pairs, "cycle_time": max(t) + rng.randint(0, 8)}


_PREC_IN = lambda: _i("precedence", "precedence rows [a, b]: task a must be done before task b", "precedence relations",
                      "predecessors", "precedence pairs", type="matrix")
_LB_OUT = [_o("stations", "number of stations"), _o("assignment", "station number of each task"),
           _o("station_times", "work assigned to each station"), _o("efficiency", "sum t / (N C)"),
           _o("idle_time", "N C - sum t"), _o("minimum_stations", "theoretical minimum stations")]

_add("ie_largest_candidate_rule", name="Line balancing by the largest candidate rule", topic=f"{_LB} / heuristics",
     level="advanced",
     purpose="Assign tasks to stations: fill each station with the longest eligible task that still fits (all "
             "predecessors assigned), then open the next station.",
     inputs=[_i("task_times", "time of each task (tasks numbered 1, 2, ...)", "task times", "element times", type="vector"),
             _PREC_IN(), _i("cycle_time", "cycle time", "c", "takt time")],
     outputs=_LB_OUT, primary="efficiency",
     conditions=["task times > 0 and <= cycle time", "precedence acyclic", "ties go to the lower task number"],
     method="largest candidate rule: at each station repeatedly add the eligible task with the largest time that fits",
     latex=r"E = \frac{\sum t_i}{N\,C}", run=largest_candidate, reference=_ref_lcr,
     wrong=lambda task_times, precedence, cycle_time: {"efficiency": sum(task_times) / (math.ceil(sum(task_times) / cycle_time) * cycle_time) + 0.01},
     rand=_rand_line,
     triggers=["largest candidate rule", "largest candidate", "longest task time rule", "longest operation time"],
     ask="line balancing by the largest candidate rule",
     examples=[{"task_times": [2, 3, 4, 3, 6, 4, 2, 3], "precedence": [[1, 2], [1, 3], [2, 4], [3, 5], [4, 6], [5, 6], [6, 7], [7, 8]],
                "cycle_time": 10},
               {"task_times": [5, 3, 4, 3, 6, 1], "precedence": [[1, 3], [2, 3], [3, 4], [3, 5], [4, 6], [5, 6]], "cycle_time": 8}],
     invalid=[{"task_times": [5, 3, 4], "precedence": [[1, 2], [2, 3], [3, 1]], "cycle_time": 8}],
     weak="the reference applies the same priority rule in a separate implementation (pandas ranking) and checks the "
          "precedence feasibility of the result")


def ranked_positional_weight(task_times, precedence, cycle_time):
    t = _vec("task_times", task_times, 2, 300, positive=True)
    c = _pos("cycle_time", cycle_time)
    _need(max(t) <= c * (1 + 1e-12), "every task must fit within the cycle time")
    n = len(t)
    preds = _precedence(n, precedence)
    succ = [[] for _ in range(n)]
    for j in range(n):
        for p in preds[j]:
            succ[p].append(j)
    rpw = []
    for j in range(n):
        seen, stack = set(), [j]
        while stack:
            u = stack.pop()
            for v in succ[u]:
                if v not in seen:
                    seen.add(v)
                    stack.append(v)
        rpw.append(t[j] + sum(t[v] for v in seen))
    order = sorted(range(n), key=lambda j: (-rpw[j], j))
    rank = [0] * n
    for pos in range(n):
        rank[order[pos]] = n - pos
    out = _balance(t, preds, c, rank)
    out["positional_weights"] = rpw
    return _rd(out)


def _ref_rpw(task_times, precedence, cycle_time):
    import numpy as np
    from scipy.sparse import csr_matrix
    from scipy.sparse.csgraph import shortest_path
    n = len(task_times)
    g = csr_matrix((np.ones(len(precedence)), ([int(a) - 1 for a, b in precedence], [int(b) - 1 for a, b in precedence])), shape=(n, n))
    reach = np.isfinite(shortest_path(g, directed=True, unweighted=True))   # j reaches k: k is a successor of j
    t = np.asarray(task_times, float)
    rpw = [float((t * reach[j]).sum()) for j in range(n)]                    # includes the task itself (distance 0)
    out = _ref_balance(task_times, precedence, cycle_time, rpw)
    out["positional_weights"] = rpw
    return out


_add("ie_ranked_positional_weight", name="Line balancing by ranked positional weight", topic=f"{_LB} / heuristics",
     level="expert",
     purpose="Helgeson-Birnie method: rank tasks by positional weight (own time + times of all following tasks) and "
             "fill stations in that order.",
     inputs=[_i("task_times", "time of each task (tasks numbered 1, 2, ...)", "task times", "element times", type="vector"),
             _PREC_IN(), _i("cycle_time", "cycle time", "c", "takt time")],
     outputs=_LB_OUT + [_o("positional_weights", "positional weight of each task")], primary="efficiency",
     conditions=["task times > 0 and <= cycle time", "precedence acyclic", "ties go to the lower task number"],
     method="RPW_j = t_j + sum of t over all successors of j; assign the eligible task with the highest RPW that fits",
     latex=r"RPW_j = t_j + \sum_{k \in S(j)} t_k", run=ranked_positional_weight, reference=_ref_rpw,
     wrong=lambda task_times, precedence, cycle_time: {"efficiency": sum(task_times) / ((math.ceil(sum(task_times) / cycle_time) + 2) * cycle_time),
                                                       "positional_weights": list(task_times)},
     rand=_rand_line,
     triggers=["ranked positional weight", "positional weight", "helgeson birnie", "helgeson-birnie"],
     ask="line balancing by ranked positional weight",
     examples=[{"task_times": [2, 3, 4, 3, 6, 4, 2, 3], "precedence": [[1, 2], [1, 3], [2, 4], [3, 5], [4, 6], [5, 6], [6, 7], [7, 8]],
                "cycle_time": 10},
               {"task_times": [5, 3, 4, 3, 6, 1], "precedence": [[1, 3], [2, 3], [3, 4], [3, 5], [4, 6], [5, 6]], "cycle_time": 8}],
     invalid=[{"task_times": [5, 3, 9], "precedence": [[1, 2], [2, 3]], "cycle_time": 8}],
     weak="positional weights are checked independently (graph reachability with SciPy); the station filling repeats "
          "the same rule in a separate implementation")


# =====================================================================================================================
# LEARNING CURVES
# =====================================================================================================================
_LC = "learning curves"
_T1 = lambda: _i("first_unit_time", "time (or cost) of the first unit", "first unit time", "time for the first unit",
                 "t1", "first unit cost")
_RATE = lambda: _i("learning_rate", "learning rate (decimal; 80% = 0.8)", "learning curve rate", "rate", "learning percentage",
                   "learning curve")


def _b(r):
    return math.log(r) / math.log(2.0)


def learning_unit(first_unit_time, learning_rate, unit):
    T1, r = _pos("first_unit_time", first_unit_time), _prob("learning_rate", learning_rate, 0.0, 1.0 + 1e-12)
    n = _int("unit", unit, 1, 10 ** 12)
    b = _b(r)
    return _rd({"unit_time": T1 * n ** b, "exponent": b})


def _ref_learning_unit(first_unit_time, learning_rate, unit):
    import mpmath
    with mpmath.workdps(40):
        b = mpmath.log(learning_rate) / mpmath.log(2)
        return {"unit_time": float(first_unit_time * mpmath.power(int(unit), b)), "exponent": float(b)}


_add("ie_learning_curve_unit", name="Learning curve: time of the n-th unit", topic=f"{_LC} / unit model",
     level="beginner",
     purpose="Time of the n-th unit on a learning curve: each doubling of output multiplies the unit time by the "
             "learning rate (T_n = T_1 n^b, b = log2 r).",
     inputs=[_T1(), _RATE(), _i("unit", "unit number n", "n", "unit number", "nth unit", type="integer")],
     outputs=[_o("unit_time", "T_1 n^b"), _o("exponent", "b = log2(learning rate)")],
     primary="unit_time", conditions=["first_unit_time > 0", "0 < learning_rate <= 1", "unit >= 1"],
     method="T_n = T_1 n^(log2 r)", latex=r"T_n = T_1\,n^{b},\quad b = \frac{\ln r}{\ln 2}", run=learning_unit,
     reference=_ref_learning_unit,
     wrong=lambda first_unit_time, learning_rate, unit: {"unit_time": first_unit_time * learning_rate ** (int(unit) - 1)},
     rand=lambda rng: {"first_unit_time": _u(rng, 1, 1000, 2), "learning_rate": rng.choice([0.7, 0.75, 0.8, 0.85, 0.9, _u(rng, 0.65, 0.95, 3)]),
                       "unit": rng.randint(3, 500)},
     triggers=["learning curve unit time", "time of the nth unit", "learning curve", "unit learning curve"],
     ask="learning curve unit time",
     examples=[{"first_unit_time": 100, "learning_rate": 0.8, "unit": 8}, {"first_unit_time": 50, "learning_rate": 0.85, "unit": 10}],
     invalid=[{"first_unit_time": 100, "learning_rate": 1.8, "unit": 8}],
     weak="the reference evaluates the same power law in 40-digit arithmetic")


def learning_cumulative(first_unit_time, learning_rate, units):
    T1, r = _pos("first_unit_time", first_unit_time), _prob("learning_rate", learning_rate, 0.0, 1.0 + 1e-12)
    N = _int("units", units, 1, 200000)
    b = _b(r)
    tot = 0.0
    for i in range(1, N + 1):
        tot += i ** b
    return _rd({"total_time": T1 * tot, "average_time": T1 * tot / N, "last_unit_time": T1 * N ** b})


def _ref_learning_cum(first_unit_time, learning_rate, units):
    import mpmath
    with mpmath.workdps(30):
        s = -mpmath.log(learning_rate) / mpmath.log(2)     # sum_{i<=N} i^(-s) = zeta(s, 1) - zeta(s, N + 1)
        tot = first_unit_time * (mpmath.mpf(int(units)) if s == 0 else mpmath.zeta(s, 1) - mpmath.zeta(s, int(units) + 1))
        return {"total_time": float(tot), "average_time": float(tot / int(units)),
                "last_unit_time": float(first_unit_time * mpmath.power(int(units), -s))}


_add("ie_learning_curve_total", name="Learning curve: total and average time of N units", topic=f"{_LC} / unit model",
     level="intermediate",
     purpose="Total and average time to make units 1 to N on a unit learning curve (sum of T_1 i^b).",
     inputs=[_T1(), _RATE(), _i("units", "number of units N", "n", "number of units", "quantity", type="integer")],
     outputs=[_o("total_time", "sum_{i=1}^{N} T_1 i^b"), _o("average_time", "total / N"), _o("last_unit_time", "T_1 N^b")],
     primary="total_time", conditions=["first_unit_time > 0", "0 < learning_rate <= 1", "1 <= units <= 200000"],
     method="total = T_1 sum_{i=1}^{N} i^b", latex=r"\sum_{i=1}^{N} T_1\,i^{b}", run=learning_cumulative,
     reference=_ref_learning_cum,
     wrong=lambda first_unit_time, learning_rate, units: {"total_time": first_unit_time * int(units) ** (_b(learning_rate) + 1)},
     rand=lambda rng: {"first_unit_time": _u(rng, 1, 1000, 2), "learning_rate": rng.choice([0.7, 0.75, 0.8, 0.85, 0.9, _u(rng, 0.65, 0.95, 3)]),
                       "units": rng.randint(2, 2000)},
     triggers=["total time for units on a learning curve", "learning curve total time", "cumulative time learning curve",
               "learning curve cumulative time", "total learning curve time"], ask="learning curve total time",
     examples=[{"first_unit_time": 100, "learning_rate": 0.8, "units": 8}, {"first_unit_time": 60, "learning_rate": 0.9, "units": 25}],
     invalid=[{"first_unit_time": 100, "learning_rate": 0.8, "units": 0}])


def learning_two_points(unit1, time1, unit2, time2):
    n1, n2 = _pos("unit1", unit1), _pos("unit2", unit2)
    t1, t2 = _pos("time1", time1), _pos("time2", time2)
    _need(n1 != n2, "the two unit numbers must differ")
    b = math.log(t2 / t1) / math.log(n2 / n1)
    return _rd({"learning_rate": 2.0 ** b, "exponent": b, "first_unit_time": t1 / n1 ** b})


def _ref_learning_two(unit1, time1, unit2, time2):
    import numpy as np
    import statsmodels.api as sm
    fit = sm.OLS(np.log([time1, time2]), sm.add_constant(np.log([unit1, unit2]))).fit()
    a, b = fit.params
    return {"learning_rate": float(2.0 ** b), "exponent": float(b), "first_unit_time": float(np.exp(a))}


_add("ie_learning_rate_from_two_units", name="Learning rate from two unit times", topic=f"{_LC} / estimation",
     level="intermediate",
     purpose="Learning rate, exponent and first-unit time of a unit learning curve through two observed (unit, time) "
             "points.",
     inputs=[_i("unit1", "first unit number", "first unit", "unit a", "n1"), _i("time1", "time of that unit", "time a", "t1 time"),
             _i("unit2", "second unit number", "second unit", "unit b", "n2"), _i("time2", "time of that unit", "time b", "t2 time")],
     outputs=[_o("learning_rate", "2^b"), _o("exponent", "b = ln(t2/t1)/ln(n2/n1)"), _o("first_unit_time", "t1 / n1^b")],
     primary="learning_rate", conditions=["units and times > 0", "unit numbers differ"],
     method="b = ln(t2/t1) / ln(n2/n1), r = 2^b, T_1 = t1 / n1^b",
     latex=r"b = \frac{\ln(t_2/t_1)}{\ln(n_2/n_1)},\quad r = 2^{b}", run=learning_two_points, reference=_ref_learning_two,
     wrong=lambda unit1, time1, unit2, time2: {"learning_rate": (time2 / time1) ** (1 / (unit2 / unit1))},
     rand=lambda rng: (lambda T, r, a, b: {"unit1": a, "time1": round(T * a ** _b(r), 3), "unit2": b, "time2": round(T * b ** _b(r), 3)})(
         _u(rng, 10, 500, 1), _u(rng, 0.65, 0.95, 3), rng.randint(1, 20), rng.randint(25, 400)),
     triggers=["learning rate from two units", "learning rate from two points", "estimate the learning rate",
               "find the learning rate"], ask="learning rate from two units",
     examples=[{"unit1": 1, "time1": 100, "unit2": 8, "time2": 51.2}, {"unit1": 10, "time1": 40, "unit2": 40, "time2": 27}],
     invalid=[{"unit1": 4, "time1": 100, "unit2": 4, "time2": 51.2}])


def learning_fit(units, times):
    n_ = _vec("units", units, 2, 10000, positive=True)
    t = _vec("times", times, 2, 10000, positive=True)
    _need(len(n_) == len(t), "units and times must have the same number of values")
    xs, ys = [math.log(v) for v in n_], [math.log(v) for v in t]
    k = len(xs)
    mx, my = sum(xs) / k, sum(ys) / k
    sxx = sum((x - mx) ** 2 for x in xs)
    _need(sxx > 0, "the unit numbers must not all be equal")
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    syy = sum((y - my) ** 2 for y in ys)
    b = sxy / sxx
    a = my - b * mx
    return _rd({"learning_rate": 2.0 ** b, "exponent": b, "first_unit_time": math.exp(a),
                "r_squared": sxy * sxy / (sxx * syy) if syy > 0 else 1.0})


def _ref_learning_fit(units, times):
    import numpy as np
    import statsmodels.api as sm
    fit = sm.OLS(np.log(times), sm.add_constant(np.log(units))).fit()
    a, b = fit.params
    return {"learning_rate": float(2.0 ** b), "exponent": float(b), "first_unit_time": float(np.exp(a)), "r_squared": float(fit.rsquared)}


_add("ie_learning_curve_fit", name="Fit a learning curve to data (log-log regression)", topic=f"{_LC} / estimation",
     level="advanced",
     purpose="Learning rate and first-unit time estimated by least squares on log(time) against log(unit number).",
     inputs=[_i("units", "unit numbers observed", "unit numbers", "unit", type="vector"),
             _i("times", "time of each observed unit", "unit times", "observed times", type="vector")],
     outputs=[_o("learning_rate", "2^b"), _o("exponent", "slope b of the log-log fit"),
              _o("first_unit_time", "exp(intercept)"), _o("r_squared", "coefficient of determination")],
     primary="learning_rate", conditions=["positive values", "at least 2 different unit numbers"],
     method="regress ln T on ln n: ln T = ln T_1 + b ln n; r = 2^b", latex=r"\ln T_n = \ln T_1 + b\ln n",
     run=learning_fit, reference=_ref_learning_fit,
     wrong=lambda units, times: {"learning_rate": times[-1] / times[0] if len(times) > 1 else 0.5},
     rand=lambda rng: (lambda T, r: (lambda us: {"units": us, "times": [round(T * u ** _b(r) * rng.uniform(0.95, 1.05), 2) for u in us]})(
         sorted(rng.sample(range(1, 200), rng.randint(3, 10)))))(_u(rng, 10, 500, 1), _u(rng, 0.65, 0.95, 3)),
     triggers=["fit a learning curve", "learning curve fit", "learning curve regression", "estimate the learning curve"],
     ask="fit a learning curve",
     examples=[{"units": [1, 2, 4, 8, 16], "times": [100, 82, 63, 52, 40]}, {"units": [5, 10, 20, 40], "times": [60, 51, 44, 38]}],
     invalid=[{"units": [3, 3], "times": [60, 51]}])


def wright_average(first_unit_time, learning_rate, units):
    T1, r = _pos("first_unit_time", first_unit_time), _prob("learning_rate", learning_rate, 0.0, 1.0 + 1e-12)
    N = _int("units", units, 1, 10 ** 12)
    b = _b(r)
    avg = T1 * N ** b
    tot = avg * N
    prev = T1 * (N - 1) ** (b + 1) if N > 1 else 0.0
    return _rd({"cumulative_average_time": avg, "total_time": tot, "unit_time": tot - prev})


def _ref_wright(first_unit_time, learning_rate, units):
    import mpmath
    with mpmath.workdps(40):
        b = mpmath.log(learning_rate) / mpmath.log(2)
        N = int(units)
        tot = first_unit_time * mpmath.power(N, b + 1)
        prev = first_unit_time * mpmath.power(N - 1, b + 1) if N > 1 else 0
        return {"cumulative_average_time": float(tot / N), "total_time": float(tot), "unit_time": float(tot - prev)}


_add("ie_learning_curve_cumulative_average", name="Cumulative-average learning curve (Wright)", topic=f"{_LC} / Wright model",
     level="advanced",
     purpose="Wright's model, where the cumulative AVERAGE time falls by the learning rate at each doubling: average, "
             "total and N-th unit time.",
     inputs=[_T1(), _RATE(), _i("units", "cumulative units N", "n", "number of units", "quantity", type="integer")],
     outputs=[_o("cumulative_average_time", "T_1 N^b"), _o("total_time", "T_1 N^(b+1)"),
              _o("unit_time", "time of unit N: total(N) - total(N - 1)")],
     primary="cumulative_average_time", conditions=["first_unit_time > 0", "0 < learning_rate <= 1", "units >= 1"],
     method="average(N) = T_1 N^b; total = N average; unit N = total(N) - total(N - 1)",
     latex=r"\bar T_N = T_1 N^{b},\quad \text{total}_N = T_1 N^{b+1}", run=wright_average, reference=_ref_wright,
     wrong=lambda first_unit_time, learning_rate, units: {"cumulative_average_time": first_unit_time * int(units) ** _b(learning_rate) / 1.0 * (1 if int(units) == 1 else 0) + first_unit_time * (int(units) ** (_b(learning_rate) + 1) - (int(units) - 1) ** (_b(learning_rate) + 1)) * (0 if int(units) == 1 else 1) + (0.5 if int(units) == 1 else 0)},
     rand=lambda rng: {"first_unit_time": _u(rng, 1, 1000, 2), "learning_rate": rng.choice([0.7, 0.75, 0.8, 0.85, 0.9, _u(rng, 0.65, 0.95, 3)]),
                       "units": rng.randint(2, 5000)},
     triggers=["cumulative average learning curve", "cumulative-average learning curve", "wright learning curve",
               "wright's learning curve", "cumulative average time"], ask="cumulative average learning curve",
     examples=[{"first_unit_time": 100, "learning_rate": 0.8, "units": 4}, {"first_unit_time": 2000, "learning_rate": 0.85, "units": 50}],
     invalid=[{"first_unit_time": 100, "learning_rate": 0, "units": 4}],
     weak="the reference evaluates the same model in 40-digit arithmetic")


# =====================================================================================================================
# FACILITY LOCATION AND BREAK-EVEN ANALYSIS
# =====================================================================================================================
_LOC = "facility location"
_XY = lambda: [_i("x", "x coordinate of each demand point", "x coordinates", "xs", type="vector"),
               _i("y", "y coordinate of each demand point", "y coordinates", "ys", type="vector"),
               _i("weights", "load or volume of each point", "loads", "volumes", "demands", "shipments", type="vector")]


def _xyw(x, y, weights):
    xs, ys = _vec("x", x, 1, 10000), _vec("y", y, 1, 10000)
    w = _vec("weights", weights, 1, 10000, nonneg=True)
    _need(len(xs) == len(ys) == len(w), "x, y and weights must have the same number of values")
    _need(sum(w) > 0, "the weights must not all be 0")
    return xs, ys, w


def center_of_gravity(x, y, weights):
    xs, ys, w = _xyw(x, y, weights)
    W = sum(w)
    cx = sum(a * b for a, b in zip(xs, w)) / W
    cy = sum(a * b for a, b in zip(ys, w)) / W
    return _rd({"x": cx, "y": cy, "total_weight": W})


def _ref_cog(x, y, weights):
    import numpy as np
    return {"x": float(np.average(x, weights=weights)), "y": float(np.average(y, weights=weights)), "total_weight": float(np.sum(weights))}


def _rand_points(rng, k_lo=3, k_hi=8):
    k = rng.randint(k_lo, k_hi)
    return {"x": [_u(rng, 0, 100, 1) for _ in range(k)], "y": [_u(rng, 0, 100, 1) for _ in range(k)],
            "weights": [rng.randint(1, 100) * 10 for _ in range(k)]}


_add("ie_center_of_gravity", name="Centre-of-gravity location", topic=f"{_LOC} / single facility", level="beginner",
     purpose="Location of a facility at the load-weighted average of the demand points' coordinates.",
     inputs=_XY(),
     outputs=[_o("x", "sum w x / sum w"), _o("y", "sum w y / sum w"), _o("total_weight", "sum of loads")],
     primary="x", conditions=["equal lengths", "weights >= 0, not all 0"],
     method="x* = sum w_i x_i / sum w_i, y* = sum w_i y_i / sum w_i",
     latex=r"x^* = \frac{\sum w_i x_i}{\sum w_i},\quad y^* = \frac{\sum w_i y_i}{\sum w_i}", run=center_of_gravity,
     reference=_ref_cog,
     wrong=lambda x, y, weights: {"x": sum(x) / len(x) + (0.5 if len(set(weights)) == 1 else 0), "y": sum(y) / len(y)},
     rand=_rand_points,
     triggers=["centre of gravity", "center of gravity", "center-of-gravity", "centre-of-gravity"],
     ask="centre-of-gravity location",
     examples=[{"x": [30, 90, 130, 60], "y": [120, 110, 130, 40], "weights": [2000, 1000, 1000, 2000]},
               {"x": [2, 6, 8], "y": [3, 9, 4], "weights": [400, 300, 200]}],
     invalid=[{"x": [30, 90], "y": [120, 110, 130], "weights": [2000, 1000]}],
     weak="a definition: the reference is NumPy's weighted average")


def _weber_vertex(xs, ys, w, i):
    gx = gy = 0.0
    for j in range(len(xs)):
        if j == i:
            continue
        d = math.hypot(xs[i] - xs[j], ys[i] - ys[j])
        if d > 0:
            gx += w[j] * (xs[i] - xs[j]) / d
            gy += w[j] * (ys[i] - ys[j]) / d
    return math.hypot(gx, gy) <= w[i]


def weber_location(x, y, weights):
    xs, ys, w = _xyw(x, y, weights)
    _need(len(xs) >= 2, "at least two points are needed")
    for i in range(len(xs)):
        for j in range(i):
            _need(xs[i] != xs[j] or ys[i] != ys[j], "the points must be distinct")
    best = None
    for i in range(len(xs)):
        if _weber_vertex(xs, ys, w, i):
            best = (xs[i], ys[i])
    if best is None:
        W = sum(w)
        px = sum(a * b for a, b in zip(xs, w)) / W
        py = sum(a * b for a, b in zip(ys, w)) / W
        scale = max(1.0, max(abs(v) for v in xs + ys))
        for _ in range(200000):
            nx = ny = den = 0.0
            for k in range(len(xs)):
                d = math.hypot(px - xs[k], py - ys[k])
                _need(d > 1e-300, "the iteration reached a demand point")
                nx += w[k] * xs[k] / d
                ny += w[k] * ys[k] / d
                den += w[k] / d
            nx, ny = nx / den, ny / den
            step = math.hypot(nx - px, ny - py)
            px, py = nx, ny
            if step <= 1e-15 * scale:
                break
        best = (px, py)
    cost = sum(w[k] * math.hypot(best[0] - xs[k], best[1] - ys[k]) for k in range(len(xs)))
    return _rd({"x": best[0], "y": best[1], "total_weighted_distance": cost})


def _ref_weber(x, y, weights):
    import numpy as np
    from scipy.optimize import minimize, root
    P, w = np.column_stack([x, y]).astype(float), np.asarray(weights, float)
    f = lambda p: float((w * np.hypot(*(p - P).T)).sum())
    for i in range(len(w)):                                # optimal at a demand point? (subgradient condition)
        d = np.hypot(*(P[i] - P).T)
        m = d > 0
        g = ((w[m] / d[m])[:, None] * (P[i] - P[m])).sum(axis=0)
        if np.hypot(*g) <= w[i]:
            return {"x": float(P[i, 0]), "y": float(P[i, 1]), "total_weighted_distance": f(P[i])}
    grad = lambda p: ((w / np.hypot(*(p - P).T))[:, None] * (p - P)).sum(axis=0)
    p0 = minimize(f, np.average(P, axis=0, weights=w), method="BFGS", options={"gtol": 1e-12}).x
    p = root(grad, p0, method="hybr", tol=1e-15).x
    return {"x": float(p[0]), "y": float(p[1]), "total_weighted_distance": f(p)}


_add("ie_weber_location", name="Minimum weighted-distance location (Weber problem)", topic=f"{_LOC} / single facility",
     level="expert",
     purpose="Location that minimises the sum of load x straight-line distance to the demand points (Weiszfeld's "
             "algorithm; a demand point when it is optimal).",
     inputs=_XY(),
     outputs=[_o("x", "optimal x"), _o("y", "optimal y"), _o("total_weighted_distance", "sum w_i distance_i at the optimum")],
     primary="total_weighted_distance", conditions=["at least 2 distinct points", "weights >= 0, not all 0", "Euclidean distance"],
     method="Weiszfeld: x <- sum (w_i x_i / d_i) / sum (w_i / d_i) until it settles; a demand point i is optimal when "
            "|sum_{j != i} w_j (p_i - p_j)/d_ij| <= w_i",
     latex=r"\min_{x,y}\sum_i w_i\sqrt{(x-x_i)^2+(y-y_i)^2}", run=weber_location, reference=_ref_weber,
     wrong=lambda x, y, weights: (lambda c: {"total_weighted_distance": sum(w * math.hypot(c["x"] - a, c["y"] - b) for a, b, w in zip(x, y, weights)) * 1.0 + 1e-3 * sum(weights)})(
         center_of_gravity(x, y, weights)),
     rand=lambda rng: _rand_points(rng, 3, 7),
     triggers=["weber problem", "minimum weighted distance location", "weiszfeld", "minimise total weighted distance",
               "minimize total weighted distance", "euclidean minisum location"], ask="Weber problem location",
     examples=[{"x": [0, 10, 5, 2], "y": [0, 0, 8, 6], "weights": [1, 1, 1, 1]},
               {"x": [30, 90, 130, 60], "y": [120, 110, 130, 40], "weights": [2000, 1000, 1000, 2000]}],
     invalid=[{"x": [1, 1], "y": [2, 2], "weights": [1, 1]}])


def _wmedian(v, w):
    order = sorted(range(len(v)), key=lambda k: (v[k], k))
    half, acc = sum(w) / 2, 0.0
    for idx in range(len(order)):
        k = order[idx]
        acc += w[k]
        if acc == half:
            nxt = v[order[idx + 1]] if idx + 1 < len(order) else v[k]
            _need(nxt == v[k], f"every value between {v[k]:g} and {nxt:g} is optimal (the median is not unique)")
        if acc >= half:
            return v[k]
    return v[order[-1]]


def rectilinear_location(x, y, weights):
    xs, ys, w = _xyw(x, y, weights)
    bx, by = _wmedian(xs, w), _wmedian(ys, w)
    cost = sum(w[k] * (abs(bx - xs[k]) + abs(by - ys[k])) for k in range(len(xs)))
    return _rd({"x": bx, "y": by, "total_weighted_distance": cost})


def _ref_rectilinear(x, y, weights):
    import numpy as np
    xs, ys, w = (np.asarray(v, float) for v in (x, y, weights))
    cx = min(xs, key=lambda c: (float((w * np.abs(c - xs)).sum()), c))
    cy = min(ys, key=lambda c: (float((w * np.abs(c - ys)).sum()), c))
    return {"x": float(cx), "y": float(cy), "total_weighted_distance": float((w * (np.abs(cx - xs) + np.abs(cy - ys))).sum())}


_add("ie_rectilinear_location", name="Minimum rectilinear-distance location (weighted median)",
     topic=f"{_LOC} / single facility", level="advanced",
     purpose="Location minimising the sum of load x rectilinear (city-block) distance: the weighted median of x and of y.",
     inputs=_XY(),
     outputs=[_o("x", "weighted median of x"), _o("y", "weighted median of y"),
              _o("total_weighted_distance", "sum w_i (|x - x_i| + |y - y_i|)")],
     primary="total_weighted_distance",
     conditions=["weights >= 0, not all 0", "refused when the optimum is a whole interval (cumulative weight exactly half)"],
     method="sort by coordinate; the first point where cumulative weight reaches half the total",
     latex=r"\min_{x,y}\sum_i w_i\,(|x-x_i| + |y-y_i|)", run=rectilinear_location, reference=_ref_rectilinear,
     wrong=lambda x, y, weights: (lambda c: {"total_weighted_distance": sum(w * (abs(c["x"] - a) + abs(c["y"] - b)) for a, b, w in zip(x, y, weights)) + 1.0})(
         center_of_gravity(x, y, weights)),
     rand=lambda rng: _rand_points(rng, 3, 9),
     triggers=["rectilinear location", "rectilinear distance location", "weighted median location", "city block location",
               "manhattan distance location"], ask="rectilinear distance location",
     examples=[{"x": [3, 8, 5, 1], "y": [2, 6, 9, 4], "weights": [5, 2, 3, 1]},
               {"x": [30, 90, 130, 60], "y": [120, 110, 130, 40], "weights": [2000, 1000, 1500, 2000]}],
     invalid=[{"x": [3, 8], "y": [2, 6], "weights": [0, 0]}])


def factor_rating(weights, scores):
    w = _vec("weights", weights, 1, 100, nonneg=True)
    s = _mat("scores", scores, 1, cols=len(w))
    tot = [sum(w[j] * row[j] for j in range(len(w))) for row in s]
    best = 0
    for i in range(len(tot)):
        if tot[i] > tot[best] + 1e-12 * max(1.0, abs(tot[best])):
            best = i
    return _rd({"weighted_scores": tot, "best_site": best + 1, "best_score": tot[best]})


def _ref_factor(weights, scores):
    import numpy as np
    tot = np.asarray(scores, float) @ np.asarray(weights, float)
    return {"weighted_scores": [float(v) for v in tot], "best_site": int(np.argmax(tot)) + 1, "best_score": float(tot.max())}


_add("ie_factor_rating", name="Factor-rating method for site selection", topic=f"{_LOC} / site selection",
     level="beginner",
     purpose="Weighted score of each candidate site (sum of factor weight x score) and the best site.",
     inputs=[_i("weights", "weight of each factor", "factor weights", "importance weights", type="vector"),
             _i("scores", "matrix: one row per site, one score per factor", "site scores", "ratings", type="matrix")],
     outputs=[_o("weighted_scores", "weighted score of each site"), _o("best_site", "site with the highest score"),
              _o("best_score", "highest weighted score")],
     primary="best_score", conditions=["weights >= 0", "one score per factor in every row", "ties go to the lower site number"],
     method="score_i = sum_j w_j s_ij", latex=r"S_i = \sum_j w_j\,s_{ij}", run=factor_rating, reference=_ref_factor,
     wrong=lambda weights, scores: {"best_score": max(sum(r) for r in scores) / len(weights) + 0.5},
     rand=lambda rng: (lambda k: {"weights": [round(v / 100, 2) for v in sorted(rng.sample(range(5, 60), k))],
                                  "scores": [[rng.randint(40, 100) for _ in range(k)] for _ in range(rng.randint(2, 5))]})(rng.randint(2, 5)),
     triggers=["factor rating", "factor-rating", "weighted factor rating", "site selection score"], ask="factor rating",
     examples=[{"weights": [0.25, 0.45, 0.3], "scores": [[70, 60, 90], [85, 80, 60]]},
               {"weights": [0.1, 0.2, 0.3, 0.4], "scores": [[80, 70, 60, 50], [60, 70, 80, 90], [75, 75, 75, 75]]}],
     invalid=[{"weights": [0.25, 0.45], "scores": [[70, 60, 90], [85, 80, 60]]}],
     weak="a definition: the reference is a NumPy matrix product")


def location_cost_volume(fixed_costs, variable_costs, volume):
    F = _vec("fixed_costs", fixed_costs, 2, 100, nonneg=True)
    V = _vec("variable_costs", variable_costs, 2, 100, nonneg=True)
    _need(len(F) == len(V), "fixed_costs and variable_costs must have the same number of values")
    q = _nonneg("volume", volume)
    tot = [F[i] + V[i] * q for i in range(len(F))]
    best = 0
    for i in range(len(tot)):
        if tot[i] < tot[best] - 1e-12 * max(1.0, abs(tot[best])):
            best = i
    return _rd({"total_costs": tot, "best_site": best + 1, "best_cost": tot[best]})


def _ref_loc_cost(fixed_costs, variable_costs, volume):
    import numpy as np
    tot = np.asarray(fixed_costs, float) + np.asarray(variable_costs, float) * volume
    return {"total_costs": [float(v) for v in tot], "best_site": int(np.argmin(tot)) + 1, "best_cost": float(tot.min())}


_add("ie_location_cost_volume", name="Locational cost-volume (break-even) analysis", topic=f"{_LOC} / site selection",
     level="intermediate",
     purpose="Total annual cost of each candidate site at an expected volume (fixed + variable x volume) and the "
             "cheapest site.",
     inputs=[_i("fixed_costs", "annual fixed cost of each site", "fixed cost", "fixed costs per site", type="vector"),
             _i("variable_costs", "variable cost per unit at each site", "variable cost", "unit costs", type="vector"),
             _i("volume", "expected annual volume", "expected volume", "annual volume", "units")],
     outputs=[_o("total_costs", "F_i + v_i Q for each site"), _o("best_site", "cheapest site"), _o("best_cost", "its total cost")],
     primary="best_cost", conditions=["costs >= 0", "volume >= 0", "ties go to the lower site number"],
     method="TC_i = F_i + v_i Q; the minimum wins", latex=r"TC_i(Q) = F_i + v_i\,Q", run=location_cost_volume,
     reference=_ref_loc_cost,
     wrong=lambda fixed_costs, variable_costs, volume: {"best_cost": min(fixed_costs) + min(variable_costs) * volume - 1.0},
     rand=lambda rng: (lambda k: {"fixed_costs": [_u(rng, 1e4, 5e5, 0) for _ in range(k)],
                                  "variable_costs": [_u(rng, 1, 100, 2) for _ in range(k)], "volume": _u(rng, 100, 20000, 0)})(rng.randint(2, 5)),
     triggers=["locational cost volume", "locational cost-volume", "location break even", "location break-even",
               "cost volume location"], ask="locational cost-volume analysis",
     examples=[{"fixed_costs": [30000, 60000, 110000], "variable_costs": [75, 45, 25], "volume": 2000},
               {"fixed_costs": [150000, 250000], "variable_costs": [62, 38], "volume": 3500}],
     invalid=[{"fixed_costs": [30000, 60000], "variable_costs": [75], "volume": 2000}],
     weak="a definition: the reference is NumPy arithmetic")


def break_even(fixed_cost, price, variable_cost, target_profit=None):
    F, p, v = _nonneg("fixed_cost", fixed_cost), _pos("price", price), _nonneg("variable_cost", variable_cost)
    _need(p > v, "the price must exceed the variable cost per unit")
    cm = p - v
    out = {"break_even_units": F / cm, "break_even_revenue": p * F / cm, "contribution_margin": cm,
           "contribution_margin_ratio": cm / p}
    if target_profit is not None:
        out["units_for_target_profit"] = (F + _fin("target_profit", target_profit)) / cm
    return _rd(out)


def _ref_break_even(fixed_cost, price, variable_cost, target_profit=None):
    prof = lambda q, goal: price * q - variable_cost * q - fixed_cost - goal
    hi = 10 * (fixed_cost + abs(target_profit or 0) + 1) / (price - variable_cost) + 10
    q = _brentq(lambda s: prof(s, 0.0), -hi, hi)
    out = {"break_even_units": q, "break_even_revenue": price * q}
    if target_profit is not None:
        out["units_for_target_profit"] = _brentq(lambda s: prof(s, target_profit), -hi, hi)
    return out


_add("ie_break_even", name="Break-even point", topic="break-even analysis", level="beginner",
     purpose="Volume (and revenue) at which revenue equals fixed plus variable cost, and the volume for a target profit.",
     inputs=[_i("fixed_cost", "fixed cost per period", "fixed costs", "f", "total fixed cost"),
             _i("price", "selling price per unit", "selling price", "p", "price per unit"),
             _i("variable_cost", "variable cost per unit", "variable cost per unit", "v", "unit variable cost"),
             _i("target_profit", "target profit (optional)", "desired profit", "profit target", required=False)],
     outputs=[_o("break_even_units", "F / (p - v)"), _o("break_even_revenue", "p F / (p - v)"),
              _o("contribution_margin", "p - v"), _o("contribution_margin_ratio", "(p - v)/p"),
              _o("units_for_target_profit", "(F + target)/(p - v)")],
     primary="break_even_units", conditions=["fixed_cost >= 0", "price > variable_cost >= 0"],
     method="Q_BE = F / (p - v)", latex=r"Q_{BE} = \frac{F}{p - v},\quad R_{BE} = p\,Q_{BE}", run=break_even,
     reference=_ref_break_even,
     wrong=lambda fixed_cost, price, variable_cost, target_profit=None: {"break_even_units": fixed_cost / price},
     rand=lambda rng: (lambda v: _drop_none({"fixed_cost": _u(rng, 1000, 1e6, 0), "variable_cost": v, "price": round(v * rng.uniform(1.1, 3) + 1, 2),
                                             "target_profit": rng.choice([None, _u(rng, 1000, 2e5, 0)])}))(_u(rng, 1, 200, 2)),
     triggers=["break even point", "break-even point", "break even", "break-even", "breakeven"], ask="break-even point",
     examples=[{"fixed_cost": 10000, "price": 25, "variable_cost": 15}, {"fixed_cost": 50000, "price": 8, "variable_cost": 3, "target_profit": 20000}],
     invalid=[{"fixed_cost": 10000, "price": 15, "variable_cost": 25}])


def multiproduct_break_even(fixed_cost, prices, variable_costs, sales_mix):
    F = _nonneg("fixed_cost", fixed_cost)
    p = _vec("prices", prices, 1, 200, positive=True)
    v = _vec("variable_costs", variable_costs, 1, 200, nonneg=True)
    w = _vec("sales_mix", sales_mix, 1, 200, nonneg=True)
    _need(len(p) == len(v) == len(w), "prices, variable_costs and sales_mix must have the same number of values")
    _need(abs(sum(w) - 1) <= 1e-9, "the sales mix fractions must add up to 1")
    cmr = sum((1 - v[i] / p[i]) * w[i] for i in range(len(p)))
    _need(cmr > 0, "the weighted contribution must be positive")
    R = F / cmr
    return _rd({"break_even_revenue": R, "weighted_contribution_ratio": cmr, "revenue_by_product": [R * x for x in w],
                "units_by_product": [R * w[i] / p[i] for i in range(len(p))]})


def _ref_multi_be(fixed_cost, prices, variable_costs, sales_mix):
    import numpy as np
    p, v, w = (np.asarray(a, float) for a in (prices, variable_costs, sales_mix))
    profit = lambda R: float(((R * w) * (1 - v / p)).sum()) - fixed_cost
    R = _brentq(profit, 0.0, 1e3 * (fixed_cost + 1) / float(((1 - v / p) * w).sum()))
    return {"break_even_revenue": R, "revenue_by_product": [float(x) for x in R * w], "units_by_product": [float(x) for x in R * w / p]}


def _rand_mix(rng):
    k = rng.randint(2, 5)
    ws = [rng.randint(1, 9) for _ in range(k)]
    s = sum(ws)
    mix = [round(x / s, 4) for x in ws[:-1]]
    mix.append(round(1 - sum(mix), 4))
    v = [_u(rng, 0.5, 20, 2) for _ in range(k)]
    return {"fixed_cost": _u(rng, 1000, 5e5, 0), "prices": [round(x * rng.uniform(1.2, 4), 2) for x in v],
            "variable_costs": v, "sales_mix": mix}


_add("ie_multiproduct_break_even", name="Multiproduct break-even (sales-mix weighted)", topic="break-even analysis",
     level="intermediate",
     purpose="Break-even revenue of several products sold in a fixed revenue mix: F / sum (1 - v_i/p_i) w_i.",
     inputs=[_i("fixed_cost", "fixed cost per period", "fixed costs", "f", "total fixed cost"),
             _i("prices", "selling price of each product", "selling prices", "price per unit", type="vector"),
             _i("variable_costs", "variable cost per unit of each product", "variable cost per unit", "unit variable costs", type="vector"),
             _i("sales_mix", "share of revenue of each product (sum 1)", "revenue mix", "mix", "sales shares", type="vector")],
     outputs=[_o("break_even_revenue", "F / sum (1 - v_i/p_i) w_i"), _o("weighted_contribution_ratio", "sum (1 - v_i/p_i) w_i"),
              _o("revenue_by_product", "break-even revenue of each product"), _o("units_by_product", "break-even units of each product")],
     primary="break_even_revenue", conditions=["fixed_cost >= 0", "prices > 0", "mix fractions of revenue adding to 1"],
     method="BEP($) = F / sum [(1 - V_i/P_i) W_i]", latex=r"BEP_{\$} = \frac{F}{\sum_i\left(1 - \frac{V_i}{P_i}\right)W_i}",
     run=multiproduct_break_even, reference=_ref_multi_be,
     wrong=lambda fixed_cost, prices, variable_costs, sales_mix: {"break_even_revenue": fixed_cost / sum(1 - v / p for p, v in zip(prices, variable_costs))},
     rand=_rand_mix,
     triggers=["multiproduct break even", "multiproduct break-even", "multi-product break-even", "multi product break even",
               "sales mix break even", "sales-mix break-even"], ask="multiproduct break-even",
     examples=[{"fixed_cost": 3000, "prices": [2.95, 1.25, 0.8], "variable_costs": [1.25, 0.3, 0.2], "sales_mix": [0.5, 0.3, 0.2]},
               {"fixed_cost": 120000, "prices": [50, 80], "variable_costs": [30, 40], "sales_mix": [0.6, 0.4]}],
     invalid=[{"fixed_cost": 3000, "prices": [2.95, 1.25], "variable_costs": [1.25, 0.3], "sales_mix": [0.5, 0.3]}])


# =====================================================================================================================
# WORK MEASUREMENT AND PRODUCTIVITY
# =====================================================================================================================
_WM = "work measurement"


def standard_time(observed_times, rating, allowance, basis):
    x = _vec("observed_times", observed_times, 1, 10000, positive=True)
    r, a = _pos("rating", rating), _prob("allowance", allowance, -1e-300, 1.0)
    _need(basis in ("job", "shift"), "basis is 'job' (allowance as a fraction of normal time) or 'shift' (of the workday)")
    mean = sum(x) / len(x)
    nt = mean * r
    st = nt * (1 + a) if basis == "job" else nt / (1 - a)
    return _rd({"standard_time": st, "normal_time": nt, "mean_observed_time": mean})


def _ref_standard(observed_times, rating, allowance, basis):
    from fractions import Fraction
    F = lambda v: Fraction(repr(float(v)))
    mean = sum(F(v) for v in observed_times) / len(observed_times)
    nt = mean * F(rating)
    st = nt * (1 + F(allowance)) if basis == "job" else nt / (1 - F(allowance))
    return {"standard_time": float(st), "normal_time": float(nt), "mean_observed_time": float(mean)}


_add("ie_standard_time", name="Standard time from a time study", topic=f"{_WM} / time study", level="beginner",
     purpose="Normal time (mean observed time x performance rating) and standard time with a personal, fatigue and "
             "delay allowance.",
     inputs=[_i("observed_times", "observed cycle times", "observed times", "cycle times", "observations", type="vector"),
             _i("rating", "performance rating (decimal; 110% = 1.1)", "performance rating", "rating factor", "pace rating"),
             _i("allowance", "allowance fraction (decimal; 15% = 0.15)", "allowance factor", "pfd allowance", "allowances"),
             _i("basis", "job: ST = NT (1 + A); shift: ST = NT / (1 - A), A a share of the workday", "allowance basis",
                type="choice", choices=["job", "shift"], syn={"workday": "shift", "day": "shift", "time": "job"})],
     outputs=[_o("standard_time", "standard time"), _o("normal_time", "mean x rating"), _o("mean_observed_time", "mean observed time")],
     primary="standard_time", conditions=["observed times > 0", "rating > 0", "0 <= allowance < 1",
                                          "the allowance basis must be stated"],
     method="NT = mean x rating; ST = NT (1 + A) on a job basis or NT / (1 - A) on a workday basis",
     latex=r"NT = \bar t \times PR,\quad ST = NT(1+A)\ \text{or}\ \frac{NT}{1-A}", run=standard_time, reference=_ref_standard,
     wrong=lambda observed_times, rating, allowance, basis: {"standard_time": sum(observed_times) / len(observed_times) * (1 + allowance)},
     rand=lambda rng: {"observed_times": [_u(rng, 0.5, 10, 2) for _ in range(rng.randint(3, 12))], "rating": rng.choice([0.85, 0.9, 1.05, 1.1, 1.2]),
                       "allowance": rng.choice([0.1, 0.12, 0.15, 0.2]), "basis": rng.choice(["job", "shift"])},
     triggers=["standard time", "time study", "normal time"], ask="standard time",
     examples=[{"observed_times": [4.2, 4.5, 4.0, 4.3, 4.5], "rating": 1.1, "allowance": 0.15, "basis": "job"},
               {"observed_times": [12, 11, 13, 12], "rating": 0.95, "allowance": 0.12, "basis": "shift"}],
     invalid=[{"observed_times": [4.2, 4.5], "rating": 1.1, "allowance": 1.5, "basis": "job"}],
     weak="a definition: the reference recomputes it exactly with fractions")


def time_study_size(mean, sd, accuracy, confidence):
    m, s = _pos("mean", mean), _pos("sd", sd)
    h, c = _prob("accuracy", accuracy), _prob("confidence", confidence)
    z = _zinv(1 - (1 - c) / 2)
    x = (z * s / (h * m)) ** 2
    return _rd({"sample_size": _ceil(x), "exact": x, "z": z})


def _ref_ts_size(mean, sd, accuracy, confidence):
    from scipy.stats import norm
    z = float(norm.ppf((1 + confidence) / 2))
    x = (z * sd / (accuracy * mean)) ** 2
    return {"sample_size": int(math.ceil(x - 1e-9 * x)), "exact": x, "z": z}


_add("ie_time_study_sample_size", name="Number of cycles to time in a time study", topic=f"{_WM} / time study",
     level="intermediate",
     purpose="Observations needed so the mean cycle time is within +/- h (a fraction of the mean) at a confidence level: "
             "n = (z s / (h x-bar))^2.",
     inputs=[_i("mean", "mean of the pilot observations", "sample mean", "average time", "mean time"),
             _i("sd", "standard deviation of the pilot observations", "standard deviation", "sample standard deviation", "s"),
             _i("accuracy", "desired accuracy as a fraction of the mean (decimal; 5% = 0.05)", "desired accuracy", "precision", "h"),
             _i("confidence", "confidence level (decimal)", "confidence level")],
     outputs=[_o("sample_size", "observations needed (rounded up)"), _o("exact", "(z s / (h x-bar))^2"), _o("z", "two-sided z")],
     primary="exact", conditions=["mean, sd > 0", "0 < accuracy < 1", "0 < confidence < 1", "normal approximation"],
     method="n = (z s / (h x-bar))^2, z for a two-sided confidence", latex=r"n = \left(\frac{z\,s}{h\,\bar x}\right)^2",
     run=time_study_size, reference=_ref_ts_size,
     wrong=lambda mean, sd, accuracy, confidence: {"exact": (_zinv(confidence) * sd / (accuracy * mean)) ** 2},
     rand=lambda rng: {"mean": _u(rng, 0.5, 20, 2), "sd": _u(rng, 0.05, 3, 3), "accuracy": rng.choice([0.02, 0.05, 0.1]),
                       "confidence": rng.choice([0.9, 0.95, 0.99])},
     triggers=["time study sample size", "number of observations for a time study", "cycles to time",
               "observations needed in a time study"], ask="time study sample size",
     examples=[{"mean": 3.0, "sd": 0.5, "accuracy": 0.05, "confidence": 0.95}, {"mean": 12, "sd": 2.2, "accuracy": 0.1, "confidence": 0.99}],
     invalid=[{"mean": 3.0, "sd": 0.5, "accuracy": 1.5, "confidence": 0.95}])


def work_sampling_size(proportion, accuracy, confidence):
    p, h, c = _prob("proportion", proportion), _prob("accuracy", accuracy), _prob("confidence", confidence)
    z = _zinv(1 - (1 - c) / 2)
    x = z * z * p * (1 - p) / (h * h)
    return _rd({"sample_size": _ceil(x), "exact": x, "z": z})


def _ref_ws_size(proportion, accuracy, confidence):
    from scipy.stats import norm
    z = float(norm.ppf((1 + confidence) / 2))
    x = z * z * proportion * (1 - proportion) / accuracy ** 2
    return {"sample_size": int(math.ceil(x - 1e-9 * x)), "exact": x, "z": z}


_add("ie_work_sampling_sample_size", name="Work sampling sample size", topic=f"{_WM} / work sampling",
     level="intermediate",
     purpose="Random observations needed to estimate the fraction of time in an activity within +/- h (absolute) at a "
             "confidence level: n = z^2 p (1 - p) / h^2.",
     inputs=[_i("proportion", "estimated fraction of time in the activity (decimal)", "estimated proportion", "p", "idle fraction"),
             _i("accuracy", "absolute accuracy (decimal; 0.05 = +/- 5 points)", "desired accuracy", "h", "margin of error"),
             _i("confidence", "confidence level (decimal)", "confidence level")],
     outputs=[_o("sample_size", "observations needed (rounded up)"), _o("exact", "z^2 p (1 - p) / h^2"), _o("z", "two-sided z")],
     primary="exact", conditions=["0 < p < 1", "0 < h < 1", "0 < confidence < 1"],
     method="n = z^2 p (1 - p) / h^2", latex=r"n = \frac{z^2\,p(1-p)}{h^2}", run=work_sampling_size, reference=_ref_ws_size,
     wrong=lambda proportion, accuracy, confidence: {"exact": _zinv((1 + confidence) / 2) * proportion * (1 - proportion) / accuracy ** 2},
     rand=lambda rng: {"proportion": _u(rng, 0.05, 0.6, 2), "accuracy": rng.choice([0.01, 0.02, 0.03, 0.05]),
                       "confidence": rng.choice([0.9, 0.95, 0.99])},
     triggers=["work sampling sample size", "work sampling", "number of work sampling observations"], ask="work sampling sample size",
     examples=[{"proportion": 0.2, "accuracy": 0.05, "confidence": 0.95}, {"proportion": 0.1, "accuracy": 0.02, "confidence": 0.9}],
     invalid=[{"proportion": 1.2, "accuracy": 0.05, "confidence": 0.95}])


def oee(planned_time, downtime, ideal_cycle_time, total_count, good_count):
    T, dt = _pos("planned_time", planned_time), _nonneg("downtime", downtime)
    ct, n, g = _pos("ideal_cycle_time", ideal_cycle_time), _pos("total_count", total_count), _nonneg("good_count", good_count)
    _need(dt < T, "downtime must be less than the planned time")
    _need(g <= n, "good count cannot exceed the total count")
    run_t = T - dt
    perf = ct * n / run_t
    _need(perf <= 1 + 1e-9, "the count exceeds what the ideal cycle time allows in the run time")
    a, q = run_t / T, g / n
    return _rd({"oee": a * perf * q, "availability": a, "performance": perf, "quality": q, "run_time": run_t})


def _ref_oee(planned_time, downtime, ideal_cycle_time, total_count, good_count):
    from fractions import Fraction
    F = lambda v: Fraction(repr(float(v)))
    good_time = F(good_count) * F(ideal_cycle_time)        # OEE = fully productive time / planned time
    return {"oee": float(good_time / F(planned_time)), "availability": float((F(planned_time) - F(downtime)) / F(planned_time)),
            "quality": float(F(good_count) / F(total_count))}


def _rand_oee(rng):
    T = rng.choice([420, 450, 480, 960])
    dt = rng.randint(0, 120)
    ct = _u(rng, 0.2, 3, 2)
    n = int((T - dt) / ct * rng.uniform(0.6, 0.98))
    return {"planned_time": T, "downtime": dt, "ideal_cycle_time": ct, "total_count": max(1, n), "good_count": int(max(1, n) * rng.uniform(0.85, 1))}


_add("ie_oee", name="Overall equipment effectiveness (OEE)", topic=f"{_WM} / productivity", level="beginner",
     purpose="OEE = availability x performance x quality from planned time, downtime, ideal cycle time and counts.",
     inputs=[_i("planned_time", "planned production time", "planned production time", "scheduled time", "shift time"),
             _i("downtime", "stop time within the planned time", "stop time", "unplanned downtime", "down time"),
             _i("ideal_cycle_time", "ideal time per unit (same time unit)", "ideal cycle time", "ideal time per unit"),
             _i("total_count", "units produced", "total units", "units produced", "total pieces"),
             _i("good_count", "good units", "good units", "good pieces")],
     outputs=[_o("oee", "availability x performance x quality"), _o("availability", "run time / planned time"),
              _o("performance", "ideal cycle time x total count / run time"), _o("quality", "good / total"),
              _o("run_time", "planned time - downtime")],
     primary="oee", conditions=["downtime < planned time", "good <= total", "performance <= 1"],
     method="A = (T - D)/T, P = ct n / (T - D), Q = good/n, OEE = A P Q (= good x ct / T)",
     latex=r"OEE = A \times P \times Q = \frac{\text{good} \times t_{ideal}}{T_{planned}}", run=oee, reference=_ref_oee,
     wrong=lambda planned_time, downtime, ideal_cycle_time, total_count, good_count: {"oee": (planned_time - downtime) / planned_time * good_count / total_count},
     rand=_rand_oee,
     triggers=["overall equipment effectiveness", "oee"], ask="overall equipment effectiveness",
     examples=[{"planned_time": 420, "downtime": 47, "ideal_cycle_time": 1.0, "total_count": 300, "good_count": 285},
               {"planned_time": 480, "downtime": 60, "ideal_cycle_time": 0.5, "total_count": 700, "good_count": 680}],
     invalid=[{"planned_time": 420, "downtime": 47, "ideal_cycle_time": 1.0, "total_count": 300, "good_count": 385}],
     weak="the reference uses the identity OEE = good units x ideal cycle time / planned time in exact fractions")


def capacity_measures(design_capacity, effective_capacity, actual_output):
    d, e, a = _pos("design_capacity", design_capacity), _pos("effective_capacity", effective_capacity), _nonneg("actual_output", actual_output)
    _need(e <= d, "effective capacity cannot exceed design capacity")
    return _rd({"utilization": a / d, "efficiency": a / e, "effective_share": e / d})


def _ref_capacity(design_capacity, effective_capacity, actual_output):
    from fractions import Fraction
    F = lambda v: Fraction(repr(float(v)))
    return {"utilization": float(F(actual_output) / F(design_capacity)), "efficiency": float(F(actual_output) / F(effective_capacity)),
            "effective_share": float(F(effective_capacity) / F(design_capacity))}


_add("ie_capacity_utilization", name="Capacity utilisation and efficiency", topic=f"{_WM} / productivity", level="beginner",
     purpose="Utilisation (actual output / design capacity) and efficiency (actual output / effective capacity).",
     inputs=[_i("design_capacity", "design capacity", "design capacity", "maximum capacity"),
             _i("effective_capacity", "effective capacity", "effective capacity", "expected capacity"),
             _i("actual_output", "actual output", "actual output", "output", "production")],
     outputs=[_o("utilization", "actual / design"), _o("efficiency", "actual / effective"), _o("effective_share", "effective / design")],
     primary="utilization", conditions=["capacities > 0", "effective <= design", "same units and period"],
     method="utilisation = actual/design, efficiency = actual/effective",
     latex=r"U = \frac{\text{actual}}{\text{design}},\quad E = \frac{\text{actual}}{\text{effective}}", run=capacity_measures,
     reference=_ref_capacity,
     wrong=lambda design_capacity, effective_capacity, actual_output: {"utilization": actual_output / effective_capacity, "efficiency": actual_output / design_capacity},
     rand=lambda rng: (lambda d: (lambda e: {"design_capacity": d, "effective_capacity": e, "actual_output": round(e * rng.uniform(0.5, 1), 0)})(
         round(d * rng.uniform(0.6, 0.95), 0)))(_u(rng, 100, 10000, 0)),
     triggers=["capacity utilization", "capacity utilisation", "utilization and efficiency", "utilisation and efficiency"],
     ask="capacity utilization and efficiency",
     examples=[{"design_capacity": 1200, "effective_capacity": 1000, "actual_output": 800},
               {"design_capacity": 50, "effective_capacity": 40, "actual_output": 36}],
     invalid=[{"design_capacity": 1000, "effective_capacity": 1200, "actual_output": 800}],
     weak="a definition: the reference recomputes it exactly with fractions")


# levels revised once the whole catalogue was in place (beginner / intermediate / advanced / expert ~ 30/35/23/12 %)
LEVELS.update({
    "ie_abc_classification": "intermediate", "ie_rolled_throughput_yield": "intermediate", "ie_np_chart_limits": "intermediate",
    "ie_eoq_quantity_discount": "advanced", "ie_order_up_to_level": "advanced", "ie_queue_server_cost": "advanced",
    "ie_multiproduct_break_even": "advanced", "ie_time_study_sample_size": "advanced",
    "ie_mg1_queue": "expert", "ie_weibull_median_rank_regression": "expert"})
