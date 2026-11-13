"""industrial-engineering-ai999x — offline industrial engineering and supply chain answers with their formula, steps and verification.

    import industrial_engineering_ai999x as m
    m.ask("Economic order quantity with annual demand = 1200, ordering cost = 100, holding cost = 6.").show()
    m.ask("M/M/c queue with arrival rate = 10, service rate = 4, servers = 3.").show()
    m.solve("ie_johnson_two_machine", machine1_times=[5, 2, 1, 7, 6], machine2_times=[2, 6, 2, 5, 3]).show()
    m.capabilities()                   # every certified capability

94 capabilities: inventory policies (EOQ with backorders and quantity discounts, EPQ, reorder point and safety stock,
periodic review, (R, Q) fill rate, newsvendor with normal, uniform, Poisson and discrete demand, ABC, kanban),
statistical process control (X-bar/R/S, p, np, c and u charts, Cp/Cpk/Pp/Ppk/Cpm, ARL, EWMA, CUSUM, DPMO, acceptance
sampling), reliability (series, parallel, k-out-of-n, standby, exponential and Weibull life, availability, MTBF
confidence bounds), queueing (M/M/1, M/M/c, M/D/1, M/G/1, M/M/1/K, Erlang C staffing, Little's law), scheduling (SPT,
EDD, dispatching rules, Johnson's rule, Moore-Hodgson, flow shop makespan), forecasting (moving averages, exponential
smoothing, Holt, trend, seasonal indices, regression, MAD/MSE/MAPE/tracking signal), line balancing (takt time,
efficiency, largest candidate rule, ranked positional weight), learning curves, facility location, break-even and work
measurement. Questions are read in one documented form (a capability phrase, then `name = value` pairs); a question
that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.6.0"
_P = Product("industrial_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]
