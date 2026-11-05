//! Industrial Engineering's own live figures: values over two inputs (queues, control charts, reliability, inventory),
//! the matrices of a schedule or a line balance beside what was returned, locations among their demand points,
//! fitted lines and forecasts. Each is computed by the model itself for the visitor's inputs (the answer, and further
//! solves through ax_engine::figkit::Solved), never drawn from a formula written here. A figure that cannot be drawn
//! for the inputs is left out.

use ax_engine::figkit::{self as k, num, Canvas, Plot, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub fn live(s: &Solved) -> Vec<Figure> {
    use Rng::*;
    let v: Vec<Option<Figure>> = match s.key {
        "ie_arl_shewhart" => vec![field2(s, "shift", Decade, "subgroup_size", Count(1, 13), "arl",
                                         "Average run length after a mean shift, over the shift and the subgroup size")],
        "ie_availability" => vec![field2(s, "mtbf", Decade, "mttr", Decade, "availability",
                                         "Inherent availability over MTBF and MTTR")],
        "ie_ewma_limits" => vec![field2(s, "target", Near, "sigma", Near, "ucl",
                                        "EWMA upper control limit over the target mean and the standard deviation")],
        "ie_exponential_reliability" => vec![field2(s, "failure_rate", Decade, "time", Decade, "reliability",
                                                    "Reliability with a constant failure rate, over the rate and the mission time")],
        "ie_inventory_turnover" => vec![field2(s, "cost_of_goods_sold", Near, "average_inventory", Near, "turnover",
                                               "Inventory turns per year over the cost of goods sold and the average inventory")],
        "ie_littles_law" => vec![field2(s, "arrival_rate", Near, "time_in_system", Near, "number_in_system",
                                        "Little's law: the number in the system over the arrival rate and the time in it")],
        "ie_md1_queue" => vec![field2(s, "service_rate", Near, "arrival_rate", Near, "Lq",
                                      "M/D/1 queue: mean number waiting over the service and arrival rates")],
        "ie_mm1_queue" => vec![field2(s, "service_rate", Near, "arrival_rate", Near, "L",
                                      "M/M/1 queue: mean number in the system over the service and arrival rates")],
        "ie_newsvendor_discrete" => vec![field2(s, "price", Near, "cost", Near, "expected_profit",
                                                "Newsvendor expected profit over the selling price and the purchase cost")],
        "ie_np_chart_limits" => vec![field2(s, "p_bar", Prob, "sample_size", Count(1, 200), "ucl",
                                            "np chart upper control limit over the fraction defective and the sample size")],
        "ie_p_chart_limits" => vec![field2(s, "p_bar", Prob, "sample_size", Count(1, 200), "ucl",
                                           "p chart upper control limit over the fraction defective and the sample size")],
        "ie_parallel_mttf" => vec![field2(s, "failure_rate", Near, "n", Count(1, 10), "mttf",
                                          "MTTF of identical parallel components over the failure rate and their number")],
        "ie_process_performance" => vec![field2(s, "usl", Spread("lsl"), "lsl", Spread("usl"), "ppk",
                                                "Process performance Ppk over the specification limits")],
        "ie_redundancy_needed" => vec![field2(s, "target_reliability", Prob, "component_reliability", Prob,
                                              "achieved_reliability",
                                              "Reliability reached with the parallel components needed, over the target and \
                                               the component reliability")],
        "ie_reorder_point" => vec![field2(s, "demand_rate", Near, "lead_time", Near, "reorder_point",
                                          "Reorder point over the demand rate and the lead time")],
        "ie_sigma_from_range" => vec![field2(s, "mean_range", Near, "subgroup_size", Count(2, 10), "sigma",
                                             "Process standard deviation from the average range, over the range and the \
                                              subgroup size")],
        "ie_standard_time" => vec![field2(s, "rating", Near, "allowance", Prob, "standard_time",
                                          "Standard time over the performance rating and the allowance")],
        "ie_standby_redundancy" => vec![field2(s, "failure_rate", Decade, "time", Decade, "reliability",
                                               "Cold standby reliability over the failure rate and the mission time")],
        "ie_u_chart_limits" => vec![field2(s, "u_bar", Decade, "units", Decade, "ucl",
                                           "u chart upper control limit over the defects per unit and the units inspected")],
        "ie_xbar_s_limits" => vec![field2(s, "grand_mean", Near, "mean_sd", Near, "xbar_ucl",
                                          "X-bar chart upper control limit over the grand mean and the average standard \
                                           deviation")],
        "ie_factor_rating" => vec![matrices(s, &[("scores", true, "site", "factor")], &["weighted_scores"],
                                            "Factor-rating method for site selection: scores and weighted scores")],
        "ie_flow_shop_makespan" => vec![matrices(s, &[("times", true, "job", "machine")], &["completion_times"],
                                                 "Permutation flow shop: processing times and completion times")],
        "ie_largest_candidate_rule" => vec![matrices(s, &[("precedence", true, "", "")], &["assignment"],
                                                     "Line balancing by the largest candidate rule: precedence and stations")],
        "ie_ranked_positional_weight" => vec![matrices(s, &[("precedence", true, "", "")], &["assignment",
                                                                                        "positional_weights"],
                                                       "Line balancing by ranked positional weight: precedence, stations \
                                                        and weights")],
        "ie_seasonal_index" => vec![matrices(s, &[("data", true, "year", "season")], &["indices"],
                                             "Seasonal indices: the data by year and season, and the indices")],
        "ie_series_parallel_reliability" => vec![matrices(s, &[("stage_reliabilities", true, "stage", "unit")], &[],
                                                          "Series system of parallel stages: the reliability of each unit")],
        "ie_center_of_gravity" | "ie_rectilinear_location" | "ie_weber_location" => vec![location(s)],
        "ie_regression_forecast" => vec![regression(s)],
        "ie_learning_curve_fit" => vec![learning(s)],
        "ie_linear_trend_forecast" | "ie_weighted_moving_average" => vec![data_bars(s)],
        "ie_exponential_smoothing" => vec![smoothing(s)],
        "ie_johnson_two_machine" => vec![sequence(s)],
        "ie_eoq" => vec![sweep(s, "q-vs-demand", "demand", "Q", (0.25, 2.0), 31,
                               "Economic order quantity against the annual demand")],
        "ie_mmc_queue" => vec![sweep(s, "lq-vs-arrival-rate", "arrival_rate", "Lq", (0.2, 1.6), 29,
                                     "M/M/c queue: mean number waiting against the arrival rate")],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
/// The unit of an input as the pack names it ("" for a pure number).
fn in_unit(s: &Solved, sym: &str) -> String {
    let kind = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.kind.clone());
    match kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

fn label(sym: &str, unit: &str) -> String {
    if unit.is_empty() { sym.to_string() } else { format!("{sym} ({unit})") }
}

/// What an input or output means, as the pack words it.
fn meaning(s: &Solved, sym: &str, output: bool) -> String {
    let p = s.p.procedure(s.key);
    let m = if output {
        p.and_then(|p| p.outputs.iter().find(|o| o.symbol == sym)).map(|o| o.meaning.clone())
    } else {
        p.and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.meaning.clone())
    };
    m.unwrap_or_else(|| sym.to_string())
}

fn is_int(s: &Solved, sym: &str) -> bool {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.ty == "integer")
        .unwrap_or(false)
}

/// A number input as given (a whole number for an integer input).
fn given(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| s.val(sym).map(|v| v.int() as f64))
}

