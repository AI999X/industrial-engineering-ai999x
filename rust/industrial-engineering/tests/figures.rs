//! Every Industrial Engineering capability's figures (src/figures.rs and the generic engine): its worked example gives
//! each published figure kind, a changed input changes the model's own drawings, and no drawing holds a non-finite
//! number or text outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use industrial_engineering_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/industrial-engineering-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("ie_abc_classification", &["sensitivity"]),
    ("ie_arl_shewhart", &["field"]),
    ("ie_availability", &["field"]),
    ("ie_center_of_gravity", &["xy"]),
    ("ie_dispatching_rule", &["series"]),
    ("ie_edd_sequence", &["series"]),
    ("ie_eoq", &["q-vs-demand"]),
    ("ie_ewma_limits", &["field"]),
    ("ie_exponential_reliability", &["field"]),
    ("ie_exponential_smoothing", &["forecasts-vs-k", "series"]),
    ("ie_factor_rating", &["matrix"]),
    ("ie_flow_shop_makespan", &["matrix", "series"]),
    ("ie_forecast_errors", &["sensitivity"]),
    ("ie_inventory_turnover", &["field"]),
    ("ie_johnson_three_machine", &["series"]),
    ("ie_johnson_two_machine", &["sequence-vs-k", "series"]),
    ("ie_largest_candidate_rule", &["matrix", "series"]),
    ("ie_learning_curve_fit", &["xy"]),
    ("ie_linear_trend_forecast", &["data"]),
    ("ie_littles_law", &["field"]),
    ("ie_location_cost_volume", &["series"]),
    ("ie_md1_queue", &["field"]),
    ("ie_mm1_queue", &["field"]),
    ("ie_mmc_queue", &["lq-vs-arrival-rate"]),
    ("ie_moore_hodgson", &["series"]),
    ("ie_moving_average_forecast", &["series"]),
    ("ie_multiproduct_break_even", &["series"]),
    ("ie_newsvendor_discrete", &["field"]),
    ("ie_np_chart_limits", &["field"]),
    ("ie_p_chart_limits", &["field"]),
    ("ie_parallel_mttf", &["field"]),
    ("ie_parallel_reliability", &["sensitivity"]),
    ("ie_process_performance", &["field"]),
    ("ie_ranked_positional_weight", &["matrix", "series"]),
    ("ie_rectilinear_location", &["xy"]),
    ("ie_redundancy_needed", &["field"]),
    ("ie_regression_forecast", &["xy"]),
    ("ie_reorder_point", &["field"]),
    ("ie_rolled_throughput_yield", &["sensitivity"]),
    ("ie_seasonal_index", &["matrix", "series"]),
    ("ie_series_parallel_reliability", &["matrix"]),
    ("ie_series_reliability", &["sensitivity"]),
    ("ie_sigma_from_range", &["field"]),
    ("ie_spt_sequence", &["series"]),
    ("ie_standard_time", &["field"]),
    ("ie_standby_redundancy", &["field"]),
    ("ie_system_failure_rate", &["sensitivity"]),
    ("ie_tabular_cusum", &["series"]),
    ("ie_tracking_signal", &["series"]),
    ("ie_u_chart_limits", &["field"]),
    ("ie_weber_location", &["xy"]),
    ("ie_weibull_median_rank_regression", &["sensitivity"]),
    ("ie_weighted_moving_average", &["data"]),
    ("ie_xbar_s_limits", &["field"]),
];

/// The kinds the model draws itself (the rest come from the generic engine, whose drawings are checked by its own
/// tests).
const OWN: &[&str] = &["field", "matrix", "xy", "data", "forecasts-vs-k", "sequence-vs-k", "q-vs-demand",
                       "lq-vs-arrival-rate"];

/// Inputs to change for a capability whose numbers cannot simply be scaled (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("ie_johnson_two_machine", "machine1_times", "[1, 2, 5, 7, 6]"),
    ("ie_series_parallel_reliability", "stage_reliabilities", "[[0.7, 0.9], [0.8, 0.6], [0.95, 0.9]]"),
    ("ie_largest_candidate_rule", "task_times", "[4, 3, 2, 3, 6, 4, 2, 3]"),
    ("ie_ranked_positional_weight", "task_times", "[4, 3, 2, 3, 6, 4, 2, 3]"),
];

/// The arguments of a structured example: solve('key', a=1, b=[1, 2], c='text') (lists kept whole).
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            let mut depth = 0i32;
            while i < cs.len() && !(depth == 0 && (cs[i] == ',' || cs[i] == ')')) {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let a = ax_engine::solve(p, key, &given, None);
        assert!(a.solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if OWN.contains(kind) {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    }
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    if OWN.contains(kind) {
                        check_texts(f);
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("[3, 6]" -> "[4.11, 8.22]").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let own: Vec<&str> = kinds.iter().copied().filter(|k| OWN.contains(k)).collect();
        if own.is_empty() {
            continue;
        }
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73, 1.1] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = own.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // an M/M/1 queue with arrivals faster than service has no steady state: refused, so no figure at all
    let p = pack();
    let given: Vec<(String, Input)> = [("arrival_rate", "5"), ("service_rate", "3")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "ie_mm1_queue", &given, None);
    assert!(!a.solved(), "an unstable queue was solved");
    assert!(figures(p, "ie_mm1_queue", &given, None, None).is_empty());
}
