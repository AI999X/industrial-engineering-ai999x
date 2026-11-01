# industrial-engineering-ai999x — offline, verified industrial engineering and supply chain solver with step-by-step answers

[![tests](https://github.com/AI999X/industrial-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/industrial-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/industrial-engineering-ai999x)](https://pypi.org/project/industrial-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/industrial-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/industrial-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/industrial-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/industrial-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

industrial-engineering-ai999x is an offline, deterministic Python solver that answers industrial engineering and supply chain questions with verified results from 94 certified capabilities, gives 644 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**industrial-engineering-ai999x** is a compact, deterministic industrial engineering and supply chain engine for Python. It reads a question in plain English (or a typed call), picks a certified industrial engineering and supply chain capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified industrial engineering and supply chain capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 644 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
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

## Quickstart

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

The Python package also installs its own command: `industrial-engineering-ai999x "<question>"`, `industrial-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on industrial-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