fn input_of(s: &Solved, sym: &str, x: f64) -> Input {
    if is_int(s, sym) { Input::Int(x.round() as i64) } else { Input::Float(x) }
}

fn list_text(v: &[f64]) -> String {
    v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")
}

// ------------------------------------------------------------------------------------------------ fields
/// A long tick number centred on the right edge of the frame ("1.8e+06" at x = 504) would leave the canvas: it is
/// right-aligned to the edge instead; a tick number that would run into its neighbour is left out (its grid line
/// stays).
fn ticks_inside(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut prev_right = f64::NEG_INFINITY;
    for (i, part) in body.split("<text class=\"axp-t\" x=\"").enumerate() {
        if i == 0 {
            out.push_str(part);
            continue;
        }
        let x: f64 = part.split('"').next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
        let content = part.split('>').nth(1).and_then(|c| c.split('<').next()).unwrap_or("");
        let w = content.chars().count() as f64 * 7.6;
        let middle = part.split('>').next().unwrap_or("").contains("text-anchor=\"middle\"");
        let moved = middle && x + w / 2.0 > 514.0;
        if middle {
            let (l, r) = if moved { (514.0 - w, 514.0) } else { (x - w / 2.0, x + w / 2.0) };
            if l < prev_right + 4.0 {
                out.push_str(part.split_once("</text>").map(|p| p.1).unwrap_or(""));
                continue;
            }
            prev_right = r;
        }
        if moved {
            let rest = &part[part.find('"').map(|j| j + 1).unwrap_or(0)..];
            out.push_str(&format!("<text class=\"axp-t\" x=\"514\"{}", rest.replacen("text-anchor=\"middle\"",
                                                                                   "text-anchor=\"end\"", 1)));
        } else {
            out.push_str("<text class=\"axp-t\" x=\"");
            out.push_str(part);
        }
    }
    out
}

