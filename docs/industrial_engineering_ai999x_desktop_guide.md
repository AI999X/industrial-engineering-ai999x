# Industrial Engineering and Supply Chain: the desktop app, user guide

**Industrial Engineering and Supply Chain**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Industrial Engineering and Supply Chain covers inventory policies (EOQ, reorder point, safety stock, newsvendor), statistical process control, reliability, queueing, scheduling, forecasting accuracy, line balancing, learning curves and facility location. Its desktop app holds all 98 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-industrial-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-industrial-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-industrial-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-industrial-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-industrial-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-industrial-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-industrial-engineering-1.6.5-linux-x86_64/ai999x-industrial-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Industrial Engineering and Supply Chain app, opened on Economic order quantity (EOQ) with its worked example.](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/hero.webp)

*The Industrial Engineering and Supply Chain app, opened on Economic order quantity (EOQ) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Inventory › Economic order quantity (EOQ) (or type part of its name in the search box). It opens with a worked example filled in: “Economic order quantity with annual demand = 1200, ordering cost = 100, carrying cost = 6.” The answer: Q = 200, orders_per_year = 6, cycle_time = 0.1666666667, ordering_cost = 600.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Economic order quantity (EOQ): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-1.webp)

*Economic order quantity (EOQ): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it: the app reads the quantities and their units, picks the calculation and fills in its form.

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![M/M/c queue (Erlang C)](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-2.webp)

*M/M/c queue (Erlang C)*

![X-bar and R chart control limits](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-3.webp)

*X-bar and R chart control limits*

![Johnson's rule (two machines)](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-4.webp)

*Johnson's rule (two machines)*

![Break-even point](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-5.webp)

*Break-even point*

![After-tax cash flow with depreciation and recapture](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-6.webp)

*After-tax cash flow with depreciation and recapture*

![Centre-of-gravity location](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-7.webp)

*Centre-of-gravity location*

![Associative forecast by simple linear regression](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/scene-8.webp)

*Associative forecast by simple linear regression*

![Economic order quantity (EOQ): a chart, enlarged.](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/chart-1.webp)

*Economic order quantity (EOQ): a chart, enlarged.*

![M/M/c queue (Erlang C): a chart, enlarged.](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/chart-2.webp)

*M/M/c queue (Erlang C): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Industrial Engineering and Supply Chain app in the dark theme.](https://www.ai999x.com/?ax_shot=industrial-engineering-ai999x/hero-dark.webp)

*The Industrial Engineering and Supply Chain app in the dark theme.*

## 7. Every calculation

98 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/industrial-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Break-even analysis | Break-even point, Multiproduct break-even (sales-mix weighted) |
| Engineering economy | After-tax cash flow with depreciation and recapture, Depreciation schedule (straight line, declining balance, sum of years' digits, MACRS) |
| Facility location | Centre-of-gravity location, Factor-rating method for site selection, Locational cost-volume (break-even) analysis, Minimum rectilinear-distance location (weighted median), Minimum weighted-distance location (Weber problem) |
| Forecasting | Associative forecast by simple linear regression, Forecast accuracy: MAD, MSE, RMSE, MAPE, bias, tracking signal, Holt's linear trend (double exponential smoothing), Linear trend projection (least squares), Moving average forecast, Seasonal indices (average method), Simple exponential smoothing forecast, Tracking signal over time, Weighted moving average forecast |
| Inventory | (R, Q) reorder point for a target fill rate, ABC inventory classification, Annual inventory cost of an order quantity, Economic order quantity (EOQ), Economic production quantity (EPQ), EOQ with all-units quantity discounts, EOQ with planned backorders, Inventory turnover and days of supply, Newsvendor order quantity (discrete demand table), Newsvendor order quantity (normal demand), Newsvendor order quantity (Poisson demand), Newsvendor order quantity (uniform demand), Number of kanbans, Periodic review order-up-to level, Reorder point (constant demand), Safety stock and reorder point (normal demand), Safety stock with variable demand and lead time, Service level of a reorder point |
| Learning curves | Cumulative-average learning curve (Wright), Fit a learning curve to data (log-log regression), Learning curve: time of the n-th unit, Learning curve: total and average time of N units, Learning rate from two unit times |
| Line balancing | Line balance efficiency, balance delay and idle time, Line balancing by ranked positional weight, Line balancing by the largest candidate rule, Takt (cycle) time and minimum stations |
| Project management | Earned value analysis (CV, SV, CPI, SPI, EAC) |
| Queueing | Cost-optimal number of servers, Little's law, M/D/1 queue (constant service time), M/G/1 queue (Pollaczek-Khinchine), M/M/1 queue, M/M/1/K queue (finite capacity), M/M/c queue (Erlang C), Servers needed for a service level (Erlang C) |
| Reliability | Cold standby redundancy, Confidence bounds on MTBF (chi-square), Exponential (constant failure rate) reliability, Failure rate and MTBF of a series system, Inherent availability from MTBF and MTTR, k-out-of-n system reliability, MTTF of identical components in parallel, Parallel components needed for a target reliability, Reliability of components in parallel, Reliability of components in series, Series system of parallel stages, Weibull fit by median-rank regression, Weibull reliability, hazard and MTTF |
| Scheduling | Earliest due date (EDD) sequence, Johnson's rule (two machines), Johnson's rule extended to three machines, Minimise the number of late jobs (Moore-Hodgson), Permutation flow shop makespan, Sequencing by a dispatching rule (FCFS, SPT, LPT, EDD, CR), Shortest processing time (SPT) sequence |
| Statistical process control | Average run length of a Shewhart X-bar chart, c chart control limits, DPMO and sigma level, EWMA chart control limits, Fraction nonconforming (normal process), np chart control limits, p chart control limits, p chart from sample counts, Process capability Cp and Cpk, Process performance Pp and Ppk from data, Process standard deviation from the average range, Rolled throughput yield, Single sampling plan: OC, AOQ and ATI, Tabular CUSUM chart, Taguchi capability index Cpm, u chart control limits, X-bar and R chart control limits, X-bar and S chart control limits |
| Work measurement | Capacity utilisation and efficiency, Number of cycles to time in a time study, Overall equipment effectiveness (OEE), Single-factor and multifactor productivity, Standard time from a time study, Work sampling sample size |

## 8. Other ways to use it

The same engine runs on the command line (`industrial-engineering`, its own guide), in Python (`pip install industrial-engineering-ai999x`, `import industrial_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/industrial-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/industrial-engineering-ai999x/
