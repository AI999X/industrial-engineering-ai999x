# industrial-engineering-ai999x — offline, verified industrial engineering and supply chain solver with step-by-step answers

[![tests](https://github.com/AI999X/industrial-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/industrial-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/industrial-engineering-ai999x)](https://pypi.org/project/industrial-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/industrial-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/industrial-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/industrial-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

industrial-engineering-ai999x is an offline, deterministic Python solver that answers industrial engineering and supply chain questions with verified results from 94 certified capabilities, gives 644 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**industrial-engineering-ai999x** is a compact, deterministic industrial engineering and supply chain engine for Python. It reads a question in plain English (or a typed call), picks a certified industrial engineering and supply chain capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** industrial-engineering-ai999x comes as the AI999X desktop app and as the command-line program `industrial-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Industrial Engineering and Supply Chain desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-industrial-engineering-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`industrial-engineering`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install industrial-engineering-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Industrial Engineering and Supply Chain also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.0](https://github.com/AI999X/industrial-engineering-ai999x/releases/tag/v1.0.0) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Industrial Engineering and Supply Chain) | Windows x86-64 | `ai999x-industrial-engineering-1.2.0-windows-x86_64.zip` | 7.3 MB | [Hub](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/rust/ai999x-industrial-engineering-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/industrial-engineering-ai999x/releases/download/v1.0.0/ai999x-industrial-engineering-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-industrial-engineering-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-industrial-engineering.exe` (no installer) |
| Desktop app (Industrial Engineering and Supply Chain) | Linux x86-64 | `ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz` | 8.2 MB | [Hub](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/rust/ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/industrial-engineering-ai999x/releases/download/v1.0.0/ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-industrial-engineering` (X11 or Wayland desktop) |
| Command line (`industrial-engineering`) | Windows x86-64 | `ai999x-industrial-engineering-cli-1.2.0-windows-x86_64.zip` | 3.0 MB | [Hub](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/rust/ai999x-industrial-engineering-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/industrial-engineering-ai999x/releases/download/v1.0.0/ai999x-industrial-engineering-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `industrial-engineering.exe ask "<question>"` or `industrial-engineering.exe solve <capability> name=value` |
| Command line (`industrial-engineering`) | Linux x86-64 | `ai999x-industrial-engineering-cli-1.2.0-linux-x86_64.tar.gz` | 3.0 MB | [Hub](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/rust/ai999x-industrial-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/industrial-engineering-ai999x/releases/download/v1.0.0/ai999x-industrial-engineering-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./industrial-engineering ask "<question>"` or `./industrial-engineering solve <capability> name=value` |

The command-line archive holds `industrial-engineering` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Industrial Engineering and Supply Chain** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Industrial Engineering and Supply Chain among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-industrial-engineering-1.2.0-linux-x86_64.tar.gz
./ai999x-industrial-engineering-1.2.0-linux-x86_64/ai999x-industrial-engineering
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
industrial-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
industrial-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ industrial-engineering info ie_eoq
Economic order quantity (EOQ)
  Q* = sqrt(2 D S / H); TC = D S / Q + H Q / 2 = sqrt(2 D S H)
  in   demand   demand per year (units) []
  in   order_cost fixed cost per order or setup []
  in   holding_cost holding cost per unit per year []
  out  Q        economic order quantity Q* []
  out  orders_per_year orders per year D/Q* []
  out  cycle_time time between orders Q*/D (years) []
  out  ordering_cost annual ordering cost DS/Q* []
  out  holding_cost_per_year annual holding cost HQ*/2 []
  out  total_cost annual ordering plus holding cost []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ industrial-engineering solve ie_eoq demand=1200.0 order_cost=100.0 holding_cost=6.0
Q = 200
orders_per_year = 6
cycle_time = 0.1666666667
ordering_cost = 600
holding_cost_per_year = 600
total_cost = 1200
  by: Economic order quantity (EOQ)  (Q* = sqrt(2 D S / H); TC = D S / Q + H Q / 2 = sqrt(2 D S H))
  demand = 1200
  order_cost = 100
  holding_cost = 6
  assumes: demand > 0; order_cost > 0; holding_cost > 0 (per unit per year)
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | industrial-engineering batch

`cases.jsonl`:

```json
{"key": "ie_eoq", "inputs": {"demand": 1200.0, "order_cost": 100.0, "holding_cost": 6.0}}
{"key": "ie_mmc_queue", "inputs": {"arrival_rate": 10.0, "service_rate": 4.0, "servers": "3"}}
```

```console
$ industrial-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":200.0,"unit":"","solved_for":"Q","si":[["demand",1200.0],["order_cost",100.0],["holding_cost",6.0]],"values":[["Q",200.0,""],["orders_per_year",6.0,""],["cycle_time",0.166666666667,""],["ordering_cost",600.0,""],["holding_cost_per_year",600.0,""],["total_cost",1200.0,""]],"target":"Q","phrase":"economic order quantity Q*","assumptions":["demand > 0","order_cost > 0","holding_cost > 0 (per unit per year)"],"name":"Economic order quantity (EOQ)","equation":"Q* = sqrt(2 D S / H); TC = D S / Q + H Q / 2 = sqrt(2 D S H)"}
{"outcome":"SOLVED","message":"","value":3.51123595506,"unit":"","solved_for":"Lq","si":[["arrival_rate",10.0],["service_rate",4.0]],"values":[["utilization",0.833333333333,""],["p_wait",0.702247191011,""],["Lq",3.51123595506,""],["Wq",0.351123595506,""],["W",0.601123595506,""],["L",6.01123595506,""],["p0",0.0449438202247,""]],"target":"Lq","phrase":"rho = lambda/(c mu)","assumptions":["arrival_rate < servers x service_rate","1 <= servers <= 500"],"name":"M/M/c queue (Erlang C)","equation":"a = lambda/mu, P0 = [sum_{k<c} a^k/k! + a^c/(c!(1-rho))]^-1, C = a^c P0/(c!(1-rho)), Lq = C rho/(1-rho)"}
```

### In your browser

Open [Industrial Engineering and Supply Chain on ai999x.com](https://www.ai999x.com/models/industrial-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `industrial_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install industrial-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Break-even point

Volume (and revenue) at which revenue equals fixed plus variable cost, and the volume for a target profit.

Command line:

```console
$ industrial-engineering solve ie_break_even fixed_cost=10000 price=25 variable_cost=15
break_even_units = 1000
break_even_revenue = 25000
contribution_margin = 10
contribution_margin_ratio = 0.4
  by: Break-even point  (Q_BE = F / (p - v))
  fixed_cost = 10000
  price = 25
  variable_cost = 15
  assumes: fixed_cost >= 0; price > variable_cost >= 0
```

Python:

```python
import industrial_engineering_ai999x as m

a = m.solve("ie_break_even", fixed_cost="10000", price="25", variable_cost="15")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#cap-ie_break_even) and press **Compute in your browser**.

### Multiproduct break-even (sales-mix weighted)

Break-even revenue of several products sold in a fixed revenue mix: F / sum (1 - v_i/p_i) w_i.

Command line:

```console
$ industrial-engineering solve ie_multiproduct_break_even fixed_cost=3000 "prices=[2.95, 1.25, 0.8]" "variable_costs=[1.25, 0.3, 0.2]" "sales_mix=[0.5, 0.3, 0.2]"
break_even_revenue = 4503.587604
weighted_contribution_ratio = 0.6661355932
revenue_by_product = [2251.79380184,1351.07628111,900.717520737]
units_by_product = [763.319932828,1080.86102488,1125.89690092]
  by: Multiproduct break-even (sales-mix weighted)  (BEP($) = F / sum [(1 - V_i/P_i) W_i])
  fixed_cost = 3000
  assumes: fixed_cost >= 0; prices > 0; mix fractions of revenue adding to 1
```

Python:

```python
import industrial_engineering_ai999x as m

a = m.solve("ie_multiproduct_break_even", fixed_cost="3000", prices=[2.95, 1.25, 0.8], variable_costs=[1.25, 0.3, 0.2], sales_mix=[0.5, 0.3, 0.2])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#cap-ie_multiproduct_break_even) and press **Compute in your browser**.

### Centre-of-gravity location

Location of a facility at the load-weighted average of the demand points' coordinates.

Command line:

```console
$ industrial-engineering solve ie_center_of_gravity "x=[30, 90, 130, 60]" "y=[120, 110, 130, 40]" "weights=[2000, 1000, 1000, 2000]"
x = 66.66666667
y = 93.33333333
total_weight = 6000
  by: Centre-of-gravity location  (x* = sum w_i x_i / sum w_i, y* = sum w_i y_i / sum w_i)
  assumes: equal lengths; weights >= 0, not all 0
```

Python:

```python
import industrial_engineering_ai999x as m

a = m.solve("ie_center_of_gravity", x=[30, 90, 130, 60], y=[120, 110, 130, 40], weights=[2000, 1000, 1000, 2000])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#cap-ie_center_of_gravity) and press **Compute in your browser**.

### Factor-rating method for site selection

Weighted score of each candidate site (sum of factor weight x score) and the best site.

Command line:

```console
$ industrial-engineering solve ie_factor_rating "weights=[0.25, 0.45, 0.3]" "scores=[[70, 60, 90], [85, 80, 60]]"
weighted_scores = [71.5,75.25]
best_site = 2
best_score = 75.25
  by: Factor-rating method for site selection  (score_i = sum_j w_j s_ij)
  assumes: weights >= 0; one score per factor in every row; ties go to the lower site number
```

Python:

```python
import industrial_engineering_ai999x as m

a = m.solve("ie_factor_rating", weights=[0.25, 0.45, 0.3], scores=[[70, 60, 90], [85, 80, 60]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#cap-ie_factor_rating) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/industrial-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified industrial engineering and supply chain capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 644 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `industrial-engineering` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** inventory policies, statistical process control, reliability, queueing, scheduling, forecasting, line balancing, learning curves and facility location.

## Model overview

| Property | Value |
|---|---|
| Package | `industrial-engineering-ai999x` 1.0.0 (import `industrial_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic industrial engineering and supply chain engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 94 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 644 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

```bash
pip install industrial-engineering-ai999x
```

```python
import industrial_engineering_ai999x as industrial_engineering

industrial_engineering.ask("Economic order quantity with annual demand = 1200, ordering cost = 100, holding cost = 6.").show()
# -> SOLVED — Q = 200, orders_per_year = 6, cycle_time = 0.166667, ordering_cost = 600  (ie_eoq)
industrial_engineering.ask("M/M/c queue with arrival rate = 10, service rate = 4, servers = 3.").show()
# -> SOLVED — utilization = 0.833333, p_wait = 0.702247, Lq = 3.51124, Wq = 0.351124  (ie_mmc_queue)
industrial_engineering.ask("X-bar and R chart limits with grand mean = 50, average range = 2, subgroup size = 5.").show()
# -> SOLVED — xbar_ucl = 51.154, xbar_lcl = 48.846, center = 50, r_ucl = 4.228  (ie_xbar_r_limits)
industrial_engineering.ask("Newsvendor order quantity with selling price = 10, unit cost = 6, mean demand = 500, sd demand = 100, salvage value = 2.").show()
# -> SOLVED — Q = 500, critical_ratio = 0.5, z = 0, expected_sales = 460.106  (ie_newsvendor_normal)
industrial_engineering.solve("ie_johnson_two_machine", machine1_times=[5, 2, 1, 7, 6], machine2_times=[2, 6, 2, 5, 3]).show()
# -> SOLVED — sequence = [3, 2, 4, 5, 1], makespan = 23, idle_time_machine2 = 5  (ie_johnson_two_machine)
industrial_engineering.ask("Who created you?").message
# -> industrial-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs a Python command, `industrial-engineering-ai999x "<question>"`; for the native program use `industrial-engineering` (above). The outputs above were produced by running this exact code on industrial-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `industrial-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Economic order quantity (EOQ)** · _Inventory_

> Economic order quantity with annual demand = 1200, ordering cost = 100, holding cost = 6.

→ **Q = 200, orders_per_year = 6, cycle_time = 0.1666666667, ordering_cost = 600, holding_cost_per_year = 600, total_cost = 1200**

<details><summary>Steps</summary>

- Economic order quantity (EOQ): demand = 1200, order_cost = 100, holding_cost = 6 → Q = 200, orders_per_year = 6, cycle_time = 0.1666666667, ordering_cost = 600, holding_cost_per_year = 600, total_cost = 1200

</details>

**M/M/c queue (Erlang C)** · _Queueing_

> M/M/c queue with arrival rate = 10, service rate = 4, servers = 3.

→ **utilization = 0.8333333333, p_wait = 0.702247191, Lq = 3.511235955, Wq = 0.3511235955, W = 0.6011235955, L = 6.011235955, p0 = 0.04494382022**

<details><summary>Steps</summary>

- M/M/c queue (Erlang C): arrival_rate = 10, service_rate = 4, servers = 3 → utilization = 0.8333333333, p_wait = 0.702247191, Lq = 3.511235955, Wq = 0.3511235955, W = 0.6011235955, L = 6.011235955, p0 = 0.04494382022

</details>

**X-bar and R chart control limits** · _Statistical process control_

> X-bar and R chart limits with grand mean = 50, average range = 2, subgroup size = 5.

→ **xbar_ucl = 51.154, xbar_lcl = 48.846, center = 50, r_ucl = 4.228, r_lcl = 0, A2 = 0.577, D3 = 0, D4 = 2.114**

<details><summary>Steps</summary>

- X-bar and R chart control limits: grand_mean = 50, mean_range = 2, subgroup_size = 5 → xbar_ucl = 51.154, xbar_lcl = 48.846, center = 50, r_ucl = 4.228, r_lcl = 0, A2 = 0.577, D3 = 0, D4 = 2.114

</details>

**Newsvendor order quantity (normal demand)** · _Inventory_

> Newsvendor order quantity with selling price = 10, unit cost = 6, mean demand = 500, sd demand = 100, salvage value = 2.

→ **Q = 500, critical_ratio = 0.5, z = 0, expected_sales = 460.105772, expected_leftover = 39.89422804, expected_shortage = 39.89422804, expected_profit = 1680.846176**

<details><summary>Steps</summary>

- Newsvendor order quantity (normal demand): price = 10, cost = 6, mean_demand = 500, sd_demand = 100, salvage = 2 → Q = 500, critical_ratio = 0.5, z = 0, expected_sales = 460.105772, expected_leftover = 39.89422804, expected_shortage = 39.89422804, expected_profit = 1680.846176

</details>

**Johnson's rule (two machines)** · _Scheduling_

`m.solve("ie_johnson_two_machine", machine1_times=[5, 2, 1, 7, 6], machine2_times=[2, 6, 2, 5, 3])`

→ **sequence = [3, 2, 4, 5, 1], makespan = 23, idle_time_machine2 = 5**

<details><summary>Steps</summary>

- Johnson's rule (two machines): machine1_times = [5, 2, 1, 7, 6], machine2_times = [2, 6, 2, 5, 3] → sequence = [3, 2, 4, 5, 1], makespan = 23, idle_time_machine2 = 5

</details>

**Break-even point** · _Break-even analysis_

> Break-even point with fixed costs = 10000, selling price = 25, variable cost per unit = 15.

→ **break_even_units = 1000, break_even_revenue = 25000, contribution_margin = 10, contribution_margin_ratio = 0.4**

<details><summary>Steps</summary>

- Break-even point: fixed_cost = 10000, price = 25, variable_cost = 15 → break_even_units = 1000, break_even_revenue = 25000, contribution_margin = 10, contribution_margin_ratio = 0.4

</details>

## Figures

Every point was computed by `industrial-engineering-ai999x`.

**Simple exponential smoothing forecast**

![Simple exponential smoothing forecast — computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-exponential-smoothing-forecasts-vs-k.svg)

_forecasts (forecasts for periods 1, ..., n + 1) for each index, as returned by industrial-engineering-ai999x for its example (5 values)._

**Economic order quantity (EOQ)**

![Economic order quantity (EOQ) — computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-eoq-q-vs-demand.svg)

_Q (economic order quantity Q*) against demand (demand per year (units)), with holding_cost = 6, order_cost = 100. 31 points computed by industrial-engineering-ai999x; marked: demand = 1200, Q = 200._

**M/M/c queue (Erlang C)**

![M/M/c queue (Erlang C) — computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-mmc-queue-lq-vs-arrival-rate.svg)

_Lq (mean number waiting) against arrival_rate (mean arrival rate lambda (per time unit)), with servers = 3, service_rate = 4. 21 points computed by industrial-engineering-ai999x; marked: arrival_rate = 10, Lq = 3.51124._

**Johnson's rule (two machines)**

![Johnson's rule (two machines) — computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-johnson-two-machine-sequence-vs-k.svg)

_sequence (job order) for each index, as returned by industrial-engineering-ai999x for its example (5 values)._

## Capabilities

94 certified procedures. A few of them:

- **Procedures:** (R, Q) reorder point for a target fill rate; ABC inventory classification; Annual inventory cost of an order quantity; Associative forecast by simple linear regression; Average run length of a Shewhart X-bar chart; Break-even point; Capacity utilisation and efficiency; Centre-of-gravity location; Cold standby redundancy; Confidence bounds on MTBF (chi-square); Cost-optimal number of servers; Cumulative-average learning curve (Wright); DPMO and sigma level; EOQ with all-units quantity discounts; EOQ with planned backorders; EWMA chart control limits; Earliest due date (EDD) sequence; Economic order quantity (EOQ); Economic production quantity (EPQ); Exponential (constant failure rate) reliability; Factor-rating method for site selection; Failure rate and MTBF of a series system; Fit a learning curve to data (log-log regression); Forecast accuracy: MAD, MSE, RMSE, MAPE, bias, tracking signal …

`industrial_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified industrial engineering and supply chain capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (inventory policies, statistical process control, reliability, queueing, scheduling, forecasting, line balancing, learning curves and facility location); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{industrial_engineering_ai999x_1_0_0,
  title  = {industrial-engineering-ai999x: an offline, verified industrial engineering and supply chain solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/industrial-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/industrial-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

industrial-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/industrial-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/industrial-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/onnx).