#[derive(Clone, Copy)]
enum Rng {
    /// half to one and a half times the value
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
    /// a probability around the value, inside (0, 1)
    Prob,
    /// whole numbers around the value, within these bounds
    Count(i64, i64),
    /// the value plus or minus the distance to another input
    Spread(&'static str),
}

/// The values of one axis: (values, logarithmic).
fn axis(s: &Solved, x0: f64, r: Rng, n: usize) -> Option<(Vec<f64>, bool)> {
    Some(match r {
        Rng::Near if x0 != 0.0 => {
            let (a, b) = (x0 * 0.5, x0 * 1.5);
            (k::span(a.min(b), a.max(b), n, false), false)
        }
        Rng::Near => (k::span(-1.0, 1.0, n, false), false),
        Rng::Decade if x0 > 0.0 => (k::span(x0 / 10.0, x0 * 10.0, n, true), true),
        Rng::Decade => return None,
        Rng::Prob => {
            let lo = (x0 - 0.45).max(0.01).min(x0);
            let hi = (x0 + (1.0 - x0) * 0.9).min(0.9999).max(x0);
            (k::span(lo, hi, n, false), false)
        }
        Rng::Count(lo, hi) => {
            let x = x0.round() as i64;
            let (lo, hi) = (lo.min(x), hi.max(x));
            if hi - lo + 1 <= n as i64 {
                ((lo..=hi).map(|i| i as f64).collect(), false)
            } else {
                // logarithmic whole numbers
                let mut v: Vec<f64> = k::span(lo.max(1) as f64, hi as f64, n, true).iter().map(|x| x.round()).collect();
                v.push(x as f64);
                v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                v.dedup();
                (v, lo >= 1)
            }
        }
        Rng::Spread(other) => {
            let d = (x0 - s.inp(other)?).abs().max(x0.abs() * 0.1).max(1e-9);
            (k::span(x0 - d, x0 + d, n, false), false)
        }
    })
}

fn field2(s: &Solved, xs_: &str, xr: Rng, ys_: &str, yr: Rng, out: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (given(s, xs_)?, given(s, ys_)?);
    let z0 = s.out(out)?;
    let (xs, xlog) = axis(s, x0, xr, 17)?;
    let (ys, ylog) = axis(s, y0, yr, 13)?;
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with_input(&[(xs_, input_of(s, xs_, *x)), (ys_, input_of(s, ys_, *y))])
                .and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let (ux, uy) = (in_unit(s, xs_), in_unit(s, ys_));
    let body = ticks_inside(&k::field(&xs, &ys, &vals, (x0, y0), &label(xs_, &ux), &label(ys_, &uy),
                                      &format!("{out} = {}", num(z0)), (xlog, ylog)));
    let blank = if n < xs.len() * ys.len() { " (a blank cell: inputs the model refuses)" } else { "" };
    Some(s.chart("field", title,
                 &format!("{} ({out}) over {xs_} ({}) from {} to {} and {ys_} ({}) from {} to {}{}: each of the {n} cells \
                           is one computation of the model, shaded by its value (darker is larger){blank}. Marked: the \
                           inputs given, {out} = {}.", meaning(s, out, true), meaning(s, xs_, false), num(xs[0]),
                          num(xs[xs.len() - 1]), meaning(s, ys_, false), num(ys[0]), num(ys[ys.len() - 1]),
                          match (xlog, ylog) {
                              (true, true) => " (logarithmic axes)",
                              (true, false) => " (logarithmic horizontal axis)",
                              (false, true) => " (logarithmic vertical axis)",
                              _ => "",
                          }, num(z0)), &body))
}

// ------------------------------------------------------------------------------------------------ matrices
/// Panels of numbers side by side, each shaded by its own largest magnitude: the input matrices first, then the
/// returned lists (one column each, or one row when they belong to the input's columns).
fn matrices(s: &Solved, inputs: &[(&str, bool, &str, &str)], outs: &[&str], title: &str) -> Option<Figure> {
    let mut panels: Vec<(String, Vec<Vec<f64>>, &str)> = vec![];
    let mut row_word = "";
    let mut col_word = "";
    for (sym, _, rw, cw) in inputs {
        let m = s.mat(sym)?;
        if m.len() > 24 || m[0].len() > 12 {
            return None;
        }
        if row_word.is_empty() {
            row_word = rw;
            col_word = cw;
        }
        panels.push((format!("{sym} (given)"), m, "in"));
    }
    let base_rows = panels.first().map(|p| p.1.len()).unwrap_or(0);
    let base_cols = panels.first().map(|p| p.1[0].len()).unwrap_or(0);
    let mut below: Option<(String, Vec<f64>)> = None;
    for sym in outs {
        let Some(v) = s.out_list(sym) else { continue };
        if v.len() > 24 {
            return None;
        }
        // a list with one value per column of the input (seasons) goes under it; otherwise a column beside it
        if v.len() == base_cols && v.len() != base_rows && below.is_none() {
            below = Some((format!("{sym} (returned)"), v));
        } else {
            panels.push((format!("{sym} (returned)"), v.iter().map(|x| vec![*x]).collect(), "out"));
        }
    }
    let ncols: usize = panels.iter().map(|p| p.1[0].len()).sum::<usize>();
    let nrows = panels.iter().map(|p| p.1.len()).max()? + if below.is_some() { 2 } else { 0 };
    let gaps = panels.len().saturating_sub(1) as f64;
    let left = 70.0;
    let cell = ((520.0 - left - 20.0 - gaps * 26.0) / ncols as f64).min(300.0 / nrows as f64).min(54.0);
    if cell < 14.0 {
        return None;
    }
    let total_w = cell * ncols as f64 + gaps * 26.0;
    let x_first = left + (520.0 - left - 20.0 - total_w) / 2.0;
    // panel titles: staggered on two lines when they are wider than their panels
    let mut centres = vec![];
    let mut xx = x_first;
    for (_, m, _) in &panels {
        let pw = cell * m[0].len() as f64;
        centres.push(xx + pw / 2.0);
        xx += pw + 26.0;
    }
    let stagger = panels.iter().enumerate().any(|(i, (name, m, _))| {
        let w = k::text_w(name, false) * 1.1;
        let room = cell * m[0].len() as f64 + if i + 1 < panels.len() { 26.0 } else { 40.0 };
        w > room
    });
    let y0 = if stagger { 68.0 } else { 52.0 };
    // the scalar outputs of the answer, under the panels (two lines when long)
    let scalars: Vec<String> = s.a.values.iter().filter_map(|(sym, v, _)| v.as_f64().map(|x| format!("{sym} = {}", num(x))))
        .collect();
    let foot = scalars.join(",  ");
    let half = (scalars.len() + 1) / 2;
    let lines: Vec<String> = if k::text_w(&foot, true) > 490.0 && scalars.len() > 1 {
        vec![scalars[..half].join(",  "), scalars[half..].join(",  ")]
    } else {
        vec![foot.clone()]
    };
    let mut c = Canvas::new(y0 + cell * nrows as f64 + 30.0 + 18.0 * lines.len() as f64 + if below.is_some() { 14.0 } else { 0.0 });
    let font_ok = cell >= 22.0;
    let mut returned = vec![];
    for (i, (name, m, _)) in panels.iter().enumerate() {
        let x = centres[i] - cell * m[0].len() as f64 / 2.0;
        let pc = m[0].len();
        let mx = m.iter().flatten().fold(0.0f64, |a, v| a.max(v.abs()));
        c.bold(centres[i], y0 - 12.0 - if stagger && i % 2 == 1 { 17.0 } else { 0.0 }, "middle", name);
        for (r, row) in m.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                cell_at(&mut c, x + cell * j as f64, y0 + cell * r as f64, cell, *v, mx, font_ok);
            }
        }
        if pc > 1 || panels.len() == 1 {
            let worded = k::text_w(&format!("{col_word} {pc}"), false) + 4.0 <= cell;
            for j in 0..pc {
                let t = if worded { format!("{col_word} {}", j + 1).trim().to_string() } else { (j + 1).to_string() };
                c.name(x + cell * (j as f64 + 0.5), y0 + cell * m.len() as f64 + 15.0, "middle", &t);
            }
        }
    }
    // row numbers left of the first panel
    let first_rows = panels.first().map(|p| p.1.len()).unwrap_or(0);
    for i in 0..first_rows {
        c.name(x_first - 6.0, y0 + cell * (i as f64 + 0.5) + 4.5, "end", &format!("{row_word} {}", i + 1).trim().to_string());
    }
    if let Some((name, v)) = &below {
        let mx = v.iter().fold(0.0f64, |a, x| a.max(x.abs()));
        let yb = y0 + cell * (base_rows as f64 + 1.0) + 14.0;
        c.name(x_first - 6.0, yb + cell / 2.0 + 4.5, "end", "returned");
        for (j, val) in v.iter().enumerate() {
            cell_at(&mut c, x_first + cell * j as f64, yb, cell, *val, mx, font_ok);
        }
        c.bold(x_first + cell * v.len() as f64 / 2.0, yb - 6.0, "middle", name);
        returned.push(format!("{name}: {}", list_text(v)));
    }
    for (name, m, kind) in &panels {
        if *kind == "out" {
            returned.push(format!("{name}: {}", list_text(&m.iter().map(|r| r[0]).collect::<Vec<_>>())));
        }
    }
    let nl = lines.len() as f64;
    for (i, l) in lines.iter().enumerate() {
        c.value(260.0, c.h - 10.0 - 18.0 * (nl - 1.0 - i as f64), "middle", &k_clip(l, 62));
    }
    let h = c.h;
    Some(s.figure("matrix", title,
                  &format!("The matrices of the inputs given, each cell shaded by its magnitude within its own panel, and \
                            what the model returned with them{}{}. Also returned: {}.",
                           if returned.is_empty() { "" } else { ": " }, returned.join("; "), foot), c.body(), h))
}

