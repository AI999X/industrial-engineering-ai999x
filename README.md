# industrial-engineering-ai999x: offline, verified industrial engineering and supply chain solver with step-by-step answers

[![tests](https://github.com/AI999X/industrial-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/industrial-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/industrial-engineering-ai999x)](https://pypi.org/project/industrial-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/industrial-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/industrial-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/industrial-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Engineering** · Tasks: Question Answering

industrial-engineering-ai999x is an offline, deterministic industrial engineering and supply chain solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers industrial engineering and supply chain questions with verified results from 98 certified capabilities, gives 678 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**industrial-engineering-ai999x** reads a question in plain English (or a typed call), picks a certified industrial engineering and supply chain capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** industrial-engineering-ai999x comes as the AI999X desktop app and as the command-line program `industrial-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Industrial Engineering and Supply Chain desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-industrial-engineering-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-industrial-engineering-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`industrial-engineering`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install industrial-engineering-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Industrial Engineering and Supply Chain** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Industrial Engineering and Supply Chain among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-industrial-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-industrial-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-industrial-engineering-1.5.0-linux-x86_64/ai999x-industrial-engineering
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
industrial-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
industrial-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

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

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

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

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | industrial-engineering batch

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

_What this means:_ The break-even volume is 1000 units, generating revenue of 25000, with a contribution margin of 10.

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

_What this means:_ This calculation determines the total sales revenue required to cover all fixed costs when multiple products are sold in a specific mix. By dividing the period's fixed cost, which is 3000, by the sum of each product's contribution margin percentage weighted by its share of total revenue, this gives the overall break-even point. The resulting weighted contribution ratio is 0.666136, leading to a necessary total revenue of 4503.59. This figure represents the exact sales level where total revenues equal total costs, ensuring neither profit nor loss occurs across the entire product portfolio.

### After-tax cash flow with depreciation and recapture

Taxable income, taxes and after-tax cash flow of each year from the before-tax cash flows and the depreciation charges, and with the first cost and a sale price the book value, the gain (depreciation recapture) or loss on the sale and its tax, and the full after-tax cash-flow series.

Command line:

```console
$ industrial-engineering solve ie_after_tax_cash_flow "before_tax_cash_flows=[5000, 5000, 5000]" "depreciation=[3000, 3000, 3000]" tax_rate=0.25 investment=9000 sale_price=1000
taxable_income = [2000.0,2000.0,2000.0]
taxes = [500.0,500.0,500.0]
after_tax_cash_flows = [4500.0,4500.0,4500.0]
total_tax = 1500
book_value_end = 0
gain_on_sale = 1000
tax_on_sale = 250
cash_flows_with_investment = [-9000.0,4500.0,4500.0,5250.0]
  by: After-tax cash flow with depreciation and recapture  (TI = BTCF - D; taxes = t TI; ATCF = BTCF - taxes; at the sale: + S - t (S - BV))
  tax_rate = 0.25
  investment = 9000
  sale_price = 1000
  assumes: 0 <= tax rate < 1; one depreciation value per year, each >= 0; sale price <= first cost (no capital gain); a negative taxable income gives a tax credit; the gain or loss on the sale is taxed at the same rate
```

Python:

```python
import industrial_engineering_ai999x as m

a = m.solve("ie_after_tax_cash_flow", before_tax_cash_flows=[5000, 5000, 5000], depreciation=[3000, 3000, 3000], tax_rate="0.25", investment="9000", sale_price="1000")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#cap-ie_after_tax_cash_flow) and press **Compute in your browser**.

### Depreciation schedule (straight line, declining balance, sum of years' digits, MACRS)

Depreciation charge and end-of-year book value for every year of an asset's life by straight line, declining balance (factor/n of the book value, never below salvage), sum of years' digits or MACRS (half-year convention, salvage ignored), with one year picked out on request.

Command line:

```console
$ industrial-engineering solve ie_depreciation_schedule cost=10000 salvage=2000 life=5 method=straight-line
depreciation = [1600.0,1600.0,1600.0,1600.0,1600.0]
book_value = [8400.0,6800.0,5200.0,3600.0,2000.0]
total_depreciation = 8000
  by: Depreciation schedule (straight line, declining balance, sum of years' digits, MACRS)  (SL: (P - S)/n; DB: d_t = min(f/n BV_{t-1}, BV_{t-1} - S); SYD: (P - S)(n - t + 1)/(n(n + 1)/2); MACRS: P x table rate)
  cost = 10000
  salvage = 2000
  method = straight-line
  assumes: cost > 0; 0 <= salvage < cost; life a whole number of years; MACRS: recovery period 3, 5, 7, 10, 15 or 20, no salvage; declining balance: no switch to straight line
```

Python:

```python
import industrial_engineering_ai999x as m

a = m.solve("ie_depreciation_schedule", cost="10000", salvage="2000", life="5", method="straight-line")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/industrial-engineering-ai999x/#cap-ie_depreciation_schedule) and press **Compute in your browser**.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/industrial-engineering-ai999x/blob/main/docs/industrial_engineering_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/industrial-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified industrial engineering and supply chain capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 678 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `industrial-engineering` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** inventory policies, statistical process control, reliability, queueing, scheduling, forecasting, line balancing, learning curves and facility location.

## Model overview

| Property | Value |
|---|---|
| Package | `industrial-engineering-ai999x` 1.6.0 (import `industrial_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic industrial engineering and supply chain engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 98 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 678 documented answers, identical on every tested platform |
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

The Python package also installs a Python command, `industrial-engineering-ai999x "<question>"`; for the native program use `industrial-engineering` (above). The outputs above were produced by running this exact code on industrial-engineering-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

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

**Economic order quantity (EOQ)**

![Economic order quantity (EOQ), computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-eoq-q-vs-demand.svg)

_Q (economic order quantity Q*) against demand (demand per year (units)), with holding_cost = 6, order_cost = 100. 31 points computed by industrial-engineering-ai999x; marked: demand = 1200, Q = 200._

**M/M/c queue (Erlang C)**

![M/M/c queue (Erlang C), computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-mmc-queue-lq-vs-arrival-rate.svg)

_Lq (mean number waiting) against arrival_rate (mean arrival rate lambda (per time unit)), with servers = 3, service_rate = 4. 21 points computed by industrial-engineering-ai999x; marked: arrival_rate = 10, Lq = 3.51124._

**Johnson's rule (two machines)**

![Johnson's rule (two machines), computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-johnson-two-machine-sequence-vs-k.svg)

_sequence (job order) for each index, as returned by industrial-engineering-ai999x for its example (5 values)._

**Depreciation schedule (straight line, declining balance, sum of years' digits, MACRS)**

![Depreciation schedule (straight line, declining balance, sum of years' digits, MACRS), computed by industrial-engineering-ai999x](https://huggingface.co/AI999X/industrial-engineering-ai999x/resolve/main/figures/ie-depreciation-schedule-book-value-vs-k.svg)

_book_value (book value at the end of each year) for each index, as returned by industrial-engineering-ai999x for its example (5 values)._

## Capabilities

98 certified procedures. A few of them:

- **Procedures:** (R, Q) reorder point for a target fill rate; ABC inventory classification; After-tax cash flow with depreciation and recapture; Annual inventory cost of an order quantity; Associative forecast by simple linear regression; Average run length of a Shewhart X-bar chart; Break-even point; Capacity utilisation and efficiency; Centre-of-gravity location; Cold standby redundancy; Confidence bounds on MTBF (chi-square); Cost-optimal number of servers; Cumulative-average learning curve (Wright); DPMO and sigma level; Depreciation schedule (straight line, declining balance, sum of years' digits, MACRS); EOQ with all-units quantity discounts; EOQ with planned backorders; EWMA chart control limits; Earliest due date (EDD) sequence; Earned value analysis (CV, SV, CPI, SPI, EAC); Economic order quantity (EOQ); Economic production quantity (EPQ); Exponential (constant failure rate) reliability; Factor-rating method for site selection …

`industrial_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified industrial engineering and supply chain capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (inventory policies, statistical process control, reliability, queueing, scheduling, forecasting, line balancing, learning curves and facility location); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{industrial_engineering_ai999x_1_6_0,
  title  = {industrial-engineering-ai999x: an offline, verified industrial engineering and supply chain solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/industrial-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/industrial-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

industrial-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/industrial-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/industrial-engineering-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `industrial-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/industrial-engineering`) and its data (`rust/models/industrial_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p industrial-engineering-ai999x
./target/release/industrial-engineering --help
cargo test -p industrial-engineering-ai999x
```