fn k_clip(t: &str, n: usize) -> String {
    let c: Vec<char> = t.chars().collect();
    if c.len() <= n { t.to_string() } else { format!("{}...", c[..n - 3].iter().collect::<String>()) }
}

fn cell_at(c: &mut Canvas, x: f64, y: f64, cell: f64, v: f64, mx: f64, text: bool) {
    let op = if mx > 0.0 { 0.12 + 0.7 * v.abs() / mx } else { 0.12 };
    c.raw(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill-opacity=\"{op:.2}\"/>",
                   if v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 2.0, cell - 2.0));
    if text {
        c.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{:.0}\">{}</text>",
                       x + cell / 2.0 - 1.0, y + cell / 2.0 + 4.0, (cell * 0.3).clamp(9.0, 13.0),
                       if num(v).chars().count() as f64 * 8.4 > cell - 6.0 { ax_engine::num::py_g(v, 3) } else { num(v) }));
    }
}

// ------------------------------------------------------------------------------------------------ points and lines
fn range_of(v: &[f64]) -> (f64, f64) {
    let lo = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let d = if hi > lo { (hi - lo) * 0.08 } else { lo.abs().max(1.0) * 0.1 };
    (lo - d, hi + d)
}

/// Demand points (sized by their weights) and the location the model returned, with a dashed line to each point.
fn location(s: &Solved) -> Option<Figure> {
    let (xs, ys) = (s.vec("x")?, s.vec("y")?);
    let w = s.vec("weights").unwrap_or_else(|| vec![1.0; xs.len()]);
    if xs.len() != ys.len() || xs.len() > 60 || w.len() != xs.len() {
        return None;
    }
    let (lx, ly) = (s.out("x")?, s.out("y")?);
    let mut ax: Vec<f64> = xs.clone();
    ax.push(lx);
    let mut ay: Vec<f64> = ys.clone();
    ay.push(ly);
    let wmax = w.iter().fold(0.0f64, |a, x| a.max(x.abs()));
    let (what, title) = match s.key {
        "ie_center_of_gravity" => ("centre of gravity", "Centre-of-gravity location among the demand points"),
        "ie_rectilinear_location" => ("weighted median (rectilinear distance)",
                                      "Minimum rectilinear-distance location among the demand points"),
        _ => ("minimum weighted distance (Weber point)", "Minimum weighted-distance location among the demand points"),
    };
    let total = s.out("total_weighted_distance").or_else(|| s.out("total_weight"));
    let header = match s.out("total_weighted_distance") {
        Some(t) => format!("total weighted distance {}", num(t)),
        None => format!("total weight {}", num(total.unwrap_or(f64::NAN))),
    };
    let (ylo, yhi) = range_of(&ay);
    let mut p = Plot::new(range_of(&ax), (ylo, yhi + (yhi - ylo) * 0.12), "x", "y", &header, (false, false), true);
    for i in 0..xs.len() {
        if s.key == "ie_rectilinear_location" {
            p.dashed(&[(xs[i], ys[i]), (lx, ys[i]), (lx, ly)], "axp-c4");
        } else {
            p.dashed(&[(xs[i], ys[i]), (lx, ly)], "axp-c4");
        }
    }
    for i in 0..xs.len() {
        let r = if wmax > 0.0 { 4.0 + 9.0 * (w[i].abs() / wmax).sqrt() } else { 6.0 };
        p.dot((xs[i], ys[i]), r, "axp-d3");
        if xs.len() <= 12 {
            p.label((xs[i], ys[i]), 0.0, -r - 5.0, "middle", "axp-lt", &format!("w = {}", num(w[i])));
        }
    }
    p.mark((lx, ly), &format!("({}, {})", num(lx), num(ly)));
    Some(s.chart("xy", title,
                 &format!("The {} demand points as given (dot area grows with the weight) and the {what} the model \
                           returned, x = {}, y = {}, with a dashed line to each point. T{}.", xs.len(), num(lx),
                          num(ly), &header[1..]), &p.body()))
}

/// The data, and the regression line traced by the model's own forecast at 21 values of the driver.
fn regression(s: &Solved) -> Option<Figure> {
    let (xs, ys) = (s.vec("x")?, s.vec("y")?);
    let (xn, f) = (s.inp("x_new")?, s.out("forecast")?);
    if xs.len() != ys.len() || xs.len() > 400 {
        return None;
    }
    let mut ax = xs.clone();
    ax.push(xn);
    let (lo, hi) = range_of(&ax);
    let line = s.curve("x_new", &k::span(lo, hi, 21, false), "forecast", &[]);
    if line.len() < 2 {
        return None;
    }
    let mut ay = ys.clone();
    ay.extend(line.iter().map(|p| p.1));
    let header = format!("r² = {}", num(s.out("r_squared").unwrap_or(f64::NAN)));
    let mut p = Plot::new((lo, hi), range_of(&ay), "x (driver)", "y (demand)", &header, (false, false), false);
    p.path(&line, "axp-c");
    for i in 0..xs.len() {
        p.dot((xs[i], ys[i]), 4.5, "axp-d3");
    }
    p.mark((xn, f), &format!("forecast {}", num(f)));
    Some(s.chart("xy", "Associative forecast: the data and the fitted line",
                 &format!("The {} data points as given and the regression line, each of its 21 points a forecast \
                           computed by the model (intercept {}, slope {}); marked: the forecast at x_new = {}, {}.",
                          xs.len(), num(s.out("intercept").unwrap_or(f64::NAN)),
                          num(s.out("slope").unwrap_or(f64::NAN)), num(xn), num(f)), &p.body()))
}

/// The observed unit times and the fitted learning curve, each point a unit time computed by the model's own
/// learning-curve capability from the returned first-unit time and learning rate.
fn learning(s: &Solved) -> Option<Figure> {
    let (us, ts) = (s.vec("units")?, s.vec("times")?);
    let (lr, t1) = (s.out("learning_rate")?, s.out("first_unit_time")?);
    if us.len() != ts.len() || us.len() > 400 {
        return None;
    }
    let umax = us.iter().cloned().fold(1.0f64, f64::max).ceil();
    let mut grid: Vec<f64> = k::span(1.0, umax, 30, true).iter().map(|x| x.round()).collect();
    grid.dedup();
    let mut line = vec![];
    for u in grid {
        if let Some(a) = s.solve_other("ie_learning_curve_unit", &[("first_unit_time", Input::Float(t1)),
                                                                   ("learning_rate", Input::Float(lr)),
                                                                   ("unit", Input::Int(u as i64))]) {
            if let Some(t) = k::num_of(&a, "unit_time") {
                line.push((u, t));
            }
        }
    }
    if line.len() < 2 {
        return None;
    }
    let mut ay = ts.clone();
    ay.extend(line.iter().map(|p| p.1));
    let (ylo, yhi) = range_of(&ay);
    let mut p = Plot::new((us.iter().cloned().fold(f64::INFINITY, f64::min).min(1.0), umax), (ylo.max(0.0), yhi),
                          "Unit number", "Time per unit", &format!("learning rate {}", num(lr)), (true, false), true);
    p.path(&line, "axp-c");
    for i in 0..us.len() {
        p.dot((us[i], ts[i]), 4.5, "axp-d3");
    }
    p.mark((1.0, t1), &format!("first unit {}", num(t1)));
    Some(s.chart("xy", "Learning curve fitted to the observed unit times",
                 &format!("The {} observed unit times as given and the fitted curve: each point a unit time computed by \
                           the model for the returned first-unit time {} and learning rate {} (r² = {}); logarithmic \
                           unit axis.", us.len(), num(t1), num(lr), num(s.out("r_squared").unwrap_or(f64::NAN))),
                 &p.body()))
}

/// The data as bars, one per period, and the forecast the model returned as a highlighted bar after them.
fn data_bars(s: &Solved) -> Option<Figure> {
    let d = s.vec("data")?;
    let f = s.out("forecast")?;
    if d.is_empty() || d.len() > 60 {
        return None;
    }
    let h = s.val("horizon").map(|v| v.int()).unwrap_or(1).max(1) as f64;
    let n = d.len() as f64;
    let at = n + h;
    let weights = s.vec("weights");
    let mut all = d.clone();
    all.push(f);
    all.push(0.0);
    let (lo, hi) = range_of(&all);
    let mut p = Plot::new((0.4, at + 0.6), (lo.min(0.0), hi), "Period", "Value", &format!("forecast {}", num(f)),
                          (false, false), false);
    let used = weights.as_ref().map(|w| w.len()).unwrap_or(0);
    for (i, v) in d.iter().enumerate() {
        let x = i as f64 + 1.0;
        let in_avg = used > 0 && i + used >= d.len();
        p.rect((x - 0.35, 0.0), (x + 0.35, *v), if in_avg { "axp-b2" } else { "axp-b4" }, "");
        if d.len() <= 16 {
            p.label((x, *v), 0.0, -6.0, "middle", "axp-vt", &num(*v));
        }
        if in_avg {
            let w = weights.as_ref().unwrap()[i + used - d.len()];
            p.label((x, 0.0), 0.0, -8.0, "middle", "axp-lt", &format!("w {}", num(w)));
        }
    }
    p.rect((at - 0.35, 0.0), (at + 0.35, f), "axp-b", "");
    p.mark((at, f), "");
    p.label((at, f), 0.0, -12.0, "middle", "axp-mt", &num(f));
    if used > 0 {
        p.legend(&[("weighted periods".into(), "axp-b2"), ("forecast".into(), "axp-b")]);
    }
    let extra = match (s.out("intercept"), s.out("slope")) {
        (Some(a), Some(b)) => format!(" Also returned: intercept {}, slope {}.", num(a), num(b)),
        _ => String::new(),
    };
    Some(s.chart("data", if used > 0 { "Weighted moving average: the data and the forecast" } else {
                     "Linear trend projection: the data and the forecast" },
                 &format!("The {} observations as given, one bar per period{}, and the forecast the model returned for \
                           period {}: {}.{extra}", d.len(),
                          if used > 0 { " (the weighted periods in the second colour, their weights written under them)" }
                          else { "" }, num(at), num(f)), &p.body()))
}

/// The observations and the smoothed forecasts the model returned, period by period.
fn smoothing(s: &Solved) -> Option<Figure> {
    let d = s.vec("data")?;
    let fc = s.out_list("forecasts")?;
    if fc.len() < 2 || fc.len() > 400 {
        return None;
    }
    let obs: Vec<P> = d.iter().enumerate().map(|(i, v)| (i as f64 + 1.0, *v)).collect();
    let fl: Vec<P> = fc.iter().enumerate().map(|(i, v)| (i as f64 + 1.0, *v)).collect();
    let mut all: Vec<f64> = d.clone();
    all.extend(fc.iter());
    let last = *fl.last()?;
    let mut p = Plot::new((0.5, fl.len() as f64 + 0.5), range_of(&all), "Period k", "Value",
                          &format!("next forecast {}", num(last.1)), (false, false), false);
    p.path(&obs, "axp-c4");
    for q in &obs {
        p.dot(*q, 4.0, "axp-d4");
    }
    p.path(&fl, "axp-c");
    for q in &fl {
        p.dot(*q, 3.5, "axp-d");
    }
    p.legend(&[("observed".into(), "axp-b4"), ("forecasts".into(), "axp-b")]);
    p.mark(last, &format!("k = {}: {}", fl.len(), num(last.1)));
    Some(s.chart("forecasts-vs-k", "Exponential smoothing: the forecast of each period",
                 &format!("The forecasts the model returned for periods 1 to {} ({}) beside the {} observations as given; \
                           marked: the forecast for the next period, {}.", fl.len(), list_text(&fc), d.len(),
                          num(last.1)), &p.body()))
}

/// The job order the model returned: the job in each position.
fn sequence(s: &Solved) -> Option<Figure> {
    let seq = s.out_list("sequence")?;
    if seq.is_empty() || seq.len() > 60 {
        return None;
    }
    let pts: Vec<P> = seq.iter().enumerate().map(|(i, j)| (i as f64 + 1.0, *j)).collect();
    let jmax = seq.iter().cloned().fold(1.0f64, f64::max);
    let mk = s.out("makespan");
    let mut p = Plot::new((0.5, seq.len() as f64 + 0.5), (0.0, jmax + 0.6), "Position k in the order", "Job number",
                          &mk.map(|m| format!("makespan {}", num(m))).unwrap_or_default(), (false, false), false);
    p.stems(&pts, "axp-d");
    if seq.len() <= 20 {
        for q in &pts {
            p.label(*q, 0.0, -9.0, "middle", "axp-vt", &format!("job {}", num(q.1)));
        }
    }
    Some(s.chart("sequence-vs-k", "Johnson's rule: the job in each position of the order",
                 &format!("The job order the model returned, {}, as the job number at each position{}.", list_text(&seq),
                          mk.map(|m| format!("; makespan {}, idle time of machine 2 {}", num(m),
                                             num(s.out("idle_time_machine2").unwrap_or(f64::NAN))))
                              .unwrap_or_default()), &p.body()))
}

/// One output against one input, each point a solve of the model, the visitor's point marked.
fn sweep(s: &Solved, kind: &str, x: &str, y: &str, (a, b): (f64, f64), n: usize, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(x)?, s.out(y)?);
    if !(x0 > 0.0) {
        return None;
    }
    let pts = s.curve(x, &k::span(x0 * a, x0 * b, n, false), y, &[]);
    if pts.len() < 2 {
        return None;
    }
    let others: Vec<String> = s.given.iter().filter(|g| g.0 != x).map(|g| match &g.1 {
        Input::Text(t) => format!("{} = {t}", g.0),
        Input::Float(v) => format!("{} = {}", g.0, num(*v)),
        Input::Int(v) => format!("{} = {v}", g.0),
        Input::Json(v) => format!("{} = {v}", g.0),
    }).collect();
    let body = k::lines_y(&[(y.to_string(), pts.clone())], &label(x, &in_unit(s, x)), y,
                          &format!("{y} = {}", num(y0)), Some((x0, y0)), &format!("{x} = {}", num(x0)), false, true);
    Some(s.chart(kind, title,
                 &format!("{} ({y}) against {} ({x}) from {} to {}, with {}: {} points computed by the model (inputs it \
                           refuses are left out); marked: {x} = {}, {y} = {}.", meaning(s, y, true),
                          meaning(s, x, false), num(x0 * a), num(x0 * b), others.join(", "), pts.len(), num(x0),
                          num(y0)), &body))
}
