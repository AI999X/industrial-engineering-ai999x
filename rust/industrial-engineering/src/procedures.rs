//! industrial_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/industrial_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "ie_abc_classification",
    "ie_acceptance_sampling",
    "ie_arl_shewhart",
    "ie_availability",
    "ie_break_even",
    "ie_c_chart_limits",
    "ie_capacity_utilization",
    "ie_center_of_gravity",
    "ie_cpm_taguchi",
    "ie_dispatching_rule",
    "ie_dpmo_sigma_level",
    "ie_edd_sequence",
    "ie_eoq",
    "ie_eoq_backorders",
    "ie_eoq_quantity_discount",
    "ie_epq",
    "ie_erlang_c_staffing",
    "ie_ewma_limits",
    "ie_exponential_reliability",
    "ie_exponential_smoothing",
    "ie_factor_rating",
    "ie_flow_shop_makespan",
    "ie_forecast_errors",
    "ie_fraction_nonconforming",
    "ie_holt_trend",
    "ie_inventory_total_cost",
    "ie_inventory_turnover",
    "ie_johnson_three_machine",
    "ie_johnson_two_machine",
    "ie_k_out_of_n_reliability",
    "ie_kanban_count",
    "ie_largest_candidate_rule",
    "ie_learning_curve_cumulative_average",
    "ie_learning_curve_fit",
    "ie_learning_curve_total",
    "ie_learning_curve_unit",
    "ie_learning_rate_from_two_units",
    "ie_line_efficiency",
    "ie_linear_trend_forecast",
    "ie_littles_law",
    "ie_location_cost_volume",
    "ie_md1_queue",
    "ie_mg1_queue",
    "ie_mm1_queue",
    "ie_mm1k_queue",
    "ie_mmc_queue",
    "ie_moore_hodgson",
    "ie_moving_average_forecast",
    "ie_mtbf_confidence",
    "ie_multiproduct_break_even",
    "ie_newsvendor_discrete",
    "ie_newsvendor_normal",
    "ie_newsvendor_poisson",
    "ie_newsvendor_uniform",
    "ie_np_chart_limits",
    "ie_oee",
    "ie_order_up_to_level",
    "ie_p_chart_from_samples",
    "ie_p_chart_limits",
    "ie_parallel_mttf",
    "ie_parallel_reliability",
    "ie_process_capability",
    "ie_process_performance",
    "ie_queue_server_cost",
    "ie_ranked_positional_weight",
    "ie_rectilinear_location",
    "ie_redundancy_needed",
    "ie_regression_forecast",
    "ie_reorder_point",
    "ie_rolled_throughput_yield",
    "ie_rq_fill_rate",
    "ie_safety_stock",
    "ie_safety_stock_variable_lead_time",
    "ie_seasonal_index",
    "ie_series_parallel_reliability",
    "ie_series_reliability",
    "ie_service_level_from_reorder_point",
    "ie_sigma_from_range",
    "ie_spt_sequence",
    "ie_standard_time",
    "ie_standby_redundancy",
    "ie_system_failure_rate",
    "ie_tabular_cusum",
    "ie_takt_time",
    "ie_time_study_sample_size",
    "ie_tracking_signal",
    "ie_u_chart_limits",
    "ie_weber_location",
    "ie_weibull_median_rank_regression",
    "ie_weibull_reliability",
    "ie_weighted_moving_average",
    "ie_work_sampling_sample_size",
    "ie_xbar_r_limits",
    "ie_xbar_s_limits",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "ie_abc_classification" => {
            let r = f_abc_classification(match v.get("annual_values") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_share") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.8)) })().ok()? } }, match v.get("b_share") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.95)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_51(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_acceptance_sampling" => {
            let r = f_acceptance_sampling(match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } }, match v.get("acceptance_number") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fraction_defective") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lot_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_110(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_arl_shewhart" => {
            let r = f_arl_shewhart(match v.get("shift") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("limit_sigma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_165(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_availability" => {
            let r = f_availability(match v.get("mtbf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mttr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hours_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(8760.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_205(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_break_even" => {
            let r = f_break_even(match v.get("fixed_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_profit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_251(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_c_chart_limits" => {
            let r = f_c_chart_limits(match v.get("c_bar") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_290(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_capacity_utilization" => {
            let r = f_capacity_measures(match v.get("design_capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("effective_capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_output") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_334(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_center_of_gravity" => {
            let r = f_center_of_gravity(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_398(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_cpm_taguchi" => {
            let r = f_cpm_index(match v.get("usl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_443(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_dispatching_rule" => {
            let r = f_dispatch_rule(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rule") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_521(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_dpmo_sigma_level" => {
            let r = f_dpmo_sigma(match v.get("defects") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("opportunities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_566(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_edd_sequence" => {
            let r = f_edd_sequence(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_634(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_eoq" => {
            let r = f_eoq(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_674(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_eoq_backorders" => {
            let r = f_eoq_backorders(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("backorder_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_714(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_eoq_quantity_discount" => {
            let r = f_eoq_discount(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("break_quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit_prices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_796(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_epq" => {
            let r = f_epq(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("production_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_839(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_erlang_c_staffing" => {
            let r = f_staffing(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_891(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_ewma_limits" => {
            let r = f_ewma_limits(match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("smoothing") { Some(x) => py::from_val(x), None => { return None } }, match v.get("limit_sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_number") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_exponential_reliability" => {
            let r = f_exponential_reliability(match v.get("failure_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_989(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_exponential_smoothing" => {
            let r = f_exp_smoothing(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_forecast") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1048(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_factor_rating" => {
            let r = f_factor_rating(match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1127(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_flow_shop_makespan" => {
            let r = f_flow_shop_makespan(match v.get("times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1195(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_forecast_errors" => {
            let r = f_forecast_errors(match v.get("actual") { Some(x) => py::from_val(x), None => { return None } }, match v.get("forecast") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_fraction_nonconforming" => {
            let r = f_fraction_nonconforming(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("usl") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1316(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_holt_trend" => {
            let r = f_holt(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_level") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_trend") { Some(x) => py::from_val(x), None => { return None } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1387(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_inventory_total_cost" => {
            let r = f_inventory_total_cost(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit_cost") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1431(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_inventory_turnover" => {
            let r = f_inventory_turnover(match v.get("cost_of_goods_sold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("average_inventory") { Some(x) => py::from_val(x), None => { return None } }, match v.get("days_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(365i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1471(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_johnson_three_machine" => {
            let r = f_johnson_three(match v.get("machine1_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("machine2_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("machine3_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1543(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_johnson_two_machine" => {
            let r = f_johnson_two(match v.get("machine1_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("machine2_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1607(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_k_out_of_n_reliability" => {
            let r = f_k_out_of_n(match v.get("component_reliability") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1663(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_kanban_count" => {
            let r = f_kanban_count(match v.get("demand_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("container_size") { Some(x) => py::from_val(x), None => { return None } }, match v.get("safety_factor") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1708(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_largest_candidate_rule" => {
            let r = f_largest_candidate(match v.get("task_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precedence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cycle_time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1835(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_cumulative_average" => {
            let r = f_wright_average(match v.get("first_unit_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("learning_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_fit" => {
            let r = f_learning_fit(match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1959(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_total" => {
            let r = f_learning_cumulative(match v.get("first_unit_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("learning_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2015(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_unit" => {
            let r = f_learning_unit(match v.get("first_unit_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("learning_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2070(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_rate_from_two_units" => {
            let r = f_learning_two_points(match v.get("unit1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_line_efficiency" => {
            let r = f_line_efficiency(match v.get("task_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cycle_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stations") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2187(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_linear_trend_forecast" => {
            let r = f_linear_trend(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2259(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_littles_law" => {
            let r = f_littles_law(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("number_in_system") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("time_in_system") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2312(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_location_cost_volume" => {
            let r = f_location_cost_volume(match v.get("fixed_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("volume") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2373(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_md1_queue" => {
            let r = f_md1(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2417(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mg1_queue" => {
            let r = f_mg1(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_service_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_service_time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2463(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mm1_queue" => {
            let r = f_mm1(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2505(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mm1k_queue" => {
            let r = f_mm1k(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2562(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mmc_queue" => {
            let r = f_mmc(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("servers") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2618(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_moore_hodgson" => {
            let r = f_moore_hodgson(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2693(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_moving_average_forecast" => {
            let r = f_moving_average(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2761(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mtbf_confidence" => {
            let r = f_mtbf_confidence(match v.get("total_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("failures") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("test") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("time")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2852(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_multiproduct_break_even" => {
            let r = f_multiproduct_break_even(match v.get("fixed_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sales_mix") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2917(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_discrete" => {
            let r = f_newsvendor_discrete(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demands") { Some(x) => py::from_val(x), None => { return None } }, match v.get("probabilities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2997(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_normal" => {
            let r = f_newsvendor_normal(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3042(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_poisson" => {
            let r = f_newsvendor_poisson(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3096(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_uniform" => {
            let r = f_newsvendor_uniform(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("low") { Some(x) => py::from_val(x), None => { return None } }, match v.get("high") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_np_chart_limits" => {
            let r = f_np_chart_limits(match v.get("p_bar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3196(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_oee" => {
            let r = f_oee(match v.get("planned_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("downtime") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ideal_cycle_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_count") { Some(x) => py::from_val(x), None => { return None } }, match v.get("good_count") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_order_up_to_level" => {
            let r = f_order_up_to(match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("review_period") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_level") { Some(x) => py::from_val(x), None => { return None } }, match v.get("inventory_position") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3294(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_p_chart_from_samples" => {
            let r = f_p_chart_counts(match v.get("defectives") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3375(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_p_chart_limits" => {
            let r = f_p_chart_limits(match v.get("p_bar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3428(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_parallel_mttf" => {
            let r = f_parallel_mttf(match v.get("failure_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3481(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_parallel_reliability" => {
            let r = f_parallel_reliability(match v.get("reliabilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3539(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_process_capability" => {
            let r = f_capability(match v.get("usl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_process_performance" => {
            let r = f_performance(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("usl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3642(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_queue_server_cost" => {
            let r = f_server_cost(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("server_cost_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("waiting_cost_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("max_servers") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(50i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3708(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_ranked_positional_weight" => {
            let r = f_ranked_positional_weight(match v.get("task_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precedence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cycle_time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3843(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_rectilinear_location" => {
            let r = f_rectilinear_location(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3915(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_redundancy_needed" => {
            let r = f_redundancy_needed(match v.get("target_reliability") { Some(x) => py::from_val(x), None => { return None } }, match v.get("component_reliability") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3960(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_regression_forecast" => {
            let r = f_regression_forecast(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_new") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4029(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_reorder_point" => {
            let r = f_reorder_point(match v.get("demand_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("safety_stock") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4071(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_rolled_throughput_yield" => {
            let r = f_rolled_yield(match v.get("yields") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4129(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_rq_fill_rate" => {
            let r = f_rq_fill_rate(match v.get("mean_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fill_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4203(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_safety_stock" => {
            let r = f_safety_stock_normal(match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4247(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_safety_stock_variable_lead_time" => {
            let r = f_safety_stock_vlt(match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4298(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_seasonal_index" => {
            let r = f_seasonal_index(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("next_year_total") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4367(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_series_parallel_reliability" => {
            let r = f_series_parallel(match v.get("stage_reliabilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4430(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_series_reliability" => {
            let r = f_series_reliability(match v.get("reliabilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4488(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_service_level_from_reorder_point" => {
            let r = f_service_from_rop(match v.get("mean_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reorder_point") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_quantity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4530(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_sigma_from_range" => {
            let r = f_sigma_from_range(match v.get("mean_range") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_spt_sequence" => {
            let r = f_spt_sequence(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4650(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_standard_time" => {
            let r = f_standard_time(match v.get("observed_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rating") { Some(x) => py::from_val(x), None => { return None } }, match v.get("allowance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("basis") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4713(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_standby_redundancy" => {
            let r = f_standby_redundancy(match v.get("failure_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4769(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_system_failure_rate" => {
            let r = f_system_failure_rate(match v.get("failure_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4826(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_tabular_cusum" => {
            let r = f_tabular_cusum(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_takt_time" => {
            let r = f_takt_time(match v.get("available_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_task_time") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4939(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_time_study_sample_size" => {
            let r = f_time_study_size(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("accuracy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_tracking_signal" => {
            let r = f_tracking_signal(match v.get("actual") { Some(x) => py::from_val(x), None => { return None } }, match v.get("forecast") { Some(x) => py::from_val(x), None => { return None } }, match v.get("limit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(4.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_u_chart_limits" => {
            let r = f_u_chart_limits(match v.get("u_bar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5097(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weber_location" => {
            let r = f_weber_location(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5192(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weibull_median_rank_regression" => {
            let r = f_weibull_regression(match v.get("failure_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5254(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weibull_reliability" => {
            let r = f_weibull_reliability(match v.get("shape") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scale") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("location") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5304(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weighted_moving_average" => {
            let r = f_weighted_moving_average(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5362(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_work_sampling_sample_size" => {
            let r = f_work_sampling_size(match v.get("proportion") { Some(x) => py::from_val(x), None => { return None } }, match v.get("accuracy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_xbar_r_limits" => {
            let r = f_xbar_r_limits(match v.get("grand_mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_range") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_xbar_s_limits" => {
            let r = f_xbar_s_limits(match v.get("grand_mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5509(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__A2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(1.88)), (Py::Int(3i64), Py::Float(1.023)), (Py::Int(4i64), Py::Float(0.729)), (Py::Int(5i64), Py::Float(0.577)), (Py::Int(6i64), Py::Float(0.483)), (Py::Int(7i64), Py::Float(0.419)), (Py::Int(8i64), Py::Float(0.373)), (Py::Int(9i64), Py::Float(0.337)), (Py::Int(10i64), Py::Float(0.308))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__A3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(2.659)), (Py::Int(3i64), Py::Float(1.954)), (Py::Int(4i64), Py::Float(1.628)), (Py::Int(5i64), Py::Float(1.427)), (Py::Int(6i64), Py::Float(1.287)), (Py::Int(7i64), Py::Float(1.182)), (Py::Int(8i64), Py::Float(1.099)), (Py::Int(9i64), Py::Float(1.032)), (Py::Int(10i64), Py::Float(0.975))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__B3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(0.0)), (Py::Int(3i64), Py::Float(0.0)), (Py::Int(4i64), Py::Float(0.0)), (Py::Int(5i64), Py::Float(0.0)), (Py::Int(6i64), Py::Float(0.03)), (Py::Int(7i64), Py::Float(0.118)), (Py::Int(8i64), Py::Float(0.185)), (Py::Int(9i64), Py::Float(0.239)), (Py::Int(10i64), Py::Float(0.284))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__B4() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(3.267)), (Py::Int(3i64), Py::Float(2.568)), (Py::Int(4i64), Py::Float(2.266)), (Py::Int(5i64), Py::Float(2.089)), (Py::Int(6i64), Py::Float(1.97)), (Py::Int(7i64), Py::Float(1.882)), (Py::Int(8i64), Py::Float(1.815)), (Py::Int(9i64), Py::Float(1.761)), (Py::Int(10i64), Py::Float(1.716))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__D2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(1.128)), (Py::Int(3i64), Py::Float(1.693)), (Py::Int(4i64), Py::Float(2.059)), (Py::Int(5i64), Py::Float(2.326)), (Py::Int(6i64), Py::Float(2.534)), (Py::Int(7i64), Py::Float(2.704)), (Py::Int(8i64), Py::Float(2.847)), (Py::Int(9i64), Py::Float(2.97)), (Py::Int(10i64), Py::Float(3.078))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__D3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(0.0)), (Py::Int(3i64), Py::Float(0.0)), (Py::Int(4i64), Py::Float(0.0)), (Py::Int(5i64), Py::Float(0.0)), (Py::Int(6i64), Py::Float(0.0)), (Py::Int(7i64), Py::Float(0.076)), (Py::Int(8i64), Py::Float(0.136)), (Py::Int(9i64), Py::Float(0.184)), (Py::Int(10i64), Py::Float(0.223))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__D4() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(3.267)), (Py::Int(3i64), Py::Float(2.574)), (Py::Int(4i64), Py::Float(2.282)), (Py::Int(5i64), Py::Float(2.114)), (Py::Int(6i64), Py::Float(2.004)), (Py::Int(7i64), Py::Float(1.924)), (Py::Int(8i64), Py::Float(1.864)), (Py::Int(9i64), Py::Float(1.816)), (Py::Int(10i64), Py::Float(1.777))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RULES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("fcfs"), Py::str("spt"), Py::str("lpt"), Py::str("edd"), Py::str("cr")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__b(mut v_r: Py) -> R {
    return Ok(py::truediv(&py::math1("log", &v_r.clone())?, &py::math1("log", &Py::Float(2.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__balance(mut v_t: Py, mut v_preds: Py, mut v_c: Py, mut v_priority: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_assigned: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_loads: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_station: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_n = py::len(&v_t.clone())?;
    v_station = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
    v_loads = Py::list(vec![]);
    v_assigned = py::mul(&Py::list(vec![Py::Bool(false)]), &v_n.clone())?;
    v_k = Py::Int(0i64);
    while ({ let t3827 = v_k.clone(); Py::Bool({ let t3828 = v_n.clone(); if py::lt(&t3827, &t3828)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_loads.clone(), "append", &[Py::Float(0.0)])?;
        v_s = py::len(&v_loads.clone())?;
        while (Py::Bool(true)).truthy() {
            v_best = py::neg(&Py::Int(1i64))?;
            for t3811 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_j = t3811;
                if ({ let t3814 = py::getitem(&v_assigned.clone(), &v_j.clone())?; if t3814.truthy() { t3814 } else { { let t3812 = py::add(&py::getitem(&v_loads.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_t.clone(), &v_j.clone())?)?; Py::Bool({ let t3813 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t3812, &t3813)? { true } else { false } }) } } }).truthy() {
                    continue;
                }
                if (py::not(&Py::Bool(({ let mut out = vec![]; for t3815 in py::iter(&(py::getitem(&v_preds.clone(), &v_j.clone())?))? { let mut v_p: Py; 
v_p = t3815;
out.push(py::getitem(&v_assigned.clone(), &v_p.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
                    continue;
                }
                if ({ let t3820 = { let t3816 = v_best.clone(); Py::Bool({ let t3817 = Py::Int(0i64); if py::lt(&t3816, &t3817)? { true } else { false } }) }; if t3820.truthy() { t3820 } else { { let t3818 = py::getitem(&v_priority.clone(), &v_j.clone())?; Py::Bool({ let t3819 = py::getitem(&v_priority.clone(), &v_best.clone())?; if py::gt(&t3818, &t3819)? { true } else { false } }) } } }).truthy() {
                    v_best = v_j.clone();
                }
            }
            if ({ let t3821 = v_best.clone(); Py::Bool({ let t3822 = Py::Int(0i64); if py::lt(&t3821, &t3822)? { true } else { false } }) }).truthy() {
                break;
            }
            py::setitem(&v_assigned.clone(), &v_best.clone(), Py::Bool(true))?;
            py::setitem(&v_station.clone(), &v_best.clone(), v_s.clone())?;
            let t3823 = v_loads.clone(); let t3824 = py::neg(&Py::Int(1i64))?;
            py::setitem(&t3823, &t3824, py::iadd(&py::getitem(&t3823, &t3824)?, &py::getitem(&v_t.clone(), &v_best.clone())?)?)?;
            v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        }
        let _ = f__need({ let t3825 = py::getitem(&v_loads.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3826 = Py::Int(0i64); if py::gt(&t3825, &t3826)? { true } else { false } }) }, Py::str("a task does not fit in the cycle time"))?;
    }
    v_N = py::len(&v_loads.clone())?;
    v_w = py::sum(&v_t.clone(), None)?;
    return Ok(Py::dict(vec![(Py::str("stations"), v_N.clone()), (Py::str("assignment"), v_station.clone()), (Py::str("station_times"), v_loads.clone()), (Py::str("efficiency"), py::truediv(&v_w.clone(), &py::mul(&v_N.clone(), &v_c.clone())?)?), (Py::str("idle_time"), py::sub(&py::mul(&v_N.clone(), &v_c.clone())?, &v_w.clone())?), (Py::str("minimum_stations"), f__ceil(py::truediv(&v_w.clone(), &v_c.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__binom_cdf(mut v_c: Py, mut v_n: Py, mut v_p: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = Py::Float(0.0);
    for t97 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&v_c.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t97;
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::comb(&v_n.clone(), &v_k.clone())?, &py::pow(&v_p.clone(), &v_k.clone())?)?, &py::pow(&py::sub(&Py::Float(1.0), &v_p.clone())?, &py::sub(&v_n.clone(), &v_k.clone())?)?)?)?;
    }
    return Ok(py::minmax(&[Py::Float(1.0), v_s.clone()], false)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect_root(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_iters: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4165 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t4165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t4165[0].clone();
    v_fb = t4165[1].clone();
    if ({ let t4166 = v_fa.clone(); Py::Bool({ let t4167 = Py::Int(0i64); if py::eq(&t4166, &t4167) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t4168 = v_fb.clone(); Py::Bool({ let t4169 = Py::Int(0i64); if py::eq(&t4168, &t4169) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t4170 = { let t4172 = v_fa.clone(); Py::Bool({ let t4173 = Py::Int(0i64); if py::lt(&t4172, &t4173)? { true } else { false } }) }; Py::Bool({ let t4171 = { let t4174 = v_fb.clone(); Py::Bool({ let t4175 = Py::Int(0i64); if py::lt(&t4174, &t4175)? { true } else { false } }) }; if py::ne(&t4170, &t4171) { true } else { false } }) }, Py::str("no solution in the admissible range"))?;
    for t4176 in py::range_iter(&[(v_iters.clone()).i()?]) {
        v__ = t4176;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4181 = { let t4177 = v_m.clone(); Py::Bool({ let t4178 = v_a.clone(); if py::eq(&t4177, &t4178) { true } else { false } }) }; if t4181.truthy() { t4181 } else { { let t4179 = v_m.clone(); Py::Bool({ let t4180 = v_b.clone(); if py::eq(&t4179, &t4180) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4182 = v_fm.clone(); Py::Bool({ let t4183 = Py::Int(0i64); if py::eq(&t4182, &t4183) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4184 = { let t4186 = v_fm.clone(); Py::Bool({ let t4187 = Py::Int(0i64); if py::lt(&t4186, &t4187)? { true } else { false } }) }; Py::Bool({ let t4185 = { let t4188 = v_fa.clone(); Py::Bool({ let t4189 = Py::Int(0i64); if py::lt(&t4188, &t4189)? { true } else { false } }) }; if py::eq(&t4184, &t4185) { true } else { false } }) }).truthy() {
            let t4190 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4190[0].clone();
            v_fa = t4190[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cdf(mut v_z: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_z.clone())?, &py::math1("sqrt", &Py::Float(2.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ceil(mut v_x: Py) -> R {
    let mut v_k: Py = Py::None;
    v_k = py::round(&v_x.clone(), None)?;
    if ({ let t5403 = py::abs(&py::sub(&v_x.clone(), &v_k.clone())?)?; Py::Bool({ let t5404 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?; if py::le(&t5403, &t5404)? { true } else { false } }) }).truthy() {
        return Ok(py::int(&v_k.clone())?);
    }
    return Ok(py::int(&py::math1("ceil", &v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_even_cdf(mut v_x: Py, mut v_m: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    v_h = py::truediv(&v_x.clone(), &Py::Int(2i64))?;
    let t2822 = (Py::tuple(vec![py::math1("exp", &py::neg(&v_h.clone())?)?, Py::Float(0.0)])).items()?;
    if t2822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t2822[0].clone();
    v_s = t2822[1].clone();
    for t2823 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_j = t2823;
        if ({ let t2824 = v_j.clone(); Py::Bool({ let t2825 = Py::Int(0i64); if py::gt(&t2824, &t2825)? { true } else { false } }) }).truthy() {
            v_term = py::mul(&v_term, &py::truediv(&v_h.clone(), &v_j.clone())?)?;
        }
        v_s = py::iadd(&v_s, &v_term.clone())?;
    }
    return Ok(py::sub(&Py::Float(1.0), &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_even_ppf(mut v_p: Py, mut v_m: Py) -> R {
    let mut v_hi: Py = Py::None;
    v_hi = py::add(&py::mul(&Py::Float(2.0), &v_m.clone())?, &Py::Float(10.0))?;
    while ({ let t2820 = f__chi2_even_cdf(v_hi.clone(), v_m.clone())?; Py::Bool({ let t2821 = v_p.clone(); if py::lt(&t2820, &t2821)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Int(2i64))?;
    }
    return Ok(f__bisect_root({ let v_hi = v_hi.clone(); let v_m = v_m.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__chi2_even_cdf(v_x.clone(), v_m.clone())?, &v_p.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_hi.clone(), Py::Int(300i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__close(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t5521 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t5521.truthy() { t5521 } else { Py::Bool(matches!(v_a.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t5522 = v_a.clone(); Py::Bool({ let t5523 = v_b.clone(); if py::eq(&t5522, &t5523) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    if ({ let t5525 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t5525.truthy() { t5525 } else { { let t5524 = py::not(&Py::Bool(matches!(v_a.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5524.truthy() { t5524 } else { py::not(&py::math1("isfinite", &v_a.clone())?) } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    return Ok((if ({ let t5526 = v_d.clone(); Py::Bool({ let t5527 = Py::Float(1e-15); if py::le(&t5526, &t5527)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__erlang_c(mut v_lam: Py, mut v_mu: Py, mut v_c: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_rho = py::truediv(&v_a.clone(), &v_c.clone())?;
    let t3685 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t3685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t3685[0].clone();
    v_s = t3685[1].clone();
    for t3686 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_c.clone()).i()?]) {
        v_k = t3686;
        v_term = py::mul(&v_term, &py::truediv(&v_a.clone(), &v_k.clone())?)?;
        v_s = py::iadd(&v_s, &v_term.clone())?;
    }
    v_last = py::truediv(&py::truediv(&py::mul(&v_term.clone(), &v_a.clone())?, &v_c.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    return Ok(py::truediv(&v_last.clone(), &py::add(&v_s.clone(), &v_last.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__need({ let t5481 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5481.truthy() { t5481 } else { { let t5480 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5480.truthy() { t5480 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let _ = f__need({ let t5489 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5489.truthy() { t5489 } else { { let t5488 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5488.truthy() { t5488 } else { { let t5487 = py::math1("isfinite", &v_v.clone())?; if !t5487.truthy() { t5487 } else { { let t5485 = py::float(&v_v.clone())?; Py::Bool({ let t5486 = py::int(&v_v.clone())?; if py::eq(&t5485, &t5486) { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))?;
    v_v = py::int(&v_v.clone())?;
    let _ = f__need({ let t5490 = v_v.clone(); Py::Bool({ let t5491 = v_lo.clone(); if py::ge(&t5490, &t5491)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
    let _ = f__need({ let t5496 = { let t5492 = v_hi.clone(); Py::Bool({ let t5493 = Py::None; if py::is(&t5492, &t5493) { true } else { false } }) }; if t5496.truthy() { t5496 } else { { let t5494 = v_v.clone(); Py::Bool({ let t5495 = v_hi.clone(); if py::le(&t5494, &t5495)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jobs(mut v_processing_times: Py, mut v_due_dates: Py, mut v_n_max: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = f__vec(Py::str("processing_times"), v_processing_times.clone(), Py::Int(1i64), v_n_max.clone(), Py::Bool(true), Py::Bool(false))?;
    v_d = Py::None;
    if ({ let t4623 = v_due_dates.clone(); Py::Bool({ let t4624 = Py::None; if !py::is(&t4623, &t4624) { true } else { false } }) }).truthy() {
        v_d = f__vec(Py::str("due_dates"), v_due_dates.clone(), Py::Int(1i64), v_n_max.clone(), Py::Bool(false), Py::Bool(true))?;
        let _ = f__need({ let t4625 = py::len(&v_d.clone())?; Py::Bool({ let t4626 = py::len(&v_p.clone())?; if py::eq(&t4625, &t4626) { true } else { false } }) }, Py::str("processing_times and due_dates must have the same number of values"))?;
    }
    return Ok(Py::tuple(vec![v_p.clone(), v_d.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__johnson_order(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_last: Py = Py::None;
    v_first = py::sorted_key(&{ let mut out = vec![]; for t1586 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1586;
if ({ let t1587 = py::getitem(&v_a.clone(), &v_j.clone())?; Py::Bool({ let t1588 = py::getitem(&v_b.clone(), &v_j.clone())?; if py::le(&t1587, &t1588)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }, &{ let v_a = v_a.clone(); let v_b = v_b.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_a.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_last = py::sorted_key(&{ let mut out = vec![]; for t1589 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1589;
if ({ let t1590 = py::getitem(&v_a.clone(), &v_j.clone())?; Py::Bool({ let t1591 = py::getitem(&v_b.clone(), &v_j.clone())?; if py::gt(&t1590, &t1591)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }, &{ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_first = v_first.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_b.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(py::add(&v_first.clone(), &v_last.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__loss(mut v_z: Py) -> R {
    return Ok(py::sub(&f__pdf(v_z.clone())?, &py::mul(&v_z.clone(), &f__sf(v_z.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mat(mut v_name: Py, mut v_m: Py, mut v_rows_min: Py, mut v_cols: Py, mut v_cols_min: Py, mut v_cols_max: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__need({ let t4392 = Py::Bool(matches!(v_m.clone(), Py::List(_) | Py::Tuple(_))); if !t4392.truthy() { t4392 } else { { let t4390 = py::len(&v_m.clone())?; Py::Bool({ let t4391 = v_rows_min.clone(); if py::ge(&t4390, &t4391)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_rows_min.clone(), "")?, " rows".to_string()].concat()))?;
    v_out = Py::list(vec![]);
    for t4393 in py::iter(&(v_m.clone()))? {
        v_r = t4393;
        let _ = f__need(Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_))), Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix (a list of rows)".to_string()].concat()))?;
        let _ = py::method(&v_out.clone(), "append", &[{ let mut out = vec![]; for t4394 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t4394;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) }])?;
    }
    v_w = py::len(&py::getitem(&v_out.clone(), &Py::Int(0i64))?)?;
    for t4397 in py::iter(&(v_out.clone()))? {
        v_r = t4397;
        let _ = f__need({ let t4398 = py::len(&v_r.clone())?; Py::Bool({ let t4399 = v_w.clone(); if py::eq(&t4398, &t4399) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular (rows of equal length)".to_string()].concat()))?;
    }
    let _ = f__need({ let t4404 = { let t4400 = v_cols.clone(); Py::Bool({ let t4401 = Py::None; if py::is(&t4400, &t4401) { true } else { false } }) }; if t4404.truthy() { t4404 } else { { let t4402 = v_w.clone(); Py::Bool({ let t4403 = v_cols.clone(); if py::eq(&t4402, &t4403) { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_cols.clone(), "")?, " columns".to_string()].concat()))?;
    let _ = f__need({ let t4412 = { let t4405 = v_w.clone(); Py::Bool({ let t4406 = v_cols_min.clone(); if py::ge(&t4405, &t4406)? { true } else { false } }) }; if !t4412.truthy() { t4412 } else { { let t4411 = { let t4407 = v_cols_max.clone(); Py::Bool({ let t4408 = Py::None; if py::is(&t4407, &t4408) { true } else { false } }) }; if t4411.truthy() { t4411 } else { { let t4409 = v_w.clone(); Py::Bool({ let t4410 = v_cols_max.clone(); if py::le(&t4409, &t4410)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " has the wrong number of columns".to_string()].concat()))?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__metrics(mut v_order: Py, mut v_p: Py, mut v_due: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_late: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tard: Py = Py::None;
    let t4640 = (Py::tuple(vec![Py::Float(0.0), Py::list(vec![])])).items()?;
    if t4640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t4640[0].clone();
    v_comp = t4640[1].clone();
    for t4641 in py::iter(&(v_order.clone()))? {
        v_j = t4641;
        v_t = py::iadd(&v_t, &py::getitem(&v_p.clone(), &v_j.clone())?)?;
        let _ = py::method(&v_comp.clone(), "append", &[v_t.clone()])?;
    }
    v_n = py::len(&v_order.clone())?;
    v_out = Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t4642 in py::iter(&(v_order.clone()))? { let mut v_j: Py; 
v_j = t4642;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("mean_flow_time"), py::truediv(&py::sum(&v_comp.clone(), None)?, &v_n.clone())?), (Py::str("makespan"), v_t.clone()), (Py::str("average_jobs_in_system"), py::truediv(&py::sum(&v_comp.clone(), None)?, &v_t.clone())?), (Py::str("utilization"), py::truediv(&v_t.clone(), &py::sum(&v_comp.clone(), None)?)?)]);
    if ({ let t4643 = v_due.clone(); Py::Bool({ let t4644 = Py::None; if !py::is(&t4643, &t4644) { true } else { false } }) }).truthy() {
        v_late = { let mut out = vec![]; for t4645 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4645;
out.push(py::sub(&py::getitem(&v_comp.clone(), &v_k.clone())?, &py::getitem(&v_due.clone(), &py::getitem(&v_order.clone(), &v_k.clone())?)?)?); } Py::list(out) };
        v_tard = { let mut out = vec![]; for t4646 in py::iter(&(v_late.clone()))? { let mut v_x: Py; 
v_x = t4646;
out.push(py::minmax(&[Py::Float(0.0), v_x.clone()], true)?); } Py::list(out) };
        let _ = py::method(&v_out.clone(), "update", &[Py::dict(vec![(Py::str("total_tardiness"), py::sum(&v_tard.clone(), None)?), (Py::str("mean_tardiness"), py::truediv(&py::sum(&v_tard.clone(), None)?, &v_n.clone())?), (Py::str("max_tardiness"), py::minmax(&[v_tard.clone()], true)?), (Py::str("max_lateness"), py::minmax(&[v_late.clone()], true)?), (Py::str("tardy_jobs"), py::sum(&{ let mut out = vec![]; for t4647 in py::iter(&(v_tard.clone()))? { let mut v_x: Py; 
v_x = t4647;
if ({ let t4648 = v_x.clone(); Py::Bool({ let t4649 = Py::Int(0i64); if py::gt(&t4648, &t4649)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mg1_values(mut v_lam: Py, mut v_es: Py, mut v_var: Py) -> R {
    let mut v_Lq: Py = Py::None;
    let mut v_Wq: Py = Py::None;
    let mut v_rho: Py = Py::None;
    v_rho = py::mul(&v_lam.clone(), &v_es.clone())?;
    let _ = f__need({ let t2459 = v_rho.clone(); Py::Bool({ let t2460 = Py::Int(1i64); if py::lt(&t2459, &t2460)? { true } else { false } }) }, Py::str("the queue is stable only if arrival rate x mean service time is below 1"))?;
    v_Lq = (if ({ let t2461 = v_var.clone(); Py::Bool({ let t2462 = Py::Int(0i64); if py::gt(&t2461, &t2462)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&v_lam.clone(), &v_lam.clone())?, &py::add(&v_var.clone(), &py::mul(&v_es.clone(), &v_es.clone())?)?)?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)? } else { py::truediv(&py::mul(&py::mul(&py::mul(&v_lam.clone(), &v_lam.clone())?, &v_es.clone())?, &v_es.clone())?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)? });
    v_Wq = py::truediv(&v_Lq.clone(), &v_lam.clone())?;
    return Ok(Py::dict(vec![(Py::str("utilization"), v_rho.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("Wq"), v_Wq.clone()), (Py::str("W"), py::add(&v_Wq.clone(), &v_es.clone())?), (Py::str("L"), py::add(&v_Lq.clone(), &v_rho.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_v: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t5282 = v_v.clone(); Py::Bool({ let t5283 = Py::Int(0i64); if py::ge(&t5282, &t5283)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must not be negative".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nv_costs(mut v_price: Py, mut v_cost: Py, mut v_salvage: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3123 = (Py::tuple(vec![f__pos(Py::str("price"), v_price.clone())?, f__pos(Py::str("cost"), v_cost.clone())?, f__fin(Py::str("salvage"), v_salvage.clone())?])).items()?;
    if t3123.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3123[0].clone();
    v_c = t3123[1].clone();
    v_v = t3123[2].clone();
    let _ = f__need({ let t3124 = v_v.clone(); Py::Bool({ let t3125 = v_c.clone(); if py::lt(&t3124, &t3125)? { { let t3126 = v_p.clone(); if py::lt(&t3125, &t3126)? { true } else { false } } } else { false } }) }, Py::str("the newsvendor needs salvage < cost < price"))?;
    return Ok(Py::tuple(vec![v_p.clone(), v_c.clone(), v_v.clone(), py::truediv(&py::sub(&v_p.clone(), &v_c.clone())?, &py::sub(&v_p.clone(), &v_v.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pair(mut v_actual: Py, mut v_forecast: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_f: Py = Py::None;
    v_a = f__vec(Py::str("actual"), v_actual.clone(), Py::Int(1i64), Py::Int(100000i64), Py::Bool(false), Py::Bool(false))?;
    v_f = f__vec(Py::str("forecast"), v_forecast.clone(), Py::Int(1i64), Py::Int(100000i64), Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t5026 = py::len(&v_a.clone())?; Py::Bool({ let t5027 = py::len(&v_f.clone())?; if py::eq(&t5026, &t5027) { true } else { false } }) }, Py::str("actual and forecast must have the same number of values"))?;
    return Ok(Py::tuple(vec![v_a.clone(), v_f.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pdf(mut v_z: Py) -> R {
    return Ok(py::normal_dist("pdf", &Py::Float(0.0), &Py::Float(1.0), &[v_z.clone()])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_pmfs(mut v_lam: Py, mut v_kmax: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = py::math1("exp", &py::neg(&v_lam.clone())?)?;
    v_out = Py::list(vec![v_p.clone()]);
    for t3076 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_kmax.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t3076;
        v_p = py::truediv(&py::mul(&v_p.clone(), &v_lam.clone())?, &v_k.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[v_p.clone()])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_v: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t5482 = v_v.clone(); Py::Bool({ let t5483 = Py::Int(0i64); if py::gt(&t5482, &t5483)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__precedence(mut v_n: Py, mut v_pairs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_done: Py = Py::None;
    let mut v_free: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_preds: Py = Py::None;
    let mut v_state: Py = Py::None;
    v_m = f__mat(Py::str("precedence"), v_pairs.clone(), Py::Int(1i64), Py::Int(2i64), Py::Int(1i64), Py::None)?;
    v_preds = { let mut out = vec![]; for t3774 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3774;
out.push(Py::list(vec![])); } Py::list(out) };
    for t3775 in py::iter(&(v_m.clone()))? {
        let t3794 = (t3775).items()?;
        if t3794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3794[0].clone();
        v_b = t3794[1].clone();
        let _ = f__need({ let t3791 = { let t3776 = v_a.clone(); Py::Bool({ let t3777 = py::int(&v_a.clone())?; if py::eq(&t3776, &t3777) { true } else { false } }) }; if !t3791.truthy() { t3791 } else { { let t3790 = { let t3778 = v_b.clone(); Py::Bool({ let t3779 = py::int(&v_b.clone())?; if py::eq(&t3778, &t3779) { true } else { false } }) }; if !t3790.truthy() { t3790 } else { { let t3789 = { let t3780 = Py::Int(1i64); Py::Bool({ let t3781 = v_a.clone(); if py::le(&t3780, &t3781)? { { let t3782 = v_n.clone(); if py::le(&t3781, &t3782)? { true } else { false } } } else { false } }) }; if !t3789.truthy() { t3789 } else { { let t3788 = { let t3783 = Py::Int(1i64); Py::Bool({ let t3784 = v_b.clone(); if py::le(&t3783, &t3784)? { { let t3785 = v_n.clone(); if py::le(&t3784, &t3785)? { true } else { false } } } else { false } }) }; if !t3788.truthy() { t3788 } else { { let t3786 = v_a.clone(); Py::Bool({ let t3787 = v_b.clone(); if py::ne(&t3786, &t3787) { true } else { false } }) } } } } } } } } }, Py::str("each precedence row [a, b] names two different task numbers (a before b)"))?;
        if ({ let t3792 = py::sub(&py::int(&v_a.clone())?, &Py::Int(1i64))?; Py::Bool({ let t3793 = py::getitem(&v_preds.clone(), &py::sub(&py::int(&v_b.clone())?, &Py::Int(1i64))?)?; if !py::contains(&t3793, &t3792)? { true } else { false } }) }).truthy() {
            let _ = py::method(&py::getitem(&v_preds.clone(), &py::sub(&py::int(&v_b.clone())?, &Py::Int(1i64))?)?, "append", &[py::sub(&py::int(&v_a.clone())?, &Py::Int(1i64))?])?;
        }
    }
    v_state = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
    v_left = v_n.clone();
    v_done = py::mul(&Py::list(vec![Py::Bool(false)]), &v_n.clone())?;
    while (v_left.clone()).truthy() {
        v_free = { let mut out = vec![]; for t3795 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3795;
if ({ let t3797 = py::not(&py::getitem(&v_done.clone(), &v_j.clone())?); if !t3797.truthy() { t3797 } else { Py::Bool(({ let mut out = vec![]; for t3796 in py::iter(&(py::getitem(&v_preds.clone(), &v_j.clone())?))? { let mut v_p: Py; 
v_p = t3796;
out.push(py::getitem(&v_done.clone(), &v_p.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
        let _ = f__need({ let t3798 = py::len(&v_free.clone())?; Py::Bool({ let t3799 = Py::Int(0i64); if py::gt(&t3798, &t3799)? { true } else { false } }) }, Py::str("the precedence relations contain a cycle"))?;
        for t3800 in py::iter(&(v_free.clone()))? {
            v_j = t3800;
            py::setitem(&v_done.clone(), &v_j.clone(), Py::Bool(true))?;
            v_left = py::sub(&v_left, &Py::Int(1i64))?;
        }
    }
    return Ok(v_preds.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prob(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t5387 = v_lo.clone(); Py::Bool({ let t5388 = v_v.clone(); if py::lt(&t5387, &t5388)? { { let t5389 = v_hi.clone(); if py::lt(&t5388, &t5389)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie strictly between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?, " (write 95% or 0.95)".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    if ({ let t5501 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5501.truthy() { t5501 } else { { let t5500 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5500.truthy() { t5500 } else { { let t5498 = v_x.clone(); Py::Bool({ let t5499 = Py::None; if py::is(&t5498, &t5499) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5502 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5502;
out.push(f__r12(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5505 = py::not(&py::math1("isfinite", &v_x.clone())?); if t5505.truthy() { t5505 } else { { let t5503 = v_x.clone(); Py::Bool({ let t5504 = Py::Float(0.0); if py::eq(&t5503, &t5504) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5506 = v_x.clone(); Py::Bool({ let t5507 = Py::Float(0.0); if py::eq(&t5506, &t5507) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rd(mut v_d: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::dict(vec![]);
    for t5497 in py::iter(&(py::method(&v_d.clone(), "items", &[])?))? {
        let t5508 = (t5497).items()?;
        if t5508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5508[0].clone();
        v_v = t5508[1].clone();
        py::setitem(&v_out.clone(), &v_k.clone(), f__r12(v_v.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel_list(mut v_name: Py, mut v_rs: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_r = f__vec(v_name.clone(), v_rs.clone(), Py::Int(1i64), Py::Int(500i64), Py::Bool(false), Py::Bool(false))?;
    for t4471 in py::iter(&(v_r.clone()))? {
        v_v = t4471;
        let _ = f__need({ let t4472 = Py::Int(0i64); Py::Bool({ let t4473 = v_v.clone(); if py::le(&t4472, &t4473)? { { let t4474 = Py::Int(1i64); if py::le(&t4473, &t4474)? { true } else { false } } } else { false } }) }, Py::str(&["every value of ".to_string(), py::format(&v_name.clone(), "")?, " must lie between 0 and 1".to_string()].concat()))?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__residual(mut v_got: Py, mut v_want: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_worst = Py::Float(0.0);
    for t5510 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5530 = (t5510).items()?;
        if t5530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5530[0].clone();
        v_w = t5530[1].clone();
        if ({ let t5511 = v_k.clone(); Py::Bool({ let t5512 = v_got.clone(); if !py::contains(&t5512, &t5511)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_o = py::getitem(&v_got.clone(), &v_k.clone())?;
        if (Py::Bool(matches!(v_w.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
            if ({ let t5515 = py::not(&Py::Bool(matches!(v_o.clone(), Py::List(_) | Py::Tuple(_)))); if t5515.truthy() { t5515 } else { { let t5513 = py::len(&v_o.clone())?; Py::Bool({ let t5514 = py::len(&v_w.clone())?; if py::ne(&t5513, &t5514) { true } else { false } }) } } }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            for t5516 in py::iter(&(py::zip(&[v_o.clone(), v_w.clone()])?))? {
                let t5529 = (t5516).items()?;
                if t5529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_a = t5529[0].clone();
                v_b = t5529[1].clone();
                if (Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
                    if ({ let t5519 = py::not(&Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_)))); if t5519.truthy() { t5519 } else { { let t5517 = py::len(&v_a.clone())?; Py::Bool({ let t5518 = py::len(&v_b.clone())?; if py::ne(&t5517, &t5518) { true } else { false } }) } } }).truthy() {
                        return Ok(Py::Float(f64::INFINITY));
                    }
                    for t5520 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
                        let t5528 = (t5520).items()?;
                        if t5528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_a2 = t5528[0].clone();
                        v_b2 = t5528[1].clone();
                        v_worst = py::minmax(&[v_worst.clone(), f__close(v_a2.clone(), v_b2.clone())?], true)?;
                    }
                } else {
                    v_worst = py::minmax(&[v_worst.clone(), f__close(v_a.clone(), v_b.clone())?], true)?;
                }
            }
        } else {
            v_worst = py::minmax(&[v_worst.clone(), f__close(v_o.clone(), v_w.clone())?], true)?;
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sf(mut v_z: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_z.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__three_machine_span(mut v_order: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let mut v_t3: Py = Py::None;
    let t1541 = Py::Float(0.0);
    v_t1 = t1541.clone();
    v_t2 = t1541.clone();
    v_t3 = t1541.clone();
    for t1542 in py::iter(&(v_order.clone()))? {
        v_j = t1542;
        v_t1 = py::iadd(&v_t1, &py::getitem(&v_a.clone(), &v_j.clone())?)?;
        v_t2 = py::add(&py::minmax(&[v_t2.clone(), v_t1.clone()], true)?, &py::getitem(&v_b.clone(), &v_j.clone())?)?;
        v_t3 = py::add(&py::minmax(&[v_t3.clone(), v_t2.clone()], true)?, &py::getitem(&v_c.clone(), &v_j.clone())?)?;
    }
    return Ok(v_t3.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_machine_span(mut v_order: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t1592 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t1592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t1592[0].clone();
    v_t2 = t1592[1].clone();
    for t1593 in py::iter(&(v_order.clone()))? {
        v_j = t1593;
        v_t1 = py::iadd(&v_t1, &py::getitem(&v_a.clone(), &v_j.clone())?)?;
        v_t2 = py::add(&py::minmax(&[v_t2.clone(), v_t1.clone()], true)?, &py::getitem(&v_b.clone(), &v_j.clone())?)?;
    }
    return Ok(v_t2.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_name: Py, mut v_xs: Py, mut v_n_min: Py, mut v_n_max: Py, mut v_positive: Py, mut v_nonneg: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t5329 = Py::Bool(matches!(v_xs.clone(), Py::List(_) | Py::Tuple(_))); if !t5329.truthy() { t5329 } else { { let t5327 = py::len(&v_xs.clone())?; Py::Bool({ let t5328 = v_n_min.clone(); if py::ge(&t5327, &t5328)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " values".to_string()].concat()))?;
    let _ = f__need({ let t5334 = { let t5330 = v_n_max.clone(); Py::Bool({ let t5331 = Py::None; if py::is(&t5330, &t5331) { true } else { false } }) }; if t5334.truthy() { t5334 } else { { let t5332 = py::len(&v_xs.clone())?; Py::Bool({ let t5333 = v_n_max.clone(); if py::le(&t5332, &t5333)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " takes at most ".to_string(), py::format(&v_n_max.clone(), "")?, " values".to_string()].concat()))?;
    v_out = { let mut out = vec![]; for t5335 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t5335;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) };
    for t5338 in py::iter(&(v_out.clone()))? {
        v_x = t5338;
        let _ = f__need({ let t5341 = py::not(&v_positive.clone()); if t5341.truthy() { t5341 } else { { let t5339 = v_x.clone(); Py::Bool({ let t5340 = Py::Int(0i64); if py::gt(&t5339, &t5340)? { true } else { false } }) } } }, Py::str(&["every value of ".to_string(), py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
        let _ = f__need({ let t5344 = py::not(&v_nonneg.clone()); if t5344.truthy() { t5344 } else { { let t5342 = v_x.clone(); Py::Bool({ let t5343 = Py::Int(0i64); if py::ge(&t5342, &t5343)? { true } else { false } }) } } }, Py::str(&["no value of ".to_string(), py::format(&v_name.clone(), "")?, " may be negative".to_string()].concat()))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__weber_vertex(mut v_xs: Py, mut v_ys: Py, mut v_w: Py, mut v_i: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_gx: Py = Py::None;
    let mut v_gy: Py = Py::None;
    let mut v_j: Py = Py::None;
    let t5155 = Py::Float(0.0);
    v_gx = t5155.clone();
    v_gy = t5155.clone();
    for t5156 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_j = t5156;
        if ({ let t5157 = v_j.clone(); Py::Bool({ let t5158 = v_i.clone(); if py::eq(&t5157, &t5158) { true } else { false } }) }).truthy() {
            continue;
        }
        v_d = py::hypot(&[py::sub(&py::getitem(&v_xs.clone(), &v_i.clone())?, &py::getitem(&v_xs.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_ys.clone(), &v_i.clone())?, &py::getitem(&v_ys.clone(), &v_j.clone())?)?])?;
        if ({ let t5159 = v_d.clone(); Py::Bool({ let t5160 = Py::Int(0i64); if py::gt(&t5159, &t5160)? { true } else { false } }) }).truthy() {
            v_gx = py::iadd(&v_gx, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::sub(&py::getitem(&v_xs.clone(), &v_i.clone())?, &py::getitem(&v_xs.clone(), &v_j.clone())?)?)?, &v_d.clone())?)?;
            v_gy = py::iadd(&v_gy, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::sub(&py::getitem(&v_ys.clone(), &v_i.clone())?, &py::getitem(&v_ys.clone(), &v_j.clone())?)?)?, &v_d.clone())?)?;
        }
    }
    return Ok({ let t5161 = py::hypot(&[v_gx.clone(), v_gy.clone()])?; Py::Bool({ let t5162 = py::getitem(&v_w.clone(), &v_i.clone())?; if py::le(&t5161, &t5162)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wmedian(mut v_v: Py, mut v_w: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_half: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nxt: Py = Py::None;
    let mut v_order: Py = Py::None;
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_v.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_v = v_v.clone(); let v_w = v_w.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_v.clone(), &v_k.clone())?, v_k.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    let t3891 = (Py::tuple(vec![py::truediv(&py::sum(&v_w.clone(), None)?, &Py::Int(2i64))?, Py::Float(0.0)])).items()?;
    if t3891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_half = t3891[0].clone();
    v_acc = t3891[1].clone();
    for t3892 in py::range_iter(&[(py::len(&v_order.clone())?).i()?]) {
        v_idx = t3892;
        v_k = py::getitem(&v_order.clone(), &v_idx.clone())?;
        v_acc = py::iadd(&v_acc, &py::getitem(&v_w.clone(), &v_k.clone())?)?;
        if ({ let t3893 = v_acc.clone(); Py::Bool({ let t3894 = v_half.clone(); if py::eq(&t3893, &t3894) { true } else { false } }) }).truthy() {
            v_nxt = (if ({ let t3895 = py::add(&v_idx.clone(), &Py::Int(1i64))?; Py::Bool({ let t3896 = py::len(&v_order.clone())?; if py::lt(&t3895, &t3896)? { true } else { false } }) }).truthy() { py::getitem(&v_v.clone(), &py::getitem(&v_order.clone(), &py::add(&v_idx.clone(), &Py::Int(1i64))?)?)? } else { py::getitem(&v_v.clone(), &v_k.clone())? });
            let _ = f__need({ let t3897 = v_nxt.clone(); Py::Bool({ let t3898 = py::getitem(&v_v.clone(), &v_k.clone())?; if py::eq(&t3897, &t3898) { true } else { false } }) }, Py::str(&["every value between ".to_string(), py::format(&py::getitem(&v_v.clone(), &v_k.clone())?, "g")?, " and ".to_string(), py::format(&v_nxt.clone(), "g")?, " is optimal (the median is not unique)".to_string()].concat()))?;
        }
        if ({ let t3899 = v_acc.clone(); Py::Bool({ let t3900 = v_half.clone(); if py::ge(&t3899, &t3900)? { true } else { false } }) }).truthy() {
            return Ok(py::getitem(&v_v.clone(), &v_k.clone())?);
        }
    }
    return Ok(py::getitem(&v_v.clone(), &py::getitem(&v_order.clone(), &py::neg(&Py::Int(1i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xyw(mut v_x: Py, mut v_y: Py, mut v_weights: Py) -> R {
    let mut v_w: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t5138 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t5138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t5138[0].clone();
    v_ys = t5138[1].clone();
    v_w = f__vec(Py::str("weights"), v_weights.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t5139 = py::len(&v_xs.clone())?; Py::Bool({ let t5140 = py::len(&v_ys.clone())?; if py::eq(&t5139, &t5140) { { let t5141 = py::len(&v_w.clone())?; if py::eq(&t5140, &t5141) { true } else { false } } } else { false } }) }, Py::str("x, y and weights must have the same number of values"))?;
    let _ = f__need({ let t5142 = py::sum(&v_w.clone(), None)?; Py::Bool({ let t5143 = Py::Int(0i64); if py::gt(&t5142, &t5143)? { true } else { false } }) }, Py::str("the weights must not all be 0"))?;
    return Ok(Py::tuple(vec![v_xs.clone(), v_ys.clone(), v_w.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zinv(mut v_p: Py) -> R {
    return Ok(py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_p.clone()])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_abc_classification(mut v_annual_values: Py, mut v_a_share: Py, mut v_b_share: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_before: Py = Py::None;
    let mut v_cls: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_na: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_share: Py = Py::None;
    let mut v_share_a: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = f__vec(Py::str("annual_values"), v_annual_values.clone(), Py::Int(2i64), Py::Int(1000i64), Py::Bool(false), Py::Bool(true))?;
    let t22 = (Py::tuple(vec![f__prob(Py::str("a_share"), v_a_share.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("b_share"), v_b_share.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t22.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t22[0].clone();
    v_b = t22[1].clone();
    let _ = f__need({ let t23 = v_a.clone(); Py::Bool({ let t24 = v_b.clone(); if py::lt(&t23, &t24)? { true } else { false } }) }, Py::str("a_share must be below b_share"))?;
    v_tot = py::sum(&v_v.clone(), None)?;
    let _ = f__need({ let t25 = v_tot.clone(); Py::Bool({ let t26 = Py::Int(0i64); if py::gt(&t25, &t26)? { true } else { false } }) }, Py::str("the annual values must not all be 0"))?;
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_v.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_a = v_a.clone(); let v_a_share = v_a_share.clone(); let v_annual_values = v_annual_values.clone(); let v_b = v_b.clone(); let v_b_share = v_b_share.clone(); let v_tot = v_tot.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_v.clone(), &v_k.clone())?)?, v_k.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_cls = py::mul(&Py::list(vec![Py::str("C")]), &py::len(&v_v.clone())?)?;
    v_before = Py::Float(0.0);
    for t27 in py::iter(&(v_order.clone()))? {
        v_k = t27;
        v_share = py::truediv(&v_before.clone(), &v_tot.clone())?;
        py::setitem(&v_cls.clone(), &v_k.clone(), (if ({ let t28 = v_share.clone(); Py::Bool({ let t29 = v_a.clone(); if py::lt(&t28, &t29)? { true } else { false } }) }).truthy() { Py::str("A") } else { (if ({ let t30 = v_share.clone(); Py::Bool({ let t31 = v_b.clone(); if py::lt(&t30, &t31)? { true } else { false } }) }).truthy() { Py::str("B") } else { Py::str("C") }) }))?;
        v_before = py::iadd(&v_before, &py::getitem(&v_v.clone(), &v_k.clone())?)?;
    }
    let t32 = (Py::tuple(vec![py::method(&v_cls.clone(), "count", &[Py::str("A")])?, py::method(&v_cls.clone(), "count", &[Py::str("B")])?])).items()?;
    if t32.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_na = t32[0].clone();
    v_nb = t32[1].clone();
    v_share_a = py::truediv(&py::sum(&{ let mut out = vec![]; for t33 in py::range_iter(&[(py::len(&v_v.clone())?).i()?]) { let mut v_k: Py; 
v_k = t33;
if ({ let t34 = py::getitem(&v_cls.clone(), &v_k.clone())?; Py::Bool({ let t35 = Py::str("A"); if py::eq(&t34, &t35) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_v.clone(), &v_k.clone())?); }} Py::list(out) }, None)?, &v_tot.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("classes"), v_cls.clone()), (Py::str("count_a"), v_na.clone()), (Py::str("count_b"), v_nb.clone()), (Py::str("count_c"), py::sub(&py::sub(&py::len(&v_v.clone())?, &v_na.clone())?, &v_nb.clone())?), (Py::str("share_a"), v_share_a.clone()), (Py::str("share_b"), py::truediv(&py::sum(&{ let mut out = vec![]; for t48 in py::range_iter(&[(py::len(&v_v.clone())?).i()?]) { let mut v_k: Py; 
v_k = t48;
if ({ let t49 = py::getitem(&v_cls.clone(), &v_k.clone())?; Py::Bool({ let t50 = Py::str("B"); if py::eq(&t49, &t50) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_v.clone(), &v_k.clone())?); }} Py::list(out) }, None)?, &v_tot.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_acceptance_sampling(mut v_sample_size: Py, mut v_acceptance_number: Py, mut v_fraction_defective: Py, mut v_lot_size: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pa: Py = Py::None;
    let t86 = (Py::tuple(vec![f__int(Py::str("sample_size"), v_sample_size.clone(), Py::Int(1i64), Py::Int(2000i64))?, f__int(Py::str("acceptance_number"), v_acceptance_number.clone(), Py::Int(0i64), Py::None)?])).items()?;
    if t86.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t86[0].clone();
    v_c = t86[1].clone();
    let t92 = (Py::tuple(vec![f__prob(Py::str("fraction_defective"), v_fraction_defective.clone(), Py::Float(0.0), Py::Float(1.0))?, f__int(Py::str("lot_size"), v_lot_size.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t92.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t92[0].clone();
    v_N = t92[1].clone();
    let _ = f__need({ let t93 = v_c.clone(); Py::Bool({ let t94 = v_n.clone(); if py::lt(&t93, &t94)? { true } else { false } }) }, Py::str("the acceptance number must be below the sample size"))?;
    let _ = f__need({ let t95 = v_N.clone(); Py::Bool({ let t96 = v_n.clone(); if py::ge(&t95, &t96)? { true } else { false } }) }, Py::str("the lot must be at least as large as the sample"))?;
    v_pa = f__binom_cdf(v_c.clone(), v_n.clone(), v_p.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("probability_accept"), v_pa.clone()), (Py::str("aoq"), py::truediv(&py::mul(&py::mul(&v_pa.clone(), &v_p.clone())?, &py::sub(&v_N.clone(), &v_n.clone())?)?, &v_N.clone())?), (Py::str("ati"), py::add(&v_n.clone(), &py::mul(&py::sub(&Py::Int(1i64), &v_pa.clone())?, &py::sub(&v_N.clone(), &v_n.clone())?)?)?), (Py::str("probability_reject"), py::sub(&Py::Int(1i64), &v_pa.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arl_shewhart(mut v_shift: Py, mut v_subgroup_size: Py, mut v_limit_sigma: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_d = f__nonneg(Py::str("shift"), v_shift.clone())?;
    v_n = f__int(Py::str("subgroup_size"), v_subgroup_size.clone(), Py::Int(1i64), Py::Int(1000i64))?;
    v_L = f__pos(Py::str("limit_sigma"), v_limit_sigma.clone())?;
    let _ = f__need({ let t151 = v_L.clone(); Py::Bool({ let t152 = Py::Int(6i64); if py::le(&t151, &t152)? { true } else { false } }) }, Py::str("limit_sigma must be at most 6"))?;
    v_a = py::mul(&v_d.clone(), &py::math1("sqrt", &v_n.clone())?)?;
    v_beta = py::sub(&f__cdf(py::sub(&v_L.clone(), &v_a.clone())?)?, &f__cdf(py::sub(&py::neg(&v_L.clone())?, &v_a.clone())?)?)?;
    v_alpha = py::mul(&Py::Int(2i64), &f__sf(v_L.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("arl"), py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_beta.clone())?)?), (Py::str("beta"), v_beta.clone()), (Py::str("detection_probability"), py::sub(&Py::Int(1i64), &v_beta.clone())?), (Py::str("arl_in_control"), py::truediv(&Py::Int(1i64), &v_alpha.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_availability(mut v_mtbf: Py, mut v_mttr: Py, mut v_hours_per_year: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_down: Py = Py::None;
    let mut v_hpy: Py = Py::None;
    let mut v_up: Py = Py::None;
    let t192 = (Py::tuple(vec![f__pos(Py::str("mtbf"), v_mtbf.clone())?, f__pos(Py::str("mttr"), v_mttr.clone())?])).items()?;
    if t192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_up = t192[0].clone();
    v_down = t192[1].clone();
    v_hpy = f__pos(Py::str("hours_per_year"), v_hours_per_year.clone())?;
    v_A = py::truediv(&v_up.clone(), &py::add(&v_up.clone(), &v_down.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("availability"), v_A.clone()), (Py::str("unavailability"), py::truediv(&v_down.clone(), &py::add(&v_up.clone(), &v_down.clone())?)?), (Py::str("downtime_per_year"), py::truediv(&py::mul(&v_hpy.clone(), &v_down.clone())?, &py::add(&v_up.clone(), &v_down.clone())?)?), (Py::str("failure_rate"), py::truediv(&Py::Int(1i64), &v_up.clone())?), (Py::str("repair_rate"), py::truediv(&Py::Int(1i64), &v_down.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_break_even(mut v_fixed_cost: Py, mut v_price: Py, mut v_variable_cost: Py, mut v_target_profit: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_cm: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t234 = (Py::tuple(vec![f__nonneg(Py::str("fixed_cost"), v_fixed_cost.clone())?, f__pos(Py::str("price"), v_price.clone())?, f__nonneg(Py::str("variable_cost"), v_variable_cost.clone())?])).items()?;
    if t234.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t234[0].clone();
    v_p = t234[1].clone();
    v_v = t234[2].clone();
    let _ = f__need({ let t235 = v_p.clone(); Py::Bool({ let t236 = v_v.clone(); if py::gt(&t235, &t236)? { true } else { false } }) }, Py::str("the price must exceed the variable cost per unit"))?;
    v_cm = py::sub(&v_p.clone(), &v_v.clone())?;
    v_out = Py::dict(vec![(Py::str("break_even_units"), py::truediv(&v_F.clone(), &v_cm.clone())?), (Py::str("break_even_revenue"), py::truediv(&py::mul(&v_p.clone(), &v_F.clone())?, &v_cm.clone())?), (Py::str("contribution_margin"), v_cm.clone()), (Py::str("contribution_margin_ratio"), py::truediv(&v_cm.clone(), &v_p.clone())?)]);
    if ({ let t237 = v_target_profit.clone(); Py::Bool({ let t238 = Py::None; if !py::is(&t237, &t238) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("units_for_target_profit"), py::truediv(&py::add(&v_F.clone(), &f__fin(Py::str("target_profit"), v_target_profit.clone())?)?, &v_cm.clone())?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_c_chart_limits(mut v_c_bar: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_c = f__pos(Py::str("c_bar"), v_c_bar.clone())?;
    v_s = py::math1("sqrt", &v_c.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("ucl"), py::add(&v_c.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?), (Py::str("lcl"), py::minmax(&[Py::Float(0.0), py::sub(&v_c.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?), (Py::str("center"), v_c.clone()), (Py::str("sigma_c"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capability(mut v_usl: Py, mut v_lsl: Py, mut v_mean: Py, mut v_sd: Py) -> R {
    let mut v_cpl: Py = Py::None;
    let mut v_cpu: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t3566 = (Py::tuple(vec![f__fin(Py::str("usl"), v_usl.clone())?, f__fin(Py::str("lsl"), v_lsl.clone())?, f__fin(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?])).items()?;
    if t3566.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3566[0].clone();
    v_l = t3566[1].clone();
    v_m = t3566[2].clone();
    v_s = t3566[3].clone();
    let _ = f__need({ let t3567 = v_u.clone(); Py::Bool({ let t3568 = v_l.clone(); if py::gt(&t3567, &t3568)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    let t3569 = (Py::tuple(vec![py::truediv(&py::sub(&v_u.clone(), &v_m.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?, py::truediv(&py::sub(&v_m.clone(), &v_l.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?])).items()?;
    if t3569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cpu = t3569[0].clone();
    v_cpl = t3569[1].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("cpk"), py::minmax(&[v_cpu.clone(), v_cpl.clone()], false)?), (Py::str("cp"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("cpu"), v_cpu.clone()), (Py::str("cpl"), v_cpl.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capacity_measures(mut v_design_capacity: Py, mut v_effective_capacity: Py, mut v_actual_output: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let t319 = (Py::tuple(vec![f__pos(Py::str("design_capacity"), v_design_capacity.clone())?, f__pos(Py::str("effective_capacity"), v_effective_capacity.clone())?, f__nonneg(Py::str("actual_output"), v_actual_output.clone())?])).items()?;
    if t319.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t319[0].clone();
    v_e = t319[1].clone();
    v_a = t319[2].clone();
    let _ = f__need({ let t320 = v_e.clone(); Py::Bool({ let t321 = v_d.clone(); if py::le(&t320, &t321)? { true } else { false } }) }, Py::str("effective capacity cannot exceed design capacity"))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("utilization"), py::truediv(&v_a.clone(), &v_d.clone())?), (Py::str("efficiency"), py::truediv(&v_a.clone(), &v_e.clone())?), (Py::str("effective_share"), py::truediv(&v_e.clone(), &v_d.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_center_of_gravity(mut v_x: Py, mut v_y: Py, mut v_weights: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t381 = (f__xyw(v_x.clone(), v_y.clone(), v_weights.clone())?).items()?;
    if t381.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t381[0].clone();
    v_ys = t381[1].clone();
    v_w = t381[2].clone();
    v_W = py::sum(&v_w.clone(), None)?;
    v_cx = py::truediv(&py::sum(&{ let mut out = vec![]; for t382 in py::iter(&(py::zip(&[v_xs.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t383 = (t382).items()?;
if t383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t383[0].clone();
v_b = t383[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
    v_cy = py::truediv(&py::sum(&{ let mut out = vec![]; for t384 in py::iter(&(py::zip(&[v_ys.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t385 = (t384).items()?;
if t385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t385[0].clone();
v_b = t385[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("x"), v_cx.clone()), (Py::str("y"), v_cy.clone()), (Py::str("total_weight"), v_W.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cpm_index(mut v_usl: Py, mut v_lsl: Py, mut v_mean: Py, mut v_sd: Py, mut v_target: Py) -> R {
    let mut v_l: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t425 = (Py::tuple(vec![f__fin(Py::str("usl"), v_usl.clone())?, f__fin(Py::str("lsl"), v_lsl.clone())?, f__fin(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?, f__fin(Py::str("target"), v_target.clone())?])).items()?;
    if t425.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_u = t425[0].clone();
    v_l = t425[1].clone();
    v_m = t425[2].clone();
    v_s = t425[3].clone();
    v_t = t425[4].clone();
    let _ = f__need({ let t426 = v_u.clone(); Py::Bool({ let t427 = v_l.clone(); if py::gt(&t426, &t427)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    let _ = f__need({ let t428 = v_l.clone(); Py::Bool({ let t429 = v_t.clone(); if py::lt(&t428, &t429)? { { let t430 = v_u.clone(); if py::lt(&t429, &t430)? { true } else { false } } } else { false } }) }, Py::str("the target must lie between the specification limits"))?;
    v_tau = py::math1("sqrt", &py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::pow(&py::sub(&v_m.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("cpm"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_tau.clone())?)?), (Py::str("cp"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dispatch_rule(mut v_processing_times: Py, mut v_due_dates: Py, mut v_rule: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t488 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t488[0].clone();
    v_d = t488[1].clone();
    let _ = f__need({ let t489 = v_rule.clone(); Py::Bool({ let t490 = c__RULES()?; if py::contains(&t490, &t489)? { true } else { false } }) }, Py::str("rule is one of fcfs, spt, lpt, edd, cr"))?;
    v_n = py::len(&v_p.clone())?;
    if ({ let t491 = v_rule.clone(); Py::Bool({ let t492 = Py::str("fcfs"); if py::eq(&t491, &t492) { true } else { false } }) }).truthy() {
        v_order = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    } else {
        if ({ let t493 = v_rule.clone(); Py::Bool({ let t494 = Py::str("spt"); if py::eq(&t493, &t494) { true } else { false } }) }).truthy() {
            v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_n = v_n.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); let v_rule = v_rule.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_p.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
        } else {
            if ({ let t495 = v_rule.clone(); Py::Bool({ let t496 = Py::str("lpt"); if py::eq(&t495, &t496) { true } else { false } }) }).truthy() {
                v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_n = v_n.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); let v_rule = v_rule.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_p.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
            } else {
                if ({ let t497 = v_rule.clone(); Py::Bool({ let t498 = Py::str("edd"); if py::eq(&t497, &t498) { true } else { false } }) }).truthy() {
                    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_n = v_n.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); let v_rule = v_rule.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_d.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
                } else {
                    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_n = v_n.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); let v_rule = v_rule.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::truediv(&py::getitem(&v_d.clone(), &v_j.clone())?, &py::getitem(&v_p.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
                }
            }
        }
    }
    return Ok(f__rd(f__metrics(v_order.clone(), v_p.clone(), v_d.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dpmo_sigma(mut v_defects: Py, mut v_units: Py, mut v_opportunities: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_dpo: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t550 = (Py::tuple(vec![f__nonneg(Py::str("defects"), v_defects.clone())?, f__pos(Py::str("units"), v_units.clone())?])).items()?;
    if t550.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t550[0].clone();
    v_u = t550[1].clone();
    v_o = f__pos(Py::str("opportunities"), v_opportunities.clone())?;
    v_dpo = py::truediv(&v_d.clone(), &py::mul(&v_u.clone(), &v_o.clone())?)?;
    let _ = f__need({ let t551 = Py::Int(0i64); Py::Bool({ let t552 = v_dpo.clone(); if py::lt(&t551, &t552)? { { let t553 = Py::Int(1i64); if py::lt(&t552, &t553)? { true } else { false } } } else { false } }) }, Py::str("defects per opportunity must lie strictly between 0 and 1"))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("dpmo"), py::mul(&v_dpo.clone(), &Py::Float(1000000.0))?), (Py::str("dpu"), py::truediv(&v_d.clone(), &v_u.clone())?), (Py::str("dpo"), v_dpo.clone()), (Py::str("yield"), py::sub(&Py::Int(1i64), &v_dpo.clone())?), (Py::str("sigma_level"), py::add(&f__zinv(py::sub(&Py::Int(1i64), &v_dpo.clone())?)?, &Py::Float(1.5))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_edd_sequence(mut v_processing_times: Py, mut v_due_dates: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t611 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t611[0].clone();
    v_d = t611[1].clone();
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_d.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(f__rd(f__metrics(v_order.clone(), v_p.clone(), v_d.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eoq(mut v_demand: Py, mut v_order_cost: Py, mut v_holding_cost: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let t661 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t661.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t661[0].clone();
    v_S = t661[1].clone();
    v_H = t661[2].clone();
    v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("orders_per_year"), py::truediv(&v_D.clone(), &v_Q.clone())?), (Py::str("cycle_time"), py::truediv(&v_Q.clone(), &v_D.clone())?), (Py::str("ordering_cost"), py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Q.clone())?), (Py::str("holding_cost_per_year"), py::truediv(&py::mul(&v_H.clone(), &v_Q.clone())?, &Py::Int(2i64))?), (Py::str("total_cost"), py::math1("sqrt", &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eoq_backorders(mut v_demand: Py, mut v_order_cost: Py, mut v_holding_cost: Py, mut v_backorder_cost: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t701 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t701.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t701[0].clone();
    v_S = t701[1].clone();
    v_H = t701[2].clone();
    v_B = f__pos(Py::str("backorder_cost"), v_backorder_cost.clone())?;
    v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?, &py::add(&v_H.clone(), &v_B.clone())?)?, &v_B.clone())?)?;
    v_b = py::truediv(&py::mul(&v_Q.clone(), &v_H.clone())?, &py::add(&v_H.clone(), &v_B.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("max_backorder"), v_b.clone()), (Py::str("max_inventory"), py::sub(&v_Q.clone(), &v_b.clone())?), (Py::str("total_cost"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?, &v_B.clone())?, &py::add(&v_H.clone(), &v_B.clone())?)?)?), (Py::str("orders_per_year"), py::truediv(&v_D.clone(), &v_Q.clone())?), (Py::str("fraction_backordered_time"), py::truediv(&v_H.clone(), &py::add(&v_H.clone(), &v_B.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eoq_discount(mut v_demand: Py, mut v_order_cost: Py, mut v_holding_rate: Py, mut v_break_quantities: Py, mut v_unit_prices: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qs: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let mut v_tier: Py = Py::None;
    let t744 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__prob(Py::str("holding_rate"), v_holding_rate.clone(), Py::Float(0.0), Py::Float(10.0))?])).items()?;
    if t744.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t744[0].clone();
    v_S = t744[1].clone();
    v_i = t744[2].clone();
    v_qs = f__vec(Py::str("break_quantities"), v_break_quantities.clone(), Py::Int(1i64), Py::Int(50i64), Py::Bool(false), Py::Bool(true))?;
    v_ps = f__vec(Py::str("unit_prices"), v_unit_prices.clone(), Py::Int(1i64), Py::Int(50i64), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t761 = py::len(&v_qs.clone())?; Py::Bool({ let t762 = py::len(&v_ps.clone())?; if py::eq(&t761, &t762) { true } else { false } }) }, Py::str("break_quantities and unit_prices must have the same number of values"))?;
    let _ = f__need({ let t763 = py::getitem(&v_qs.clone(), &Py::Int(0i64))?; Py::Bool({ let t764 = Py::Int(0i64); if py::eq(&t763, &t764) { true } else { false } }) }, Py::str("the first break quantity must be 0 (the price for any quantity)"))?;
    for t765 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_qs.clone())?).i()?]) {
        v_k = t765;
        let _ = f__need({ let t766 = py::getitem(&v_qs.clone(), &v_k.clone())?; Py::Bool({ let t767 = py::getitem(&v_qs.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t766, &t767)? { true } else { false } }) }, Py::str("break quantities must increase"))?;
        let _ = f__need({ let t768 = py::getitem(&v_ps.clone(), &v_k.clone())?; Py::Bool({ let t769 = py::getitem(&v_ps.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::lt(&t768, &t769)? { true } else { false } }) }, Py::str("unit prices must fall as the break quantity rises"))?;
    }
    v_best = Py::None;
    for t770 in py::range_iter(&[(py::len(&v_qs.clone())?).i()?]) {
        v_k = t770;
        v_H = py::mul(&v_i.clone(), &py::getitem(&v_ps.clone(), &v_k.clone())?)?;
        v_q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?)?;
        if ({ let t775 = { let t771 = py::add(&v_k.clone(), &Py::Int(1i64))?; Py::Bool({ let t772 = py::len(&v_qs.clone())?; if py::lt(&t771, &t772)? { true } else { false } }) }; if !t775.truthy() { t775 } else { { let t773 = v_q.clone(); Py::Bool({ let t774 = py::getitem(&v_qs.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::ge(&t773, &t774)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        if ({ let t776 = v_q.clone(); Py::Bool({ let t777 = py::getitem(&v_qs.clone(), &v_k.clone())?; if py::lt(&t776, &t777)? { true } else { false } }) }).truthy() {
            v_q = py::getitem(&v_qs.clone(), &v_k.clone())?;
        }
        v_tc = py::add(&py::add(&py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_q.clone())?, &py::truediv(&py::mul(&v_H.clone(), &v_q.clone())?, &Py::Int(2i64))?)?, &py::mul(&v_D.clone(), &py::getitem(&v_ps.clone(), &v_k.clone())?)?)?;
        if ({ let t782 = { let t778 = v_best.clone(); Py::Bool({ let t779 = Py::None; if py::is(&t778, &t779) { true } else { false } }) }; if t782.truthy() { t782 } else { { let t780 = v_tc.clone(); Py::Bool({ let t781 = py::sub(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &py::mul(&Py::Float(1e-12), &v_tc.clone())?)?; if py::lt(&t780, &t781)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_tc.clone(), v_q.clone(), py::getitem(&v_ps.clone(), &v_k.clone())?, py::add(&v_k.clone(), &Py::Int(1i64))?]);
        }
    }
    let t783 = (v_best.clone()).items()?;
    if t783.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_tc = t783[0].clone();
    v_q = t783[1].clone();
    v_p = t783[2].clone();
    v_tier = t783[3].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_q.clone()), (Py::str("unit_price"), v_p.clone()), (Py::str("total_cost"), v_tc.clone()), (Py::str("tier"), v_tier.clone()), (Py::str("ordering_cost"), py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_q.clone())?), (Py::str("holding_cost_per_year"), py::truediv(&py::mul(&py::mul(&v_i.clone(), &v_p.clone())?, &v_q.clone())?, &Py::Int(2i64))?), (Py::str("purchase_cost"), py::mul(&v_D.clone(), &v_p.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epq(mut v_demand: Py, mut v_production_rate: Py, mut v_order_cost: Py, mut v_holding_cost: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_f: Py = Py::None;
    let t823 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("production_rate"), v_production_rate.clone())?])).items()?;
    if t823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_D = t823[0].clone();
    v_P = t823[1].clone();
    let t824 = (Py::tuple(vec![f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t824[0].clone();
    v_H = t824[1].clone();
    let _ = f__need({ let t825 = v_P.clone(); Py::Bool({ let t826 = v_D.clone(); if py::gt(&t825, &t826)? { true } else { false } }) }, Py::str("the production rate must exceed the demand rate"))?;
    v_f = py::sub(&Py::Int(1i64), &py::truediv(&v_D.clone(), &v_P.clone())?)?;
    v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &py::mul(&v_H.clone(), &v_f.clone())?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("max_inventory"), py::mul(&v_Q.clone(), &v_f.clone())?), (Py::str("production_run_time"), py::truediv(&v_Q.clone(), &v_P.clone())?), (Py::str("cycle_time"), py::truediv(&v_Q.clone(), &v_D.clone())?), (Py::str("runs_per_year"), py::truediv(&v_D.clone(), &v_Q.clone())?), (Py::str("total_cost"), py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?, &v_f.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ewma_limits(mut v_target: Py, mut v_sigma: Py, mut v_smoothing: Py, mut v_limit_sigma: Py, mut v_sample_number: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wi: Py = Py::None;
    let t918 = (Py::tuple(vec![f__fin(Py::str("target"), v_target.clone())?, f__pos(Py::str("sigma"), v_sigma.clone())?])).items()?;
    if t918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t918[0].clone();
    v_s = t918[1].clone();
    let t922 = (Py::tuple(vec![f__prob(Py::str("smoothing"), v_smoothing.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?, f__pos(Py::str("limit_sigma"), v_limit_sigma.clone())?])).items()?;
    if t922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t922[0].clone();
    v_L = t922[1].clone();
    v_i = f__int(Py::str("sample_number"), v_sample_number.clone(), Py::Int(1i64), Py::Int(100000i64))?;
    v_f = py::truediv(&v_lam.clone(), &py::sub(&Py::Int(2i64), &v_lam.clone())?)?;
    v_w = py::mul(&py::mul(&v_L.clone(), &v_s.clone())?, &py::math1("sqrt", &py::mul(&v_f.clone(), &py::sub(&Py::Int(1i64), &py::pow(&py::sub(&Py::Int(1i64), &v_lam.clone())?, &py::mul(&Py::Int(2i64), &v_i.clone())?)?)?)?)?)?;
    v_wi = py::mul(&py::mul(&v_L.clone(), &v_s.clone())?, &py::math1("sqrt", &v_f.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("ucl"), py::add(&v_m.clone(), &v_w.clone())?), (Py::str("lcl"), py::sub(&v_m.clone(), &v_w.clone())?), (Py::str("ucl_steady"), py::add(&v_m.clone(), &v_wi.clone())?), (Py::str("lcl_steady"), py::sub(&v_m.clone(), &v_wi.clone())?), (Py::str("sd_ewma"), py::truediv(&v_w.clone(), &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exp_smoothing(mut v_data: Py, mut v_alpha: Py, mut v_initial_forecast: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("data"), v_data.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    v_a = f__prob(Py::str("alpha"), v_alpha.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?;
    v_f = (if ({ let t1033 = v_initial_forecast.clone(); Py::Bool({ let t1034 = Py::None; if py::is(&t1033, &t1034) { true } else { false } }) }).truthy() { py::getitem(&v_x.clone(), &Py::Int(0i64))? } else { f__fin(Py::str("initial_forecast"), v_initial_forecast.clone())? });
    v_fc = Py::list(vec![v_f.clone()]);
    for t1035 in py::iter(&(v_x.clone()))? {
        v_v = t1035;
        v_f = py::add(&v_f.clone(), &py::mul(&v_a.clone(), &py::sub(&v_v.clone(), &v_f.clone())?)?)?;
        let _ = py::method(&v_fc.clone(), "append", &[v_f.clone()])?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("forecast"), py::getitem(&v_fc.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("forecasts"), v_fc.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_reliability(mut v_failure_rate: Py, mut v_time: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t976 = (Py::tuple(vec![f__pos(Py::str("failure_rate"), v_failure_rate.clone())?, f__nonneg(Py::str("time"), v_time.clone())?])).items()?;
    if t976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t976[0].clone();
    v_t = t976[1].clone();
    v_R = py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), v_R.clone()), (Py::str("unreliability"), py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?)?), (Py::str("mtbf"), py::truediv(&Py::Int(1i64), &v_lam.clone())?), (Py::str("hazard_rate"), v_lam.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_factor_rating(mut v_weights: Py, mut v_scores: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = f__vec(Py::str("weights"), v_weights.clone(), Py::Int(1i64), Py::Int(100i64), Py::Bool(false), Py::Bool(true))?;
    v_s = f__mat(Py::str("scores"), v_scores.clone(), Py::Int(1i64), py::len(&v_w.clone())?, Py::Int(1i64), Py::None)?;
    v_tot = { let mut out = vec![]; for t1110 in py::iter(&(v_s.clone()))? { let mut v_row: Py; 
v_row = t1110;
out.push(py::sum(&{ let mut out = vec![]; for t1111 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1111;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&v_row.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_best = Py::Int(0i64);
    for t1112 in py::range_iter(&[(py::len(&v_tot.clone())?).i()?]) {
        v_i = t1112;
        if ({ let t1113 = py::getitem(&v_tot.clone(), &v_i.clone())?; Py::Bool({ let t1114 = py::add(&py::getitem(&v_tot.clone(), &v_best.clone())?, &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_tot.clone(), &v_best.clone())?)?], true)?)?)?; if py::gt(&t1113, &t1114)? { true } else { false } }) }).truthy() {
            v_best = v_i.clone();
        }
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("weighted_scores"), v_tot.clone()), (Py::str("best_site"), py::add(&v_best.clone(), &Py::Int(1i64))?), (Py::str("best_score"), py::getitem(&v_tot.clone(), &v_best.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flow_shop_makespan(mut v_times: Py) -> R {
    let mut v_cur: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_ready: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_m = f__mat(Py::str("times"), v_times.clone(), Py::Int(1i64), Py::None, Py::Int(1i64), Py::None)?;
    for t1173 in py::iter(&(v_m.clone()))? {
        v_row = t1173;
        for t1174 in py::iter(&(v_row.clone()))? {
            v_v = t1174;
            let _ = f__need({ let t1175 = v_v.clone(); Py::Bool({ let t1176 = Py::Int(0i64); if py::ge(&t1175, &t1176)? { true } else { false } }) }, Py::str("processing times must not be negative"))?;
        }
    }
    let t1177 = (Py::tuple(vec![py::len(&v_m.clone())?, py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?])).items()?;
    if t1177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1177[0].clone();
    v_k = t1177[1].clone();
    v_prev = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?;
    v_last = Py::list(vec![]);
    for t1178 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1178;
        v_cur = Py::list(vec![]);
        for t1179 in py::range_iter(&[(v_k.clone()).i()?]) {
            v_j = t1179;
            v_ready = py::minmax(&[py::getitem(&v_prev.clone(), &v_j.clone())?, (if ({ let t1180 = v_j.clone(); Py::Bool({ let t1181 = Py::Int(0i64); if py::gt(&t1180, &t1181)? { true } else { false } }) }).truthy() { py::getitem(&v_cur.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)? } else { Py::Float(0.0) })], true)?;
            let _ = py::method(&v_cur.clone(), "append", &[py::add(&v_ready.clone(), &py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_j.clone())?)?])?;
        }
        let _ = py::method(&v_last.clone(), "append", &[py::getitem(&v_cur.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?])?;
        v_prev = v_cur.clone();
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("makespan"), py::getitem(&v_last.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("completion_times"), v_last.clone()), (Py::str("idle_time_last_machine"), py::sub(&py::getitem(&v_last.clone(), &py::neg(&Py::Int(1i64))?)?, &py::sum(&{ let mut out = vec![]; for t1194 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1194;
out.push(py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &py::sub(&v_k.clone(), &Py::Int(1i64))?)?); } Py::list(out) }, None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forecast_errors(mut v_actual: Py, mut v_forecast: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_mad: Py = Py::None;
    let mut v_mse: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1238 = (f__pair(v_actual.clone(), v_forecast.clone())?).items()?;
    if t1238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1238[0].clone();
    v_f = t1238[1].clone();
    v_n = py::len(&v_a.clone())?;
    v_e = { let mut out = vec![]; for t1239 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1239;
out.push(py::sub(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::getitem(&v_f.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_mad = py::truediv(&py::sum(&{ let mut out = vec![]; for t1240 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t1240;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_mse = py::truediv(&py::sum(&{ let mut out = vec![]; for t1241 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t1241;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_out = Py::dict(vec![(Py::str("mad"), v_mad.clone()), (Py::str("mse"), v_mse.clone()), (Py::str("rmse"), py::math1("sqrt", &v_mse.clone())?), (Py::str("bias"), py::truediv(&py::sum(&v_e.clone(), None)?, &v_n.clone())?), (Py::str("running_sum_of_errors"), py::sum(&v_e.clone(), None)?)]);
    if (Py::Bool(({ let mut out = vec![]; for t1242 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1242;
out.push({ let t1243 = v_v.clone(); Py::Bool({ let t1244 = Py::Int(0i64); if py::ne(&t1243, &t1244) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        py::setitem(&v_out.clone(), &Py::str("mape"), py::truediv(&py::mul(&Py::Int(100i64), &py::sum(&{ let mut out = vec![]; for t1245 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1245;
out.push(py::abs(&py::truediv(&py::getitem(&v_e.clone(), &v_i.clone())?, &py::getitem(&v_a.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None)?)?, &v_n.clone())?)?;
    }
    if ({ let t1246 = v_mad.clone(); Py::Bool({ let t1247 = Py::Int(0i64); if py::gt(&t1246, &t1247)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("tracking_signal"), py::truediv(&py::sum(&v_e.clone(), None)?, &v_mad.clone())?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fraction_nonconforming(mut v_mean: Py, mut v_sd: Py, mut v_lsl: Py, mut v_usl: Py) -> R {
    let mut v_above: Py = Py::None;
    let mut v_below: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let t1287 = (Py::tuple(vec![f__fin(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?])).items()?;
    if t1287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1287[0].clone();
    v_s = t1287[1].clone();
    let _ = f__need({ let t1292 = { let t1288 = v_lsl.clone(); Py::Bool({ let t1289 = Py::None; if !py::is(&t1288, &t1289) { true } else { false } }) }; if t1292.truthy() { t1292 } else { { let t1290 = v_usl.clone(); Py::Bool({ let t1291 = Py::None; if !py::is(&t1290, &t1291) { true } else { false } }) } } }, Py::str("give at least one specification limit"))?;
    v_below = (if ({ let t1293 = v_lsl.clone(); Py::Bool({ let t1294 = Py::None; if !py::is(&t1293, &t1294) { true } else { false } }) }).truthy() { f__cdf(py::truediv(&py::sub(&f__fin(Py::str("lsl"), v_lsl.clone())?, &v_m.clone())?, &v_s.clone())?)? } else { Py::Float(0.0) });
    v_above = (if ({ let t1295 = v_usl.clone(); Py::Bool({ let t1296 = Py::None; if !py::is(&t1295, &t1296) { true } else { false } }) }).truthy() { f__sf(py::truediv(&py::sub(&f__fin(Py::str("usl"), v_usl.clone())?, &v_m.clone())?, &v_s.clone())?)? } else { Py::Float(0.0) });
    if ({ let t1301 = { let t1297 = v_lsl.clone(); Py::Bool({ let t1298 = Py::None; if !py::is(&t1297, &t1298) { true } else { false } }) }; if !t1301.truthy() { t1301 } else { { let t1299 = v_usl.clone(); Py::Bool({ let t1300 = Py::None; if !py::is(&t1299, &t1300) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t1302 = v_usl.clone(); Py::Bool({ let t1303 = v_lsl.clone(); if py::gt(&t1302, &t1303)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    }
    v_tot = py::add(&v_below.clone(), &v_above.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("fraction_out"), v_tot.clone()), (Py::str("below_lsl"), v_below.clone()), (Py::str("above_usl"), v_above.clone()), (Py::str("ppm"), py::mul(&v_tot.clone(), &Py::Float(1000000.0))?), (Py::str("yield"), py::sub(&Py::Int(1i64), &v_tot.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_holt(mut v_data: Py, mut v_alpha: Py, mut v_beta: Py, mut v_initial_level: Py, mut v_initial_trend: Py, mut v_horizon: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_lv: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("data"), v_data.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    let t1360 = (Py::tuple(vec![f__prob(Py::str("alpha"), v_alpha.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?, f__prob(Py::str("beta"), v_beta.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t1360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1360[0].clone();
    v_b = t1360[1].clone();
    let t1361 = (Py::tuple(vec![f__fin(Py::str("initial_level"), v_initial_level.clone())?, f__fin(Py::str("initial_trend"), v_initial_trend.clone())?])).items()?;
    if t1361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lv = t1361[0].clone();
    v_tr = t1361[1].clone();
    v_h = f__int(Py::str("horizon"), v_horizon.clone(), Py::Int(1i64), Py::Int(1000i64))?;
    for t1374 in py::iter(&(v_x.clone()))? {
        v_v = t1374;
        v_prev = v_lv.clone();
        v_lv = py::add(&py::mul(&v_a.clone(), &v_v.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_a.clone())?, &py::add(&v_lv.clone(), &v_tr.clone())?)?)?;
        v_tr = py::add(&py::mul(&v_b.clone(), &py::sub(&v_lv.clone(), &v_prev.clone())?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_b.clone())?, &v_tr.clone())?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("forecast"), py::add(&v_lv.clone(), &py::mul(&v_h.clone(), &v_tr.clone())?)?), (Py::str("level"), v_lv.clone()), (Py::str("trend"), v_tr.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inventory_total_cost(mut v_demand: Py, mut v_order_cost: Py, mut v_holding_cost: Py, mut v_order_quantity: Py, mut v_unit_cost: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_hc: Py = Py::None;
    let mut v_oc: Py = Py::None;
    let t1414 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t1414.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1414[0].clone();
    v_S = t1414[1].clone();
    v_H = t1414[2].clone();
    let t1417 = (Py::tuple(vec![f__pos(Py::str("order_quantity"), v_order_quantity.clone())?, f__nonneg(Py::str("unit_cost"), v_unit_cost.clone())?])).items()?;
    if t1417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t1417[0].clone();
    v_c = t1417[1].clone();
    let t1418 = (Py::tuple(vec![py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Q.clone())?, py::truediv(&py::mul(&v_H.clone(), &v_Q.clone())?, &Py::Int(2i64))?])).items()?;
    if t1418.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_oc = t1418[0].clone();
    v_hc = t1418[1].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("total_cost"), py::add(&py::add(&v_oc.clone(), &v_hc.clone())?, &py::mul(&v_D.clone(), &v_c.clone())?)?), (Py::str("ordering_cost"), v_oc.clone()), (Py::str("holding_cost_per_year"), v_hc.clone()), (Py::str("purchase_cost"), py::mul(&v_D.clone(), &v_c.clone())?), (Py::str("average_inventory"), py::truediv(&v_Q.clone(), &Py::Int(2i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inventory_turnover(mut v_cost_of_goods_sold: Py, mut v_average_inventory: Py, mut v_days_per_year: Py) -> R {
    let mut v_cogs: Py = Py::None;
    let mut v_dpy: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1458 = (Py::tuple(vec![f__pos(Py::str("cost_of_goods_sold"), v_cost_of_goods_sold.clone())?, f__pos(Py::str("average_inventory"), v_average_inventory.clone())?])).items()?;
    if t1458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cogs = t1458[0].clone();
    v_inv = t1458[1].clone();
    v_dpy = f__pos(Py::str("days_per_year"), v_days_per_year.clone())?;
    v_t = py::truediv(&v_cogs.clone(), &v_inv.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("turnover"), v_t.clone()), (Py::str("days_of_supply"), py::truediv(&v_dpy.clone(), &v_t.clone())?), (Py::str("weeks_of_supply"), py::truediv(&py::mul(&Py::Int(52i64), &v_inv.clone())?, &v_cogs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_johnson_three(mut v_machine1_times: Py, mut v_machine2_times: Py, mut v_machine3_times: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_order: Py = Py::None;
    v_a = f__vec(Py::str("machine1_times"), v_machine1_times.clone(), Py::Int(1i64), Py::Int(500i64), Py::Bool(false), Py::Bool(true))?;
    v_b = f__vec(Py::str("machine2_times"), v_machine2_times.clone(), Py::Int(1i64), Py::Int(500i64), Py::Bool(false), Py::Bool(true))?;
    v_c = f__vec(Py::str("machine3_times"), v_machine3_times.clone(), Py::Int(1i64), Py::Int(500i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t1512 = py::len(&v_a.clone())?; Py::Bool({ let t1513 = py::len(&v_b.clone())?; if py::eq(&t1512, &t1513) { { let t1514 = py::len(&v_c.clone())?; if py::eq(&t1513, &t1514) { true } else { false } } } else { false } }) }, Py::str("every machine needs a time for every job"))?;
    let _ = f__need({ let t1519 = { let t1515 = py::minmax(&[v_a.clone()], false)?; Py::Bool({ let t1516 = py::minmax(&[v_b.clone()], true)?; if py::ge(&t1515, &t1516)? { true } else { false } }) }; if t1519.truthy() { t1519 } else { { let t1517 = py::minmax(&[v_c.clone()], false)?; Py::Bool({ let t1518 = py::minmax(&[v_b.clone()], true)?; if py::ge(&t1517, &t1518)? { true } else { false } }) } } }, Py::str("Johnson's extension needs min(machine 1) >= max(machine 2) or min(machine 3) >= max(machine 2)"))?;
    v_order = f__johnson_order({ let mut out = vec![]; for t1526 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1526;
out.push(py::add(&py::getitem(&v_a.clone(), &v_j.clone())?, &py::getitem(&v_b.clone(), &v_j.clone())?)?); } Py::list(out) }, { let mut out = vec![]; for t1527 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1527;
out.push(py::add(&py::getitem(&v_b.clone(), &v_j.clone())?, &py::getitem(&v_c.clone(), &v_j.clone())?)?); } Py::list(out) })?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t1540 in py::iter(&(v_order.clone()))? { let mut v_j: Py; 
v_j = t1540;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("makespan"), f__three_machine_span(v_order.clone(), v_a.clone(), v_b.clone(), v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_johnson_two(mut v_machine1_times: Py, mut v_machine2_times: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_span: Py = Py::None;
    v_a = f__vec(Py::str("machine1_times"), v_machine1_times.clone(), Py::Int(1i64), Py::Int(500i64), Py::Bool(false), Py::Bool(true))?;
    v_b = f__vec(Py::str("machine2_times"), v_machine2_times.clone(), Py::Int(1i64), Py::Int(500i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t1584 = py::len(&v_a.clone())?; Py::Bool({ let t1585 = py::len(&v_b.clone())?; if py::eq(&t1584, &t1585) { true } else { false } }) }, Py::str("both machines need a time for every job"))?;
    v_order = f__johnson_order(v_a.clone(), v_b.clone())?;
    v_span = f__two_machine_span(v_order.clone(), v_a.clone(), v_b.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t1606 in py::iter(&(v_order.clone()))? { let mut v_j: Py; 
v_j = t1606;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("makespan"), v_span.clone()), (Py::str("idle_time_machine2"), py::sub(&v_span.clone(), &py::sum(&v_b.clone(), None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_k_out_of_n(mut v_component_reliability: Py, mut v_k: Py, mut v_n: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_kk: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_r = f__fin(Py::str("component_reliability"), v_component_reliability.clone())?;
    let _ = f__need({ let t1632 = Py::Int(0i64); Py::Bool({ let t1633 = v_r.clone(); if py::le(&t1632, &t1633)? { { let t1634 = Py::Int(1i64); if py::le(&t1633, &t1634)? { true } else { false } } } else { false } }) }, Py::str("component_reliability must lie between 0 and 1"))?;
    let t1647 = (Py::tuple(vec![f__int(Py::str("n"), v_n.clone(), Py::Int(1i64), Py::Int(1000i64))?, f__int(Py::str("k"), v_k.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t1647.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nn = t1647[0].clone();
    v_kk = t1647[1].clone();
    let _ = f__need({ let t1648 = v_kk.clone(); Py::Bool({ let t1649 = v_nn.clone(); if py::le(&t1648, &t1649)? { true } else { false } }) }, Py::str("k must not exceed n"))?;
    v_s = Py::Float(0.0);
    for t1650 in py::range_iter(&[(v_kk.clone()).i()?, (py::add(&v_nn.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t1650;
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::comb(&v_nn.clone(), &v_j.clone())?, &py::pow(&v_r.clone(), &v_j.clone())?)?, &py::pow(&py::sub(&Py::Int(1i64), &v_r.clone())?, &py::sub(&v_nn.clone(), &v_j.clone())?)?)?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), py::minmax(&[Py::Float(1.0), v_s.clone()], false)?), (Py::str("unreliability"), py::minmax(&[Py::Float(0.0), py::sub(&Py::Int(1i64), &v_s.clone())?], true)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kanban_count(mut v_demand_rate: Py, mut v_lead_time: Py, mut v_container_size: Py, mut v_safety_factor: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1690 = (Py::tuple(vec![f__pos(Py::str("demand_rate"), v_demand_rate.clone())?, f__pos(Py::str("lead_time"), v_lead_time.clone())?])).items()?;
    if t1690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1690[0].clone();
    v_L = t1690[1].clone();
    let t1693 = (Py::tuple(vec![f__pos(Py::str("container_size"), v_container_size.clone())?, f__nonneg(Py::str("safety_factor"), v_safety_factor.clone())?])).items()?;
    if t1693.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1693[0].clone();
    v_a = t1693[1].clone();
    v_x = py::truediv(&py::mul(&py::mul(&v_d.clone(), &v_L.clone())?, &py::add(&Py::Int(1i64), &v_a.clone())?)?, &v_c.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("kanbans"), f__ceil(v_x.clone())?), (Py::str("exact"), v_x.clone()), (Py::str("max_inventory"), py::mul(&f__ceil(v_x.clone())?, &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_largest_candidate(mut v_task_times: Py, mut v_precedence: Py, mut v_cycle_time: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_pr: Py = Py::None;
    let mut v_preds: Py = Py::None;
    let mut v_rank: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f__vec(Py::str("task_times"), v_task_times.clone(), Py::Int(2i64), Py::Int(300i64), Py::Bool(true), Py::Bool(false))?;
    v_c = f__pos(Py::str("cycle_time"), v_cycle_time.clone())?;
    let _ = f__need({ let t1751 = py::minmax(&[v_t.clone()], true)?; Py::Bool({ let t1752 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t1751, &t1752)? { true } else { false } }) }, Py::str("every task must fit within the cycle time"))?;
    v_preds = f__precedence(py::len(&v_t.clone())?, v_precedence.clone())?;
    v_n = py::len(&v_t.clone())?;
    v_pr = { let mut out = vec![]; for t1801 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1801;
out.push(py::sub(&py::getitem(&v_t.clone(), &v_j.clone())?, &py::mul(&py::mul(&v_j.clone(), &Py::Float(1e-09))?, &Py::Int(0i64))?)?); } Py::list(out) };
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_c = v_c.clone(); let v_cycle_time = v_cycle_time.clone(); let v_n = v_n.clone(); let v_pr = v_pr.clone(); let v_precedence = v_precedence.clone(); let v_preds = v_preds.clone(); let v_t = v_t.clone(); let v_task_times = v_task_times.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_t.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_rank = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
    for t1802 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_pos = t1802;
        py::setitem(&v_rank.clone(), &py::getitem(&v_order.clone(), &v_pos.clone())?, py::sub(&v_n.clone(), &v_pos.clone())?)?;
    }
    return Ok(f__rd(f__balance(v_t.clone(), v_preds.clone(), v_c.clone(), v_rank.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_learning_cumulative(mut v_first_unit_time: Py, mut v_learning_rate: Py, mut v_units: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_T1: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let t1989 = (Py::tuple(vec![f__pos(Py::str("first_unit_time"), v_first_unit_time.clone())?, f__prob(Py::str("learning_rate"), v_learning_rate.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t1989.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t1989[0].clone();
    v_r = t1989[1].clone();
    v_N = f__int(Py::str("units"), v_units.clone(), Py::Int(1i64), Py::Int(200000i64))?;
    v_b = f__b(v_r.clone())?;
    v_tot = Py::Float(0.0);
    for t2002 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t2002;
        v_tot = py::iadd(&v_tot, &py::pow(&v_i.clone(), &v_b.clone())?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("total_time"), py::mul(&v_T1.clone(), &v_tot.clone())?), (Py::str("average_time"), py::truediv(&py::mul(&v_T1.clone(), &v_tot.clone())?, &v_N.clone())?), (Py::str("last_unit_time"), py::mul(&v_T1.clone(), &py::pow(&v_N.clone(), &v_b.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_learning_fit(mut v_units: Py, mut v_times: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n_: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    v_n_ = f__vec(Py::str("units"), v_units.clone(), Py::Int(2i64), Py::Int(10000i64), Py::Bool(true), Py::Bool(false))?;
    v_t = f__vec(Py::str("times"), v_times.clone(), Py::Int(2i64), Py::Int(10000i64), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t1933 = py::len(&v_n_.clone())?; Py::Bool({ let t1934 = py::len(&v_t.clone())?; if py::eq(&t1933, &t1934) { true } else { false } }) }, Py::str("units and times must have the same number of values"))?;
    let t1937 = (Py::tuple(vec![{ let mut out = vec![]; for t1935 in py::iter(&(v_n_.clone()))? { let mut v_v: Py; 
v_v = t1935;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1936 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t1936;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }])).items()?;
    if t1937.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t1937[0].clone();
    v_ys = t1937[1].clone();
    v_k = py::len(&v_xs.clone())?;
    let t1938 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_k.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_k.clone())?])).items()?;
    if t1938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t1938[0].clone();
    v_my = t1938[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t1939 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1939;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t1940 = v_sxx.clone(); Py::Bool({ let t1941 = Py::Int(0i64); if py::gt(&t1940, &t1941)? { true } else { false } }) }, Py::str("the unit numbers must not all be equal"))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t1942 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1943 = (t1942).items()?;
if t1943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1943[0].clone();
v_y = t1943[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t1944 in py::iter(&(v_ys.clone()))? { let mut v_y: Py; 
v_y = t1944;
out.push(py::pow(&py::sub(&v_y.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("learning_rate"), py::pow(&Py::Float(2.0), &v_b.clone())?), (Py::str("exponent"), v_b.clone()), (Py::str("first_unit_time"), py::math1("exp", &v_a.clone())?), (Py::str("r_squared"), (if ({ let t1957 = v_syy.clone(); Py::Bool({ let t1958 = Py::Int(0i64); if py::gt(&t1957, &t1958)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)? } else { Py::Float(1.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_learning_two_points(mut v_unit1: Py, mut v_time1: Py, mut v_unit2: Py, mut v_time2: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t2097 = (Py::tuple(vec![f__pos(Py::str("unit1"), v_unit1.clone())?, f__pos(Py::str("unit2"), v_unit2.clone())?])).items()?;
    if t2097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t2097[0].clone();
    v_n2 = t2097[1].clone();
    let t2098 = (Py::tuple(vec![f__pos(Py::str("time1"), v_time1.clone())?, f__pos(Py::str("time2"), v_time2.clone())?])).items()?;
    if t2098.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t2098[0].clone();
    v_t2 = t2098[1].clone();
    let _ = f__need({ let t2099 = v_n1.clone(); Py::Bool({ let t2100 = v_n2.clone(); if py::ne(&t2099, &t2100) { true } else { false } }) }, Py::str("the two unit numbers must differ"))?;
    v_b = py::truediv(&py::math1("log", &py::truediv(&v_t2.clone(), &v_t1.clone())?)?, &py::math1("log", &py::truediv(&v_n2.clone(), &v_n1.clone())?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("learning_rate"), py::pow(&Py::Float(2.0), &v_b.clone())?), (Py::str("exponent"), v_b.clone()), (Py::str("first_unit_time"), py::truediv(&v_t1.clone(), &py::pow(&v_n1.clone(), &v_b.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_learning_unit(mut v_first_unit_time: Py, mut v_learning_rate: Py, mut v_unit: Py) -> R {
    let mut v_T1: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2045 = (Py::tuple(vec![f__pos(Py::str("first_unit_time"), v_first_unit_time.clone())?, f__prob(Py::str("learning_rate"), v_learning_rate.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t2045.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t2045[0].clone();
    v_r = t2045[1].clone();
    v_n = f__int(Py::str("unit"), v_unit.clone(), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    v_b = f__b(v_r.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("unit_time"), py::mul(&v_T1.clone(), &py::pow(&v_n.clone(), &v_b.clone())?)?), (Py::str("exponent"), v_b.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_efficiency(mut v_task_times: Py, mut v_cycle_time: Py, mut v_stations: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_eff: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_t = f__vec(Py::str("task_times"), v_task_times.clone(), Py::Int(1i64), Py::Int(1000i64), Py::Bool(true), Py::Bool(false))?;
    let t2168 = (Py::tuple(vec![f__pos(Py::str("cycle_time"), v_cycle_time.clone())?, f__int(Py::str("stations"), v_stations.clone(), Py::Int(1i64), Py::Int(10000i64))?])).items()?;
    if t2168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2168[0].clone();
    v_n = t2168[1].clone();
    v_w = py::sum(&v_t.clone(), None)?;
    let _ = f__need({ let t2169 = py::minmax(&[v_t.clone()], true)?; Py::Bool({ let t2170 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t2169, &t2170)? { true } else { false } }) }, Py::str("every task must fit within the cycle time"))?;
    let _ = f__need({ let t2171 = v_w.clone(); Py::Bool({ let t2172 = py::mul(&py::mul(&v_n.clone(), &v_c.clone())?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t2171, &t2172)? { true } else { false } }) }, Py::str("the stations cannot hold the total work at this cycle time"))?;
    v_eff = py::truediv(&v_w.clone(), &py::mul(&v_n.clone(), &v_c.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("efficiency"), v_eff.clone()), (Py::str("balance_delay"), py::sub(&Py::Int(1i64), &v_eff.clone())?), (Py::str("idle_time"), py::sub(&py::mul(&v_n.clone(), &v_c.clone())?, &v_w.clone())?), (Py::str("total_task_time"), v_w.clone()), (Py::str("minimum_stations"), f__ceil(py::truediv(&v_w.clone(), &v_c.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_trend(mut v_data: Py, mut v_horizon: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_y = f__vec(Py::str("data"), v_data.clone(), Py::Int(2i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    v_h = f__int(Py::str("horizon"), v_horizon.clone(), Py::Int(1i64), Py::Int(10000i64))?;
    v_n = py::len(&v_y.clone())?;
    v_xs = Py::list((Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    let t2240 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?])).items()?;
    if t2240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2240[0].clone();
    v_my = t2240[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2241 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2241;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_b = py::truediv(&py::sum(&{ let mut out = vec![]; for t2242 in py::iter(&(py::zip(&[v_xs.clone(), v_y.clone()])?))? { let mut v_v: Py; let mut v_x: Py; 
let t2243 = (t2242).items()?;
if t2243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2243[0].clone();
v_v = t2243[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_v.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    if ({ let t2244 = py::abs(&v_b.clone())?; Py::Bool({ let t2245 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t2246 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t2246;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::le(&t2244, &t2245)? { true } else { false } }) }).truthy() {
        v_b = Py::Float(0.0);
    }
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("forecast"), py::add(&v_a.clone(), &py::mul(&v_b.clone(), &py::add(&v_n.clone(), &v_h.clone())?)?)?), (Py::str("intercept"), v_a.clone()), (Py::str("slope"), v_b.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_littles_law(mut v_arrival_rate: Py, mut v_number_in_system: Py, mut v_time_in_system: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_lam: Py = Py::None;
    v_given = { let mut out = vec![]; for t2282 in py::iter(&(Py::tuple(vec![v_arrival_rate.clone(), v_number_in_system.clone(), v_time_in_system.clone()])))? { let mut v_v: Py; 
v_v = t2282;
out.push({ let t2283 = v_v.clone(); Py::Bool({ let t2284 = Py::None; if !py::is(&t2283, &t2284) { true } else { false } }) }); } Py::list(out) };
    let _ = f__need({ let t2285 = py::sum(&v_given.clone(), None)?; Py::Bool({ let t2286 = Py::Int(2i64); if py::eq(&t2285, &t2286) { true } else { false } }) }, Py::str("give exactly two of arrival rate, number in system and time in system"))?;
    if ({ let t2287 = v_arrival_rate.clone(); Py::Bool({ let t2288 = Py::None; if py::is(&t2287, &t2288) { true } else { false } }) }).truthy() {
        let t2295 = (Py::tuple(vec![f__nonneg(Py::str("number_in_system"), v_number_in_system.clone())?, f__pos(Py::str("time_in_system"), v_time_in_system.clone())?])).items()?;
        if t2295.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_L = t2295[0].clone();
        v_W = t2295[1].clone();
        v_lam = py::truediv(&v_L.clone(), &v_W.clone())?;
    } else {
        if ({ let t2296 = v_number_in_system.clone(); Py::Bool({ let t2297 = Py::None; if py::is(&t2296, &t2297) { true } else { false } }) }).truthy() {
            let t2298 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__nonneg(Py::str("time_in_system"), v_time_in_system.clone())?])).items()?;
            if t2298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lam = t2298[0].clone();
            v_W = t2298[1].clone();
            v_L = py::mul(&v_lam.clone(), &v_W.clone())?;
        } else {
            let t2299 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__nonneg(Py::str("number_in_system"), v_number_in_system.clone())?])).items()?;
            if t2299.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lam = t2299[0].clone();
            v_L = t2299[1].clone();
            v_W = py::truediv(&v_L.clone(), &v_lam.clone())?;
        }
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("arrival_rate"), v_lam.clone()), (Py::str("number_in_system"), v_L.clone()), (Py::str("time_in_system"), v_W.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_location_cost_volume(mut v_fixed_costs: Py, mut v_variable_costs: Py, mut v_volume: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_tot: Py = Py::None;
    v_F = f__vec(Py::str("fixed_costs"), v_fixed_costs.clone(), Py::Int(2i64), Py::Int(100i64), Py::Bool(false), Py::Bool(true))?;
    v_V = f__vec(Py::str("variable_costs"), v_variable_costs.clone(), Py::Int(2i64), Py::Int(100i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t2353 = py::len(&v_F.clone())?; Py::Bool({ let t2354 = py::len(&v_V.clone())?; if py::eq(&t2353, &t2354) { true } else { false } }) }, Py::str("fixed_costs and variable_costs must have the same number of values"))?;
    v_q = f__nonneg(Py::str("volume"), v_volume.clone())?;
    v_tot = { let mut out = vec![]; for t2357 in py::range_iter(&[(py::len(&v_F.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2357;
out.push(py::add(&py::getitem(&v_F.clone(), &v_i.clone())?, &py::mul(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_q.clone())?)?); } Py::list(out) };
    v_best = Py::Int(0i64);
    for t2358 in py::range_iter(&[(py::len(&v_tot.clone())?).i()?]) {
        v_i = t2358;
        if ({ let t2359 = py::getitem(&v_tot.clone(), &v_i.clone())?; Py::Bool({ let t2360 = py::sub(&py::getitem(&v_tot.clone(), &v_best.clone())?, &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_tot.clone(), &v_best.clone())?)?], true)?)?)?; if py::lt(&t2359, &t2360)? { true } else { false } }) }).truthy() {
            v_best = v_i.clone();
        }
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("total_costs"), v_tot.clone()), (Py::str("best_site"), py::add(&v_best.clone(), &Py::Int(1i64))?), (Py::str("best_cost"), py::getitem(&v_tot.clone(), &v_best.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_md1(mut v_arrival_rate: Py, mut v_service_rate: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let t2400 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t2400.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2400[0].clone();
    v_mu = t2400[1].clone();
    return Ok(f__rd(f__mg1_values(v_lam.clone(), py::truediv(&Py::Int(1i64), &v_mu.clone())?, Py::Float(0.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mg1(mut v_arrival_rate: Py, mut v_mean_service_time: Py, mut v_sd_service_time: Py) -> R {
    let mut v_es: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let t2446 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("mean_service_time"), v_mean_service_time.clone())?, f__nonneg(Py::str("sd_service_time"), v_sd_service_time.clone())?])).items()?;
    if t2446.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2446[0].clone();
    v_es = t2446[1].clone();
    v_sd = t2446[2].clone();
    return Ok(f__rd(f__mg1_values(v_lam.clone(), v_es.clone(), py::mul(&v_sd.clone(), &v_sd.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mm1(mut v_arrival_rate: Py, mut v_service_rate: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let t2490 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t2490.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2490[0].clone();
    v_mu = t2490[1].clone();
    let _ = f__need({ let t2491 = v_lam.clone(); Py::Bool({ let t2492 = v_mu.clone(); if py::lt(&t2491, &t2492)? { true } else { false } }) }, Py::str("the queue is stable only if the arrival rate is below the service rate"))?;
    v_rho = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("utilization"), v_rho.clone()), (Py::str("L"), py::truediv(&v_rho.clone(), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?), (Py::str("Lq"), py::truediv(&py::mul(&v_rho.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?), (Py::str("W"), py::truediv(&Py::Int(1i64), &py::sub(&v_mu.clone(), &v_lam.clone())?)?), (Py::str("Wq"), py::truediv(&v_rho.clone(), &py::sub(&v_mu.clone(), &v_lam.clone())?)?), (Py::str("p0"), py::sub(&Py::Int(1i64), &v_rho.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mm1k(mut v_arrival_rate: Py, mut v_service_rate: Py, mut v_capacity: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_Lq: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_le: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2544 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?, f__int(Py::str("capacity"), v_capacity.clone(), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t2544.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2544[0].clone();
    v_mu = t2544[1].clone();
    v_K = t2544[2].clone();
    v_rho = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    let t2545 = (Py::tuple(vec![Py::list(vec![]), Py::Float(1.0)])).items()?;
    if t2545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ps = t2545[0].clone();
    v_w = t2545[1].clone();
    for t2546 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t2546;
        let _ = py::method(&v_ps.clone(), "append", &[v_w.clone()])?;
        v_w = py::mul(&v_w, &v_rho.clone())?;
    }
    v_tot = py::sum(&v_ps.clone(), None)?;
    v_ps = { let mut out = vec![]; for t2547 in py::iter(&(v_ps.clone()))? { let mut v_p: Py; 
v_p = t2547;
out.push(py::truediv(&v_p.clone(), &v_tot.clone())?); } Py::list(out) };
    v_L = py::sum(&{ let mut out = vec![]; for t2548 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t2548;
out.push(py::mul(&v_n.clone(), &py::getitem(&v_ps.clone(), &v_n.clone())?)?); } Py::list(out) }, None)?;
    v_Lq = py::sum(&{ let mut out = vec![]; for t2549 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t2549;
out.push(py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::getitem(&v_ps.clone(), &v_n.clone())?)?); } Py::list(out) }, None)?;
    v_le = py::mul(&v_lam.clone(), &py::sub(&Py::Int(1i64), &py::getitem(&v_ps.clone(), &v_K.clone())?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("p0"), py::getitem(&v_ps.clone(), &Py::Int(0i64))?), (Py::str("p_block"), py::getitem(&v_ps.clone(), &v_K.clone())?), (Py::str("L"), v_L.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("effective_arrival_rate"), v_le.clone()), (Py::str("W"), py::truediv(&v_L.clone(), &v_le.clone())?), (Py::str("Wq"), py::truediv(&v_Lq.clone(), &v_le.clone())?), (Py::str("utilization"), py::sub(&Py::Int(1i64), &py::getitem(&v_ps.clone(), &Py::Int(0i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mmc(mut v_arrival_rate: Py, mut v_service_rate: Py, mut v_servers: Py) -> R {
    let mut v_Lq: Py = Py::None;
    let mut v_Wq: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_pw: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    let t2601 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?, f__int(Py::str("servers"), v_servers.clone(), Py::Int(1i64), Py::Int(500i64))?])).items()?;
    if t2601.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2601[0].clone();
    v_mu = t2601[1].clone();
    v_c = t2601[2].clone();
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_rho = py::truediv(&v_a.clone(), &v_c.clone())?;
    let _ = f__need({ let t2602 = v_rho.clone(); Py::Bool({ let t2603 = Py::Int(1i64); if py::lt(&t2602, &t2603)? { true } else { false } }) }, Py::str("the queue is stable only if the arrival rate is below servers x service rate"))?;
    let t2604 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t2604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t2604[0].clone();
    v_s = t2604[1].clone();
    for t2605 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_c.clone()).i()?]) {
        v_k = t2605;
        v_term = py::mul(&v_term, &py::truediv(&v_a.clone(), &v_k.clone())?)?;
        v_s = py::iadd(&v_s, &v_term.clone())?;
    }
    v_last = py::truediv(&py::truediv(&py::mul(&v_term.clone(), &v_a.clone())?, &v_c.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    v_p0 = py::truediv(&Py::Int(1i64), &py::add(&v_s.clone(), &v_last.clone())?)?;
    v_pw = py::mul(&v_last.clone(), &v_p0.clone())?;
    v_Lq = py::truediv(&py::mul(&v_pw.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    v_Wq = py::truediv(&v_Lq.clone(), &v_lam.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("utilization"), v_rho.clone()), (Py::str("p_wait"), v_pw.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("Wq"), v_Wq.clone()), (Py::str("W"), py::add(&v_Wq.clone(), &py::truediv(&Py::Int(1i64), &v_mu.clone())?)?), (Py::str("L"), py::add(&v_Lq.clone(), &v_a.clone())?), (Py::str("p0"), v_p0.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moore_hodgson(mut v_processing_times: Py, mut v_due_dates: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_late: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2663 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t2663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2663[0].clone();
    v_d = t2663[1].clone();
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_d.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    let t2664 = (Py::tuple(vec![Py::list(vec![]), Py::Float(0.0), Py::list(vec![])])).items()?;
    if t2664.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_on = t2664[0].clone();
    v_t = t2664[1].clone();
    v_late = t2664[2].clone();
    for t2665 in py::iter(&(v_order.clone()))? {
        v_j = t2665;
        let _ = py::method(&v_on.clone(), "append", &[v_j.clone()])?;
        v_t = py::iadd(&v_t, &py::getitem(&v_p.clone(), &v_j.clone())?)?;
        if ({ let t2666 = v_t.clone(); Py::Bool({ let t2667 = py::getitem(&v_d.clone(), &v_j.clone())?; if py::gt(&t2666, &t2667)? { true } else { false } }) }).truthy() {
            v_k = py::getitem(&v_on.clone(), &Py::Int(0i64))?;
            for t2668 in py::iter(&(v_on.clone()))? {
                v_x = t2668;
                if ({ let t2676 = { let t2669 = py::getitem(&v_p.clone(), &v_x.clone())?; Py::Bool({ let t2670 = py::getitem(&v_p.clone(), &v_k.clone())?; if py::gt(&t2669, &t2670)? { true } else { false } }) }; if t2676.truthy() { t2676 } else { { let t2675 = { let t2671 = py::getitem(&v_p.clone(), &v_x.clone())?; Py::Bool({ let t2672 = py::getitem(&v_p.clone(), &v_k.clone())?; if py::eq(&t2671, &t2672) { true } else { false } }) }; if !t2675.truthy() { t2675 } else { { let t2673 = v_x.clone(); Py::Bool({ let t2674 = v_k.clone(); if py::lt(&t2673, &t2674)? { true } else { false } }) } } } } }).truthy() {
                    v_k = v_x.clone();
                }
            }
            let _ = py::method(&v_on.clone(), "remove", &[v_k.clone()])?;
            v_t = py::sub(&v_t, &py::getitem(&v_p.clone(), &v_k.clone())?)?;
            let _ = py::method(&v_late.clone(), "append", &[v_k.clone()])?;
        }
    }
    v_seq = py::add(&{ let mut out = vec![]; for t2677 in py::iter(&(v_on.clone()))? { let mut v_j: Py; 
v_j = t2677;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, &py::sorted(&{ let mut out = vec![]; for t2678 in py::iter(&(v_late.clone()))? { let mut v_j: Py; 
v_j = t2678;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, (Py::Bool(false)).truthy())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("number_late"), py::len(&v_late.clone())?), (Py::str("late_jobs"), py::sorted(&{ let mut out = vec![]; for t2691 in py::iter(&(v_late.clone()))? { let mut v_j: Py; 
v_j = t2691;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, (Py::Bool(false)).truthy())?), (Py::str("on_time_jobs"), { let mut out = vec![]; for t2692 in py::iter(&(v_on.clone()))? { let mut v_j: Py; 
v_j = t2692;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("sequence"), v_seq.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moving_average(mut v_data: Py, mut v_periods: Py) -> R {
    let mut v_fc: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("data"), v_data.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    v_n = f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::None)?;
    let _ = f__need({ let t2746 = v_n.clone(); Py::Bool({ let t2747 = py::len(&v_x.clone())?; if py::le(&t2746, &t2747)? { true } else { false } }) }, Py::str("periods must not exceed the number of observations"))?;
    v_fc = Py::list(vec![]);
    for t2748 in py::range_iter(&[(v_n.clone()).i()?, (py::add(&py::len(&v_x.clone())?, &Py::Int(1i64))?).i()?]) {
        v_t = t2748;
        let _ = py::method(&v_fc.clone(), "append", &[py::truediv(&py::sum(&py::slice(&v_x.clone(), Some((py::sub(&v_t.clone(), &v_n.clone())?).i()?), Some((v_t.clone()).i()?), None)?, None)?, &v_n.clone())?])?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("forecast"), py::getitem(&v_fc.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("forecasts"), v_fc.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mtbf_confidence(mut v_total_time: Py, mut v_failures: Py, mut v_confidence: Py, mut v_test: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2800 = (Py::tuple(vec![f__pos(Py::str("total_time"), v_total_time.clone())?, f__int(Py::str("failures"), v_failures.clone(), Py::Int(1i64), Py::Int(10000i64))?])).items()?;
    if t2800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t2800[0].clone();
    v_r = t2800[1].clone();
    v_c = f__prob(Py::str("confidence"), v_confidence.clone(), Py::Float(0.0), Py::Float(1.0))?;
    let _ = f__need({ let t2804 = v_test.clone(); Py::Bool({ let t2805 = Py::tuple(vec![Py::str("time"), Py::str("failure")]); if py::contains(&t2805, &t2804)? { true } else { false } }) }, Py::str("test is 'time' (time-terminated) or 'failure' (failure-terminated)"))?;
    v_dl = (if ({ let t2806 = v_test.clone(); Py::Bool({ let t2807 = Py::str("time"); if py::eq(&t2806, &t2807) { true } else { false } }) }).truthy() { py::add(&py::mul(&Py::Int(2i64), &v_r.clone())?, &Py::Int(2i64))? } else { py::mul(&Py::Int(2i64), &v_r.clone())? });
    v_a = py::sub(&Py::Int(1i64), &v_c.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("mtbf"), py::truediv(&v_T.clone(), &v_r.clone())?), (Py::str("lower_one_sided"), py::truediv(&py::mul(&Py::Int(2i64), &v_T.clone())?, &f__chi2_even_ppf(v_c.clone(), py::floordiv(&v_dl.clone(), &Py::Int(2i64))?)?)?), (Py::str("lower_two_sided"), py::truediv(&py::mul(&Py::Int(2i64), &v_T.clone())?, &f__chi2_even_ppf(py::sub(&Py::Int(1i64), &py::truediv(&v_a.clone(), &Py::Int(2i64))?)?, py::floordiv(&v_dl.clone(), &Py::Int(2i64))?)?)?), (Py::str("upper_two_sided"), py::truediv(&py::mul(&Py::Int(2i64), &v_T.clone())?, &f__chi2_even_ppf(py::truediv(&v_a.clone(), &Py::Int(2i64))?, v_r.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multiproduct_break_even(mut v_fixed_cost: Py, mut v_prices: Py, mut v_variable_costs: Py, mut v_sales_mix: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_cmr: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_F = f__nonneg(Py::str("fixed_cost"), v_fixed_cost.clone())?;
    v_p = f__vec(Py::str("prices"), v_prices.clone(), Py::Int(1i64), Py::Int(200i64), Py::Bool(true), Py::Bool(false))?;
    v_v = f__vec(Py::str("variable_costs"), v_variable_costs.clone(), Py::Int(1i64), Py::Int(200i64), Py::Bool(false), Py::Bool(true))?;
    v_w = f__vec(Py::str("sales_mix"), v_sales_mix.clone(), Py::Int(1i64), Py::Int(200i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t2895 = py::len(&v_p.clone())?; Py::Bool({ let t2896 = py::len(&v_v.clone())?; if py::eq(&t2895, &t2896) { { let t2897 = py::len(&v_w.clone())?; if py::eq(&t2896, &t2897) { true } else { false } } } else { false } }) }, Py::str("prices, variable_costs and sales_mix must have the same number of values"))?;
    let _ = f__need({ let t2898 = py::abs(&py::sub(&py::sum(&v_w.clone(), None)?, &Py::Int(1i64))?)?; Py::Bool({ let t2899 = Py::Float(1e-09); if py::le(&t2898, &t2899)? { true } else { false } }) }, Py::str("the sales mix fractions must add up to 1"))?;
    v_cmr = py::sum(&{ let mut out = vec![]; for t2900 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2900;
out.push(py::mul(&py::sub(&Py::Int(1i64), &py::truediv(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&v_p.clone(), &v_i.clone())?)?)?, &py::getitem(&v_w.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2901 = v_cmr.clone(); Py::Bool({ let t2902 = Py::Int(0i64); if py::gt(&t2901, &t2902)? { true } else { false } }) }, Py::str("the weighted contribution must be positive"))?;
    v_R = py::truediv(&v_F.clone(), &v_cmr.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("break_even_revenue"), v_R.clone()), (Py::str("weighted_contribution_ratio"), v_cmr.clone()), (Py::str("revenue_by_product"), { let mut out = vec![]; for t2915 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t2915;
out.push(py::mul(&v_R.clone(), &v_x.clone())?); } Py::list(out) }), (Py::str("units_by_product"), { let mut out = vec![]; for t2916 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2916;
out.push(py::truediv(&py::mul(&v_R.clone(), &py::getitem(&v_w.clone(), &v_i.clone())?)?, &py::getitem(&v_p.clone(), &v_i.clone())?)?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_discrete(mut v_price: Py, mut v_cost: Py, mut v_demands: Py, mut v_probabilities: Py, mut v_salvage: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pr: Py = Py::None;
    let mut v_sales: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2948 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t2948.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2948[0].clone();
    v_c = t2948[1].clone();
    v_v = t2948[2].clone();
    v_cr = t2948[3].clone();
    v_d = f__vec(Py::str("demands"), v_demands.clone(), Py::Int(2i64), Py::Int(200i64), Py::Bool(false), Py::Bool(true))?;
    v_pr = f__vec(Py::str("probabilities"), v_probabilities.clone(), Py::Int(2i64), Py::Int(200i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t2965 = py::len(&v_d.clone())?; Py::Bool({ let t2966 = py::len(&v_pr.clone())?; if py::eq(&t2965, &t2966) { true } else { false } }) }, Py::str("demands and probabilities must have the same number of values"))?;
    for t2967 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_d.clone())?).i()?]) {
        v_k = t2967;
        let _ = f__need({ let t2968 = py::getitem(&v_d.clone(), &v_k.clone())?; Py::Bool({ let t2969 = py::getitem(&v_d.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t2968, &t2969)? { true } else { false } }) }, Py::str("demands must be listed in increasing order"))?;
    }
    v_tot = py::sum(&v_pr.clone(), None)?;
    let _ = f__need({ let t2970 = py::abs(&py::sub(&v_tot.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2971 = Py::Float(1e-09); if py::le(&t2970, &t2971)? { true } else { false } }) }, Py::str("the probabilities must add up to 1"))?;
    let t2972 = (Py::tuple(vec![Py::Float(0.0), Py::None])).items()?;
    if t2972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2972[0].clone();
    v_Q = t2972[1].clone();
    for t2973 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
        v_k = t2973;
        v_F = py::iadd(&v_F, &py::getitem(&v_pr.clone(), &v_k.clone())?)?;
        if ({ let t2978 = { let t2974 = py::abs(&py::sub(&v_F.clone(), &v_cr.clone())?)?; Py::Bool({ let t2975 = Py::Float(1e-12); if py::le(&t2974, &t2975)? { true } else { false } }) }; if !t2978.truthy() { t2978 } else { { let t2976 = v_k.clone(); Py::Bool({ let t2977 = py::sub(&py::len(&v_d.clone())?, &Py::Int(1i64))?; if py::lt(&t2976, &t2977)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("two order quantities are equally good (the cumulative probability equals the critical ratio)"))));
        }
        if ({ let t2983 = { let t2979 = v_F.clone(); Py::Bool({ let t2980 = v_cr.clone(); if py::gt(&t2979, &t2980)? { true } else { false } }) }; if t2983.truthy() { t2983 } else { { let t2981 = v_k.clone(); Py::Bool({ let t2982 = py::sub(&py::len(&v_d.clone())?, &Py::Int(1i64))?; if py::eq(&t2981, &t2982) { true } else { false } }) } } }).truthy() {
            v_Q = py::getitem(&v_d.clone(), &v_k.clone())?;
            break;
        }
    }
    v_sales = Py::Float(0.0);
    for t2984 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
        v_k = t2984;
        v_sales = py::iadd(&v_sales, &py::mul(&py::getitem(&v_pr.clone(), &v_k.clone())?, &py::minmax(&[py::getitem(&v_d.clone(), &v_k.clone())?, v_Q.clone()], false)?)?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("critical_ratio"), v_cr.clone()), (Py::str("expected_sales"), v_sales.clone()), (Py::str("expected_profit"), py::sub(&py::add(&py::mul(&v_p.clone(), &v_sales.clone())?, &py::mul(&v_v.clone(), &py::sub(&v_Q.clone(), &v_sales.clone())?)?)?, &py::mul(&v_c.clone(), &v_Q.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_normal(mut v_price: Py, mut v_cost: Py, mut v_mean_demand: Py, mut v_sd_demand: Py, mut v_salvage: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sales: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3028 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3028.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3028[0].clone();
    v_c = t3028[1].clone();
    v_v = t3028[2].clone();
    v_cr = t3028[3].clone();
    let t3029 = (Py::tuple(vec![f__fin(Py::str("mean_demand"), v_mean_demand.clone())?, f__pos(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t3029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t3029[0].clone();
    v_sd = t3029[1].clone();
    v_z = f__zinv(v_cr.clone())?;
    v_Q = py::add(&v_mu.clone(), &py::mul(&v_z.clone(), &v_sd.clone())?)?;
    v_sales = py::sub(&v_mu.clone(), &py::mul(&v_sd.clone(), &f__loss(v_z.clone())?)?)?;
    v_left = py::sub(&v_Q.clone(), &v_sales.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("critical_ratio"), v_cr.clone()), (Py::str("z"), v_z.clone()), (Py::str("expected_sales"), v_sales.clone()), (Py::str("expected_leftover"), v_left.clone()), (Py::str("expected_shortage"), py::sub(&v_mu.clone(), &v_sales.clone())?), (Py::str("expected_profit"), py::sub(&py::add(&py::mul(&v_p.clone(), &v_sales.clone())?, &py::mul(&v_v.clone(), &v_left.clone())?)?, &py::mul(&v_c.clone(), &v_Q.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_poisson(mut v_price: Py, mut v_cost: Py, mut v_mean_demand: Py, mut v_salvage: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kmax: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_sales: Py = Py::None;
    let mut v_short: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3073 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3073.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3073[0].clone();
    v_c = t3073[1].clone();
    v_v = t3073[2].clone();
    v_cr = t3073[3].clone();
    v_lam = f__pos(Py::str("mean_demand"), v_mean_demand.clone())?;
    let _ = f__need({ let t3074 = v_lam.clone(); Py::Bool({ let t3075 = Py::Int(500i64); if py::le(&t3074, &t3075)? { true } else { false } }) }, Py::str("mean_demand must be at most 500 for the Poisson model"))?;
    v_kmax = py::int(&py::add(&py::add(&v_lam.clone(), &py::mul(&Py::Int(40i64), &py::math1("sqrt", &v_lam.clone())?)?)?, &Py::Int(40i64))?)?;
    v_pm = f__pois_pmfs(v_lam.clone(), v_kmax.clone())?;
    let t3077 = (Py::tuple(vec![Py::Float(0.0), Py::None])).items()?;
    if t3077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3077[0].clone();
    v_Q = t3077[1].clone();
    for t3078 in py::range_iter(&[(py::add(&v_kmax.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t3078;
        v_F = py::iadd(&v_F, &py::getitem(&v_pm.clone(), &v_k.clone())?)?;
        if ({ let t3079 = py::abs(&py::sub(&v_F.clone(), &v_cr.clone())?)?; Py::Bool({ let t3080 = Py::Float(1e-12); if py::le(&t3079, &t3080)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("two order quantities are equally good (the cumulative probability equals the critical ratio)"))));
        }
        if ({ let t3081 = v_F.clone(); Py::Bool({ let t3082 = v_cr.clone(); if py::gt(&t3081, &t3082)? { true } else { false } }) }).truthy() {
            v_Q = v_k.clone();
            break;
        }
    }
    v_short = Py::Float(0.0);
    for t3083 in py::range_iter(&[(v_Q.clone()).i()?]) {
        v_k = t3083;
        v_short = py::iadd(&v_short, &py::mul(&py::sub(&v_Q.clone(), &v_k.clone())?, &py::getitem(&v_pm.clone(), &v_k.clone())?)?)?;
    }
    v_sales = py::sub(&v_Q.clone(), &v_short.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("critical_ratio"), v_cr.clone()), (Py::str("expected_sales"), v_sales.clone()), (Py::str("expected_leftover"), v_short.clone()), (Py::str("expected_profit"), py::sub(&py::add(&py::mul(&v_p.clone(), &v_sales.clone())?, &py::mul(&v_v.clone(), &v_short.clone())?)?, &py::mul(&v_c.clone(), &v_Q.clone())?)?), (Py::str("stockout_probability"), py::sub(&Py::Int(1i64), &v_F.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_uniform(mut v_price: Py, mut v_cost: Py, mut v_low: Py, mut v_high: Py, mut v_salvage: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sales: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3127 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3127.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3127[0].clone();
    v_c = t3127[1].clone();
    v_v = t3127[2].clone();
    v_cr = t3127[3].clone();
    let t3128 = (Py::tuple(vec![f__fin(Py::str("low"), v_low.clone())?, f__fin(Py::str("high"), v_high.clone())?])).items()?;
    if t3128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3128[0].clone();
    v_b = t3128[1].clone();
    let _ = f__need({ let t3129 = v_b.clone(); Py::Bool({ let t3130 = v_a.clone(); if py::gt(&t3129, &t3130)? { true } else { false } }) }, Py::str("high must exceed low"))?;
    v_Q = py::add(&v_a.clone(), &py::mul(&v_cr.clone(), &py::sub(&v_b.clone(), &v_a.clone())?)?)?;
    v_sales = py::sub(&v_Q.clone(), &py::truediv(&py::pow(&py::sub(&v_Q.clone(), &v_a.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::sub(&v_b.clone(), &v_a.clone())?)?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("critical_ratio"), v_cr.clone()), (Py::str("expected_sales"), v_sales.clone()), (Py::str("expected_leftover"), py::sub(&v_Q.clone(), &v_sales.clone())?), (Py::str("expected_profit"), py::sub(&py::add(&py::mul(&v_p.clone(), &v_sales.clone())?, &py::mul(&v_v.clone(), &py::sub(&v_Q.clone(), &v_sales.clone())?)?)?, &py::mul(&v_c.clone(), &v_Q.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_np_chart_limits(mut v_p_bar: Py, mut v_sample_size: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3183 = (Py::tuple(vec![f__prob(Py::str("p_bar"), v_p_bar.clone(), Py::Float(0.0), Py::Float(1.0))?, f__int(Py::str("sample_size"), v_sample_size.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t3183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3183[0].clone();
    v_n = t3183[1].clone();
    v_s = py::math1("sqrt", &py::mul(&py::mul(&v_n.clone(), &v_p.clone())?, &py::sub(&Py::Int(1i64), &v_p.clone())?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("ucl"), py::minmax(&[v_n.clone(), py::add(&py::mul(&v_n.clone(), &v_p.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?], false)?), (Py::str("lcl"), py::minmax(&[Py::Float(0.0), py::sub(&py::mul(&v_n.clone(), &v_p.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?), (Py::str("center"), py::mul(&v_n.clone(), &v_p.clone())?), (Py::str("sigma_np"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oee(mut v_planned_time: Py, mut v_downtime: Py, mut v_ideal_cycle_time: Py, mut v_total_count: Py, mut v_good_count: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_ct: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_perf: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_run_t: Py = Py::None;
    let t3225 = (Py::tuple(vec![f__pos(Py::str("planned_time"), v_planned_time.clone())?, f__nonneg(Py::str("downtime"), v_downtime.clone())?])).items()?;
    if t3225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3225[0].clone();
    v_dt = t3225[1].clone();
    let t3226 = (Py::tuple(vec![f__pos(Py::str("ideal_cycle_time"), v_ideal_cycle_time.clone())?, f__pos(Py::str("total_count"), v_total_count.clone())?, f__nonneg(Py::str("good_count"), v_good_count.clone())?])).items()?;
    if t3226.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ct = t3226[0].clone();
    v_n = t3226[1].clone();
    v_g = t3226[2].clone();
    let _ = f__need({ let t3227 = v_dt.clone(); Py::Bool({ let t3228 = v_T.clone(); if py::lt(&t3227, &t3228)? { true } else { false } }) }, Py::str("downtime must be less than the planned time"))?;
    let _ = f__need({ let t3229 = v_g.clone(); Py::Bool({ let t3230 = v_n.clone(); if py::le(&t3229, &t3230)? { true } else { false } }) }, Py::str("good count cannot exceed the total count"))?;
    v_run_t = py::sub(&v_T.clone(), &v_dt.clone())?;
    v_perf = py::truediv(&py::mul(&v_ct.clone(), &v_n.clone())?, &v_run_t.clone())?;
    let _ = f__need({ let t3231 = v_perf.clone(); Py::Bool({ let t3232 = py::add(&Py::Int(1i64), &Py::Float(1e-09))?; if py::le(&t3231, &t3232)? { true } else { false } }) }, Py::str("the count exceeds what the ideal cycle time allows in the run time"))?;
    let t3233 = (Py::tuple(vec![py::truediv(&v_run_t.clone(), &v_T.clone())?, py::truediv(&v_g.clone(), &v_n.clone())?])).items()?;
    if t3233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3233[0].clone();
    v_q = t3233[1].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("oee"), py::mul(&py::mul(&v_a.clone(), &v_perf.clone())?, &v_q.clone())?), (Py::str("availability"), v_a.clone()), (Py::str("performance"), v_perf.clone()), (Py::str("quality"), v_q.clone()), (Py::str("run_time"), v_run_t.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_order_up_to(mut v_mean_demand: Py, mut v_sd_demand: Py, mut v_review_period: Py, mut v_lead_time: Py, mut v_service_level: Py, mut v_inventory_position: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_csl: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ip: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3273 = (Py::tuple(vec![f__pos(Py::str("mean_demand"), v_mean_demand.clone())?, f__pos(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t3273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t3273[0].clone();
    v_s = t3273[1].clone();
    let t3276 = (Py::tuple(vec![f__pos(Py::str("review_period"), v_review_period.clone())?, f__nonneg(Py::str("lead_time"), v_lead_time.clone())?])).items()?;
    if t3276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3276[0].clone();
    v_L = t3276[1].clone();
    v_csl = f__prob(Py::str("service_level"), v_service_level.clone(), Py::Float(0.5), Py::Float(1.0))?;
    v_z = f__zinv(v_csl.clone())?;
    v_sig = py::mul(&v_s.clone(), &py::math1("sqrt", &py::add(&v_T.clone(), &v_L.clone())?)?)?;
    v_S = py::add(&py::mul(&v_d.clone(), &py::add(&v_T.clone(), &v_L.clone())?)?, &py::mul(&v_z.clone(), &v_sig.clone())?)?;
    v_out = Py::dict(vec![(Py::str("order_up_to_level"), v_S.clone()), (Py::str("safety_stock"), py::mul(&v_z.clone(), &v_sig.clone())?), (Py::str("z"), v_z.clone()), (Py::str("sd_protection_demand"), v_sig.clone())]);
    if ({ let t3280 = v_inventory_position.clone(); Py::Bool({ let t3281 = Py::None; if !py::is(&t3280, &t3281) { true } else { false } }) }).truthy() {
        v_ip = f__fin(Py::str("inventory_position"), v_inventory_position.clone())?;
        py::setitem(&v_out.clone(), &Py::str("order_quantity"), py::minmax(&[Py::Float(0.0), py::sub(&v_S.clone(), &v_ip.clone())?], true)?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_p_chart_counts(mut v_defectives: Py, mut v_sample_size: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lcl: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_ucl: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_d = f__vec(Py::str("defectives"), v_defectives.clone(), Py::Int(2i64), Py::Int(500i64), Py::Bool(false), Py::Bool(true))?;
    v_n = f__int(Py::str("sample_size"), v_sample_size.clone(), Py::Int(1i64), Py::None)?;
    for t3347 in py::iter(&(v_d.clone()))? {
        v_x = t3347;
        let _ = f__need({ let t3352 = { let t3348 = v_x.clone(); Py::Bool({ let t3349 = py::int(&v_x.clone())?; if py::eq(&t3348, &t3349) { true } else { false } }) }; if !t3352.truthy() { t3352 } else { { let t3350 = v_x.clone(); Py::Bool({ let t3351 = v_n.clone(); if py::le(&t3350, &t3351)? { true } else { false } }) } } }, Py::str("each count of defectives must be a whole number no larger than the sample size"))?;
    }
    v_p = py::truediv(&py::sum(&v_d.clone(), None)?, &py::mul(&v_n.clone(), &py::len(&v_d.clone())?)?)?;
    let _ = f__need({ let t3353 = Py::Int(0i64); Py::Bool({ let t3354 = v_p.clone(); if py::lt(&t3353, &t3354)? { { let t3355 = Py::Int(1i64); if py::lt(&t3354, &t3355)? { true } else { false } } } else { false } }) }, Py::str("the average fraction defective must lie strictly between 0 and 1"))?;
    v_s = py::math1("sqrt", &py::truediv(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_n.clone())?)?;
    let t3356 = (Py::tuple(vec![py::minmax(&[Py::Float(1.0), py::add(&v_p.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], false)?, py::minmax(&[Py::Float(0.0), py::sub(&v_p.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?])).items()?;
    if t3356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ucl = t3356[0].clone();
    v_lcl = t3356[1].clone();
    v_out = Py::list(vec![]);
    for t3357 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
        v_k = t3357;
        v_f = py::truediv(&py::getitem(&v_d.clone(), &v_k.clone())?, &v_n.clone())?;
        if ({ let t3362 = { let t3358 = v_f.clone(); Py::Bool({ let t3359 = v_ucl.clone(); if py::gt(&t3358, &t3359)? { true } else { false } }) }; if t3362.truthy() { t3362 } else { { let t3360 = v_f.clone(); Py::Bool({ let t3361 = v_lcl.clone(); if py::lt(&t3360, &t3361)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::add(&v_k.clone(), &Py::Int(1i64))?])?;
        }
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("p_bar"), v_p.clone()), (Py::str("ucl"), v_ucl.clone()), (Py::str("lcl"), v_lcl.clone()), (Py::str("out_of_control"), v_out.clone()), (Py::str("count_out"), py::len(&v_out.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_p_chart_limits(mut v_p_bar: Py, mut v_sample_size: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3415 = (Py::tuple(vec![f__prob(Py::str("p_bar"), v_p_bar.clone(), Py::Float(0.0), Py::Float(1.0))?, f__int(Py::str("sample_size"), v_sample_size.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t3415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3415[0].clone();
    v_n = t3415[1].clone();
    v_s = py::math1("sqrt", &py::truediv(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_n.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("ucl"), py::minmax(&[Py::Float(1.0), py::add(&v_p.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], false)?), (Py::str("lcl"), py::minmax(&[Py::Float(0.0), py::sub(&v_p.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?), (Py::str("center"), v_p.clone()), (Py::str("sigma_p"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_mttf(mut v_failure_rate: Py, mut v_n: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3467 = (Py::tuple(vec![f__pos(Py::str("failure_rate"), v_failure_rate.clone())?, f__int(Py::str("n"), v_n.clone(), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t3467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t3467[0].clone();
    v_nn = t3467[1].clone();
    v_s = Py::Float(0.0);
    for t3468 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nn.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t3468;
        v_s = py::iadd(&v_s, &py::truediv(&Py::Float(1.0), &v_i.clone())?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("mttf"), py::truediv(&v_s.clone(), &v_lam.clone())?), (Py::str("single_mttf"), py::truediv(&Py::Int(1i64), &v_lam.clone())?), (Py::str("improvement_factor"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_reliability(mut v_reliabilities: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_r = f__rel_list(Py::str("reliabilities"), v_reliabilities.clone())?;
    v_q = Py::Float(1.0);
    for t3526 in py::iter(&(v_r.clone()))? {
        v_v = t3526;
        v_q = py::mul(&v_q, &py::sub(&Py::Int(1i64), &v_v.clone())?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), py::sub(&Py::Int(1i64), &v_q.clone())?), (Py::str("unreliability"), v_q.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_performance(mut v_data: Py, mut v_usl: Py, mut v_lsl: Py) -> R {
    let mut v_l: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_ppl: Py = Py::None;
    let mut v_ppu: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("data"), v_data.clone(), Py::Int(2i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    let t3623 = (Py::tuple(vec![f__fin(Py::str("usl"), v_usl.clone())?, f__fin(Py::str("lsl"), v_lsl.clone())?])).items()?;
    if t3623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3623[0].clone();
    v_l = t3623[1].clone();
    let _ = f__need({ let t3624 = v_u.clone(); Py::Bool({ let t3625 = v_l.clone(); if py::gt(&t3624, &t3625)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    v_m = py::truediv(&py::sum(&v_x.clone(), None)?, &py::len(&v_x.clone())?)?;
    v_s = py::math1("sqrt", &py::truediv(&py::sum(&{ let mut out = vec![]; for t3626 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3626;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?, &py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?)?;
    let _ = f__need({ let t3627 = v_s.clone(); Py::Bool({ let t3628 = Py::Int(0i64); if py::gt(&t3627, &t3628)? { true } else { false } }) }, Py::str("the data must vary"))?;
    let t3629 = (Py::tuple(vec![py::truediv(&py::sub(&v_u.clone(), &v_m.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?, py::truediv(&py::sub(&v_m.clone(), &v_l.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?])).items()?;
    if t3629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ppu = t3629[0].clone();
    v_ppl = t3629[1].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("ppk"), py::minmax(&[v_ppu.clone(), v_ppl.clone()], false)?), (Py::str("pp"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("mean"), v_m.clone()), (Py::str("sd"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ranked_positional_weight(mut v_task_times: Py, mut v_precedence: Py, mut v_cycle_time: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_preds: Py = Py::None;
    let mut v_rank: Py = Py::None;
    let mut v_rpw: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_succ: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_t = f__vec(Py::str("task_times"), v_task_times.clone(), Py::Int(2i64), Py::Int(300i64), Py::Bool(true), Py::Bool(false))?;
    v_c = f__pos(Py::str("cycle_time"), v_cycle_time.clone())?;
    let _ = f__need({ let t3751 = py::minmax(&[v_t.clone()], true)?; Py::Bool({ let t3752 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t3751, &t3752)? { true } else { false } }) }, Py::str("every task must fit within the cycle time"))?;
    v_n = py::len(&v_t.clone())?;
    v_preds = f__precedence(v_n.clone(), v_precedence.clone())?;
    v_succ = { let mut out = vec![]; for t3801 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3801;
out.push(Py::list(vec![])); } Py::list(out) };
    for t3802 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t3802;
        for t3803 in py::iter(&(py::getitem(&v_preds.clone(), &v_j.clone())?))? {
            v_p = t3803;
            let _ = py::method(&py::getitem(&v_succ.clone(), &v_p.clone())?, "append", &[v_j.clone()])?;
        }
    }
    v_rpw = Py::list(vec![]);
    for t3804 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t3804;
        let t3805 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, Py::list(vec![v_j.clone()])])).items()?;
        if t3805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_seen = t3805[0].clone();
        v_stack = t3805[1].clone();
        while (v_stack.clone()).truthy() {
            v_u = py::method(&v_stack.clone(), "pop", &[])?;
            for t3806 in py::iter(&(py::getitem(&v_succ.clone(), &v_u.clone())?))? {
                v_v = t3806;
                if ({ let t3807 = v_v.clone(); Py::Bool({ let t3808 = v_seen.clone(); if !py::contains(&t3808, &t3807)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[v_v.clone()])?;
                    let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
                }
            }
        }
        let _ = py::method(&v_rpw.clone(), "append", &[py::add(&py::getitem(&v_t.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t3809 in py::iter(&(v_seen.clone()))? { let mut v_v: Py; 
v_v = t3809;
out.push(py::getitem(&v_t.clone(), &v_v.clone())?); } Py::list(out) }, None)?)?])?;
    }
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_c = v_c.clone(); let v_cycle_time = v_cycle_time.clone(); let v_n = v_n.clone(); let v_p = v_p.clone(); let v_precedence = v_precedence.clone(); let v_preds = v_preds.clone(); let v_rpw = v_rpw.clone(); let v_seen = v_seen.clone(); let v_stack = v_stack.clone(); let v_succ = v_succ.clone(); let v_t = v_t.clone(); let v_task_times = v_task_times.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_rpw.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_rank = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
    for t3810 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_pos = t3810;
        py::setitem(&v_rank.clone(), &py::getitem(&v_order.clone(), &v_pos.clone())?, py::sub(&v_n.clone(), &v_pos.clone())?)?;
    }
    v_out = f__balance(v_t.clone(), v_preds.clone(), v_c.clone(), v_rank.clone())?;
    py::setitem(&v_out.clone(), &Py::str("positional_weights"), v_rpw.clone())?;
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectilinear_location(mut v_x: Py, mut v_y: Py, mut v_weights: Py) -> R {
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t3890 = (f__xyw(v_x.clone(), v_y.clone(), v_weights.clone())?).items()?;
    if t3890.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t3890[0].clone();
    v_ys = t3890[1].clone();
    v_w = t3890[2].clone();
    let t3901 = (Py::tuple(vec![f__wmedian(v_xs.clone(), v_w.clone())?, f__wmedian(v_ys.clone(), v_w.clone())?])).items()?;
    if t3901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t3901[0].clone();
    v_by = t3901[1].clone();
    v_cost = py::sum(&{ let mut out = vec![]; for t3902 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3902;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::add(&py::abs(&py::sub(&v_bx.clone(), &py::getitem(&v_xs.clone(), &v_k.clone())?)?)?, &py::abs(&py::sub(&v_by.clone(), &py::getitem(&v_ys.clone(), &v_k.clone())?)?)?)?)?); } Py::list(out) }, None)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("x"), v_bx.clone()), (Py::str("y"), v_by.clone()), (Py::str("total_weighted_distance"), v_cost.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_redundancy_needed(mut v_target_reliability: Py, mut v_component_reliability: Py) -> R {
    let mut v_Rt: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_Rt = f__prob(Py::str("target_reliability"), v_target_reliability.clone(), Py::Float(0.0), Py::Float(1.0))?;
    v_r = f__prob(Py::str("component_reliability"), v_component_reliability.clone(), Py::Float(0.0), Py::Float(1.0))?;
    let t3943 = (Py::tuple(vec![Py::Int(1i64), py::sub(&Py::Int(1i64), &v_r.clone())?])).items()?;
    if t3943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3943[0].clone();
    v_q = t3943[1].clone();
    while ({ let t3946 = py::sub(&Py::Int(1i64), &v_q.clone())?; Py::Bool({ let t3947 = v_Rt.clone(); if py::lt(&t3946, &t3947)? { true } else { false } }) }).truthy() {
        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
        v_q = py::mul(&v_q, &py::sub(&Py::Int(1i64), &v_r.clone())?)?;
        let _ = f__need({ let t3944 = v_m.clone(); Py::Bool({ let t3945 = Py::Int(10000i64); if py::le(&t3944, &t3945)? { true } else { false } }) }, Py::str("more than 10000 components would be needed"))?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("components"), v_m.clone()), (Py::str("achieved_reliability"), py::sub(&Py::Int(1i64), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regression_forecast(mut v_x: Py, mut v_y: Py, mut v_x_new: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t4001 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(3i64), Py::Int(100000i64), Py::Bool(false), Py::Bool(false))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64), Py::Int(100000i64), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t4001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t4001[0].clone();
    v_ys = t4001[1].clone();
    let _ = f__need({ let t4002 = py::len(&v_xs.clone())?; Py::Bool({ let t4003 = py::len(&v_ys.clone())?; if py::eq(&t4002, &t4003) { true } else { false } }) }, Py::str("x and y must have the same number of values"))?;
    v_n = py::len(&v_xs.clone())?;
    let t4004 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t4004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4004[0].clone();
    v_my = t4004[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t4005 in py::iter(&(v_xs.clone()))? { let mut v_v: Py; 
v_v = t4005;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t4006 in py::iter(&(v_ys.clone()))? { let mut v_v: Py; 
v_v = t4006;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t4011 = { let t4007 = v_sxx.clone(); Py::Bool({ let t4008 = Py::Int(0i64); if py::gt(&t4007, &t4008)? { true } else { false } }) }; if !t4011.truthy() { t4011 } else { { let t4009 = v_syy.clone(); Py::Bool({ let t4010 = Py::Int(0i64); if py::gt(&t4009, &t4010)? { true } else { false } }) } } }, Py::str("x and y must both vary"))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t4012 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4013 = (t4012).items()?;
if t4013.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4013[0].clone();
v_b = t4013[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    v_sse = py::sum(&{ let mut out = vec![]; for t4014 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4014;
out.push(py::pow(&py::sub(&py::sub(&py::getitem(&v_ys.clone(), &v_i.clone())?, &v_a.clone())?, &py::mul(&v_b.clone(), &py::getitem(&v_xs.clone(), &v_i.clone())?)?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_out = Py::dict(vec![(Py::str("intercept"), v_a.clone()), (Py::str("slope"), v_b.clone()), (Py::str("r"), py::truediv(&v_sxy.clone(), &py::math1("sqrt", &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)?), (Py::str("r_squared"), py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?), (Py::str("standard_error"), py::math1("sqrt", &py::truediv(&v_sse.clone(), &py::sub(&v_n.clone(), &Py::Int(2i64))?)?)?)]);
    if ({ let t4015 = v_x_new.clone(); Py::Bool({ let t4016 = Py::None; if !py::is(&t4015, &t4016) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("forecast"), py::add(&v_a.clone(), &py::mul(&v_b.clone(), &f__fin(Py::str("x_new"), v_x_new.clone())?)?)?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reorder_point(mut v_demand_rate: Py, mut v_lead_time: Py, mut v_safety_stock: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let t4058 = (Py::tuple(vec![f__pos(Py::str("demand_rate"), v_demand_rate.clone())?, f__nonneg(Py::str("lead_time"), v_lead_time.clone())?, f__nonneg(Py::str("safety_stock"), v_safety_stock.clone())?])).items()?;
    if t4058.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4058[0].clone();
    v_L = t4058[1].clone();
    v_ss = t4058[2].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("reorder_point"), py::add(&py::mul(&v_d.clone(), &v_L.clone())?, &v_ss.clone())?), (Py::str("lead_time_demand"), py::mul(&v_d.clone(), &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rolled_yield(mut v_yields: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_y = f__vec(Py::str("yields"), v_yields.clone(), Py::Int(1i64), Py::Int(200i64), Py::Bool(false), Py::Bool(false))?;
    for t4112 in py::iter(&(v_y.clone()))? {
        v_v = t4112;
        let _ = f__need({ let t4113 = Py::Int(0i64); Py::Bool({ let t4114 = v_v.clone(); if py::lt(&t4113, &t4114)? { { let t4115 = Py::Int(1i64); if py::le(&t4114, &t4115)? { true } else { false } } } else { false } }) }, Py::str("every yield must lie in (0, 1]"))?;
    }
    v_p = Py::Float(1.0);
    for t4116 in py::iter(&(v_y.clone()))? {
        v_v = t4116;
        v_p = py::mul(&v_p, &v_v.clone())?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("rty"), v_p.clone()), (Py::str("normalized_yield"), py::pow(&v_p.clone(), &py::truediv(&Py::Float(1.0), &py::len(&v_y.clone())?)?)?), (Py::str("total_dpu"), py::add(&py::neg(&py::math1("log", &v_p.clone())?)?, &Py::Float(0.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rq_fill_rate(mut v_mean_ltd: Py, mut v_sd_ltd: Py, mut v_order_quantity: Py, mut v_fill_rate: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_target: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t4156 = (Py::tuple(vec![f__fin(Py::str("mean_ltd"), v_mean_ltd.clone())?, f__pos(Py::str("sd_ltd"), v_sd_ltd.clone())?])).items()?;
    if t4156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t4156[0].clone();
    v_s = t4156[1].clone();
    let t4160 = (Py::tuple(vec![f__pos(Py::str("order_quantity"), v_order_quantity.clone())?, f__prob(Py::str("fill_rate"), v_fill_rate.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t4160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t4160[0].clone();
    v_beta = t4160[1].clone();
    v_target = py::truediv(&py::mul(&py::sub(&Py::Int(1i64), &v_beta.clone())?, &v_Q.clone())?, &v_s.clone())?;
    let _ = f__need({ let t4161 = v_target.clone(); Py::Bool({ let t4162 = f__loss(py::neg(&Py::Float(8.0))?)?; if py::lt(&t4161, &t4162)? { true } else { false } }) }, Py::str("the fill rate is too low for this order quantity (the reorder point would be negative)"))?;
    let _ = f__need({ let t4163 = v_target.clone(); Py::Bool({ let t4164 = f__loss(Py::Float(8.0))?; if py::gt(&t4163, &t4164)? { true } else { false } }) }, Py::str("the fill rate is too close to 1 for this order quantity"))?;
    v_z = f__bisect_root({ let v_Q = v_Q.clone(); let v_beta = v_beta.clone(); let v_fill_rate = v_fill_rate.clone(); let v_mean_ltd = v_mean_ltd.clone(); let v_mu = v_mu.clone(); let v_order_quantity = v_order_quantity.clone(); let v_s = v_s.clone(); let v_sd_ltd = v_sd_ltd.clone(); let v_target = v_target.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__loss(v_x.clone())?, &v_target.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::neg(&Py::Float(8.0))?, Py::Float(8.0), Py::Int(300i64))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("reorder_point"), py::add(&v_mu.clone(), &py::mul(&v_z.clone(), &v_s.clone())?)?), (Py::str("safety_stock"), py::mul(&v_z.clone(), &v_s.clone())?), (Py::str("z"), v_z.clone()), (Py::str("expected_shortage_per_cycle"), py::mul(&v_target.clone(), &v_s.clone())?), (Py::str("cycle_service_level"), f__cdf(v_z.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_safety_stock_normal(mut v_mean_demand: Py, mut v_sd_demand: Py, mut v_lead_time: Py, mut v_service_level: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_csl: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t4230 = (Py::tuple(vec![f__pos(Py::str("mean_demand"), v_mean_demand.clone())?, f__pos(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t4230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4230[0].clone();
    v_s = t4230[1].clone();
    let t4234 = (Py::tuple(vec![f__pos(Py::str("lead_time"), v_lead_time.clone())?, f__prob(Py::str("service_level"), v_service_level.clone(), Py::Float(0.5), Py::Float(1.0))?])).items()?;
    if t4234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t4234[0].clone();
    v_csl = t4234[1].clone();
    v_z = f__zinv(v_csl.clone())?;
    v_sl = py::mul(&v_s.clone(), &py::math1("sqrt", &v_L.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("safety_stock"), py::mul(&v_z.clone(), &v_sl.clone())?), (Py::str("reorder_point"), py::add(&py::mul(&v_d.clone(), &v_L.clone())?, &py::mul(&v_z.clone(), &v_sl.clone())?)?), (Py::str("z"), v_z.clone()), (Py::str("sd_lead_time_demand"), v_sl.clone()), (Py::str("lead_time_demand"), py::mul(&v_d.clone(), &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_safety_stock_vlt(mut v_mean_demand: Py, mut v_sd_demand: Py, mut v_mean_lead_time: Py, mut v_sd_lead_time: Py, mut v_service_level: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_csl: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sL: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t4276 = (Py::tuple(vec![f__pos(Py::str("mean_demand"), v_mean_demand.clone())?, f__nonneg(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t4276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4276[0].clone();
    v_s = t4276[1].clone();
    let t4277 = (Py::tuple(vec![f__pos(Py::str("mean_lead_time"), v_mean_lead_time.clone())?, f__nonneg(Py::str("sd_lead_time"), v_sd_lead_time.clone())?])).items()?;
    if t4277.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t4277[0].clone();
    v_sL = t4277[1].clone();
    v_csl = f__prob(Py::str("service_level"), v_service_level.clone(), Py::Float(0.5), Py::Float(1.0))?;
    let _ = f__need({ let t4285 = { let t4281 = v_s.clone(); Py::Bool({ let t4282 = Py::Int(0i64); if py::gt(&t4281, &t4282)? { true } else { false } }) }; if t4285.truthy() { t4285 } else { { let t4283 = v_sL.clone(); Py::Bool({ let t4284 = Py::Int(0i64); if py::gt(&t4283, &t4284)? { true } else { false } }) } } }, Py::str("demand or lead time must vary (a standard deviation must be positive)"))?;
    v_sig = py::math1("sqrt", &py::add(&py::mul(&py::mul(&v_L.clone(), &v_s.clone())?, &v_s.clone())?, &py::mul(&py::mul(&py::mul(&v_d.clone(), &v_d.clone())?, &v_sL.clone())?, &v_sL.clone())?)?)?;
    v_z = f__zinv(v_csl.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("safety_stock"), py::mul(&v_z.clone(), &v_sig.clone())?), (Py::str("reorder_point"), py::add(&py::mul(&v_d.clone(), &v_L.clone())?, &py::mul(&v_z.clone(), &v_sig.clone())?)?), (Py::str("sd_lead_time_demand"), v_sig.clone()), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_seasonal_index(mut v_data: Py, mut v_next_year_total: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_overall: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_season: Py = Py::None;
    let mut v_years: Py = Py::None;
    v_m = f__mat(Py::str("data"), v_data.clone(), Py::Int(1i64), Py::None, Py::Int(2i64), Py::Int(366i64))?;
    let t4344 = (Py::tuple(vec![py::len(&v_m.clone())?, py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4344.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_years = t4344[0].clone();
    v_s = t4344[1].clone();
    v_season = { let mut out = vec![]; for t4345 in py::range_iter(&[(v_s.clone()).i()?]) { let mut v_k: Py; 
v_k = t4345;
out.push(py::truediv(&py::sum(&{ let mut out = vec![]; for t4346 in py::range_iter(&[(v_years.clone()).i()?]) { let mut v_y: Py; 
v_y = t4346;
out.push(py::getitem(&py::getitem(&v_m.clone(), &v_y.clone())?, &v_k.clone())?); } Py::list(out) }, None)?, &v_years.clone())?); } Py::list(out) };
    v_overall = py::truediv(&py::sum(&v_season.clone(), None)?, &v_s.clone())?;
    let _ = f__need({ let t4347 = v_overall.clone(); Py::Bool({ let t4348 = Py::Int(0i64); if py::gt(&t4347, &t4348)? { true } else { false } }) }, Py::str("the average demand must be positive"))?;
    v_idx = { let mut out = vec![]; for t4349 in py::iter(&(v_season.clone()))? { let mut v_v: Py; 
v_v = t4349;
out.push(py::truediv(&v_v.clone(), &v_overall.clone())?); } Py::list(out) };
    v_out = Py::dict(vec![(Py::str("indices"), v_idx.clone()), (Py::str("season_averages"), v_season.clone()), (Py::str("overall_average"), v_overall.clone())]);
    if ({ let t4350 = v_next_year_total.clone(); Py::Bool({ let t4351 = Py::None; if !py::is(&t4350, &t4351) { true } else { false } }) }).truthy() {
        v_T = f__nonneg(Py::str("next_year_total"), v_next_year_total.clone())?;
        py::setitem(&v_out.clone(), &Py::str("forecasts"), { let mut out = vec![]; for t4354 in py::iter(&(v_idx.clone()))? { let mut v_v: Py; 
v_v = t4354;
out.push(py::mul(&py::truediv(&v_T.clone(), &v_s.clone())?, &v_v.clone())?); } Py::list(out) })?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_parallel(mut v_stage_reliabilities: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_m = f__mat(Py::str("stage_reliabilities"), v_stage_reliabilities.clone(), Py::Int(1i64), Py::None, Py::Int(1i64), Py::None)?;
    v_R = Py::Float(1.0);
    for t4413 in py::iter(&(v_m.clone()))? {
        v_row = t4413;
        v_q = Py::Float(1.0);
        for t4414 in py::iter(&(v_row.clone()))? {
            v_v = t4414;
            let _ = f__need({ let t4415 = Py::Int(0i64); Py::Bool({ let t4416 = v_v.clone(); if py::le(&t4415, &t4416)? { { let t4417 = Py::Int(1i64); if py::le(&t4416, &t4417)? { true } else { false } } } else { false } }) }, Py::str("every reliability must lie between 0 and 1"))?;
            v_q = py::mul(&v_q, &py::sub(&Py::Int(1i64), &v_v.clone())?)?;
        }
        v_R = py::mul(&v_R, &py::sub(&Py::Int(1i64), &v_q.clone())?)?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), v_R.clone()), (Py::str("unreliability"), py::sub(&Py::Int(1i64), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_reliability(mut v_reliabilities: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_r = f__rel_list(Py::str("reliabilities"), v_reliabilities.clone())?;
    v_p = Py::Float(1.0);
    for t4475 in py::iter(&(v_r.clone()))? {
        v_v = t4475;
        v_p = py::mul(&v_p, &v_v.clone())?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), v_p.clone()), (Py::str("unreliability"), py::sub(&Py::Int(1i64), &v_p.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_server_cost(mut v_arrival_rate: Py, mut v_service_rate: Py, mut v_server_cost_rate: Py, mut v_waiting_cost_rate: Py, mut v_max_servers: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cmax: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_cw: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let t3669 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t3669.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t3669[0].clone();
    v_mu = t3669[1].clone();
    let t3672 = (Py::tuple(vec![f__nonneg(Py::str("server_cost_rate"), v_server_cost_rate.clone())?, f__nonneg(Py::str("waiting_cost_rate"), v_waiting_cost_rate.clone())?])).items()?;
    if t3672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cs = t3672[0].clone();
    v_cw = t3672[1].clone();
    v_cmax = f__int(Py::str("max_servers"), v_max_servers.clone(), Py::Int(1i64), Py::Int(500i64))?;
    v_best = Py::None;
    v_c = py::add(&py::int(&py::truediv(&v_lam.clone(), &v_mu.clone())?)?, &Py::Int(1i64))?;
    while ({ let t3692 = v_c.clone(); Py::Bool({ let t3693 = v_cmax.clone(); if py::le(&t3692, &t3693)? { true } else { false } }) }).truthy() {
        v_C = f__erlang_c(v_lam.clone(), v_mu.clone(), v_c.clone())?;
        v_rho = py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?;
        v_L = py::add(&py::truediv(&py::mul(&v_C.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?, &py::truediv(&v_lam.clone(), &v_mu.clone())?)?;
        v_cost = py::add(&py::mul(&v_c.clone(), &v_cs.clone())?, &py::mul(&v_cw.clone(), &v_L.clone())?)?;
        if ({ let t3691 = { let t3687 = v_best.clone(); Py::Bool({ let t3688 = Py::None; if py::is(&t3687, &t3688) { true } else { false } }) }; if t3691.truthy() { t3691 } else { { let t3689 = v_cost.clone(); Py::Bool({ let t3690 = py::sub(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &py::mul(&Py::Float(1e-12), &v_cost.clone())?)?; if py::lt(&t3689, &t3690)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_cost.clone(), v_c.clone(), v_L.clone()]);
        }
        v_c = py::iadd(&v_c, &Py::Int(1i64))?;
    }
    let _ = f__need({ let t3694 = v_best.clone(); Py::Bool({ let t3695 = Py::None; if !py::is(&t3694, &t3695) { true } else { false } }) }, Py::str("max_servers is too small for a stable queue"))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("servers"), py::getitem(&v_best.clone(), &Py::Int(1i64))?), (Py::str("total_cost"), py::getitem(&v_best.clone(), &Py::Int(0i64))?), (Py::str("L"), py::getitem(&v_best.clone(), &Py::Int(2i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_service_from_rop(mut v_mean_ltd: Py, mut v_sd_ltd: Py, mut v_reorder_point: Py, mut v_order_quantity: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_esc: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t4515 = (Py::tuple(vec![f__fin(Py::str("mean_ltd"), v_mean_ltd.clone())?, f__pos(Py::str("sd_ltd"), v_sd_ltd.clone())?, f__fin(Py::str("reorder_point"), v_reorder_point.clone())?])).items()?;
    if t4515.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t4515[0].clone();
    v_s = t4515[1].clone();
    v_R = t4515[2].clone();
    v_z = py::truediv(&py::sub(&v_R.clone(), &v_mu.clone())?, &v_s.clone())?;
    v_esc = py::mul(&v_s.clone(), &f__loss(v_z.clone())?)?;
    v_out = Py::dict(vec![(Py::str("cycle_service_level"), f__cdf(v_z.clone())?), (Py::str("z"), v_z.clone()), (Py::str("safety_stock"), py::sub(&v_R.clone(), &v_mu.clone())?), (Py::str("expected_shortage_per_cycle"), v_esc.clone())]);
    if ({ let t4516 = v_order_quantity.clone(); Py::Bool({ let t4517 = Py::None; if !py::is(&t4516, &t4517) { true } else { false } }) }).truthy() {
        v_Q = f__pos(Py::str("order_quantity"), v_order_quantity.clone())?;
        py::setitem(&v_out.clone(), &Py::str("fill_rate"), py::minmax(&[Py::Float(0.0), py::sub(&Py::Int(1i64), &py::truediv(&v_esc.clone(), &v_Q.clone())?)?], true)?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sigma_from_range(mut v_mean_range: Py, mut v_subgroup_size: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t4569 = (Py::tuple(vec![f__pos(Py::str("mean_range"), v_mean_range.clone())?, f__int(Py::str("subgroup_size"), v_subgroup_size.clone(), Py::Int(2i64), Py::Int(10i64))?])).items()?;
    if t4569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rb = t4569[0].clone();
    v_n = t4569[1].clone();
    v_s = py::truediv(&v_rb.clone(), &py::getitem(&c__D2()?, &v_n.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sigma"), v_s.clone()), (Py::str("d2"), py::getitem(&c__D2()?, &v_n.clone())?), (Py::str("natural_tolerance"), py::mul(&Py::Int(6i64), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spt_sequence(mut v_processing_times: Py, mut v_due_dates: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4627 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t4627.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4627[0].clone();
    v_d = t4627[1].clone();
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_p.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(f__rd(f__metrics(v_order.clone(), v_p.clone(), v_d.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_staffing(mut v_arrival_rate: Py, mut v_service_rate: Py, mut v_target_time: Py, mut v_target_level: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_level: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t866 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t866[0].clone();
    v_mu = t866[1].clone();
    let t872 = (Py::tuple(vec![f__nonneg(Py::str("target_time"), v_target_time.clone())?, f__prob(Py::str("target_level"), v_target_level.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t872[0].clone();
    v_sl = t872[1].clone();
    v_c = py::add(&py::int(&py::truediv(&v_lam.clone(), &v_mu.clone())?)?, &Py::Int(1i64))?;
    while (Py::Bool(true)).truthy() {
        let _ = f__need({ let t873 = v_c.clone(); Py::Bool({ let t874 = Py::Int(2000i64); if py::le(&t873, &t874)? { true } else { false } }) }, Py::str("more than 2000 servers would be needed"))?;
        v_C = f__erlang_c(v_lam.clone(), v_mu.clone(), v_c.clone())?;
        v_level = py::sub(&Py::Int(1i64), &py::mul(&v_C.clone(), &py::math1("exp", &py::mul(&py::neg(&py::sub(&py::mul(&v_c.clone(), &v_mu.clone())?, &v_lam.clone())?)?, &v_t.clone())?)?)?)?;
        if ({ let t877 = v_level.clone(); Py::Bool({ let t878 = v_sl.clone(); if py::ge(&t877, &t878)? { true } else { false } }) }).truthy() {
            break;
        }
        v_c = py::iadd(&v_c, &Py::Int(1i64))?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("servers"), v_c.clone()), (Py::str("service_level"), v_level.clone()), (Py::str("p_wait"), v_C.clone()), (Py::str("utilization"), py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?), (Py::str("mean_wait"), py::truediv(&v_C.clone(), &py::sub(&py::mul(&v_c.clone(), &v_mu.clone())?, &v_lam.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_standard_time(mut v_observed_times: Py, mut v_rating: Py, mut v_allowance: Py, mut v_basis: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_nt: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("observed_times"), v_observed_times.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(true), Py::Bool(false))?;
    let t4696 = (Py::tuple(vec![f__pos(Py::str("rating"), v_rating.clone())?, f__prob(Py::str("allowance"), v_allowance.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?])).items()?;
    if t4696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t4696[0].clone();
    v_a = t4696[1].clone();
    let _ = f__need({ let t4697 = v_basis.clone(); Py::Bool({ let t4698 = Py::tuple(vec![Py::str("job"), Py::str("shift")]); if py::contains(&t4698, &t4697)? { true } else { false } }) }, Py::str("basis is 'job' (allowance as a fraction of normal time) or 'shift' (of the workday)"))?;
    v_mean = py::truediv(&py::sum(&v_x.clone(), None)?, &py::len(&v_x.clone())?)?;
    v_nt = py::mul(&v_mean.clone(), &v_r.clone())?;
    v_st = (if ({ let t4699 = v_basis.clone(); Py::Bool({ let t4700 = Py::str("job"); if py::eq(&t4699, &t4700) { true } else { false } }) }).truthy() { py::mul(&v_nt.clone(), &py::add(&Py::Int(1i64), &v_a.clone())?)? } else { py::truediv(&v_nt.clone(), &py::sub(&Py::Int(1i64), &v_a.clone())?)? });
    return Ok(f__rd(Py::dict(vec![(Py::str("standard_time"), v_st.clone()), (Py::str("normal_time"), v_nt.clone()), (Py::str("mean_observed_time"), v_mean.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_standby_redundancy(mut v_failure_rate: Py, mut v_time: Py, mut v_units: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_term: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t4754 = (Py::tuple(vec![f__pos(Py::str("failure_rate"), v_failure_rate.clone())?, f__nonneg(Py::str("time"), v_time.clone())?, f__int(Py::str("units"), v_units.clone(), Py::Int(1i64), Py::Int(200i64))?])).items()?;
    if t4754.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t4754[0].clone();
    v_t = t4754[1].clone();
    v_n = t4754[2].clone();
    v_x = py::mul(&v_lam.clone(), &v_t.clone())?;
    let t4755 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t4755.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t4755[0].clone();
    v_s = t4755[1].clone();
    for t4756 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t4756;
        v_term = py::mul(&v_term, &py::truediv(&v_x.clone(), &v_k.clone())?)?;
        v_s = py::iadd(&v_s, &v_term.clone())?;
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), py::mul(&py::math1("exp", &py::neg(&v_x.clone())?)?, &v_s.clone())?), (Py::str("mttf"), py::truediv(&v_n.clone(), &v_lam.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_system_failure_rate(mut v_failure_rates: Py, mut v_time: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_lam = f__vec(Py::str("failure_rates"), v_failure_rates.clone(), Py::Int(1i64), Py::Int(1000i64), Py::Bool(true), Py::Bool(false))?;
    v_L = py::sum(&v_lam.clone(), None)?;
    v_out = Py::dict(vec![(Py::str("system_failure_rate"), v_L.clone()), (Py::str("mtbf"), py::truediv(&Py::Int(1i64), &v_L.clone())?)]);
    if ({ let t4810 = v_time.clone(); Py::Bool({ let t4811 = Py::None; if !py::is(&t4810, &t4811) { true } else { false } }) }).truthy() {
        v_t = f__nonneg(Py::str("time"), v_time.clone())?;
        py::setitem(&v_out.clone(), &Py::str("reliability"), py::math1("exp", &py::mul(&py::neg(&v_L.clone())?, &v_t.clone())?)?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tabular_cusum(mut v_data: Py, mut v_target: Py, mut v_k: Py, mut v_h: Py) -> R {
    let mut v_cm: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_downs: Py = Py::None;
    let mut v_hh: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_kk: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_ups: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("data"), v_data.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    let t4871 = (Py::tuple(vec![f__fin(Py::str("target"), v_target.clone())?, f__nonneg(Py::str("k"), v_k.clone())?, f__pos(Py::str("h"), v_h.clone())?])).items()?;
    if t4871.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4871[0].clone();
    v_kk = t4871[1].clone();
    v_hh = t4871[2].clone();
    let t4872 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Int(0i64)])).items()?;
    if t4872.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_cp = t4872[0].clone();
    v_cm = t4872[1].clone();
    v_sig = t4872[2].clone();
    let t4873 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t4873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ups = t4873[0].clone();
    v_downs = t4873[1].clone();
    for t4874 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) {
        v_i = t4874;
        v_cp = py::minmax(&[Py::Float(0.0), py::add(&py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::add(&v_m.clone(), &v_kk.clone())?)?, &v_cp.clone())?], true)?;
        v_cm = py::minmax(&[Py::Float(0.0), py::add(&py::sub(&py::sub(&v_m.clone(), &v_kk.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_cm.clone())?], true)?;
        let _ = py::method(&v_ups.clone(), "append", &[v_cp.clone()])?;
        let _ = py::method(&v_downs.clone(), "append", &[v_cm.clone()])?;
        if ({ let t4882 = { let t4875 = v_sig.clone(); Py::Bool({ let t4876 = Py::Int(0i64); if py::eq(&t4875, &t4876) { true } else { false } }) }; if !t4882.truthy() { t4882 } else { { let t4881 = { let t4877 = v_cp.clone(); Py::Bool({ let t4878 = v_hh.clone(); if py::gt(&t4877, &t4878)? { true } else { false } }) }; if t4881.truthy() { t4881 } else { { let t4879 = v_cm.clone(); Py::Bool({ let t4880 = v_hh.clone(); if py::gt(&t4879, &t4880)? { true } else { false } }) } } } } }).truthy() {
            v_sig = py::add(&v_i.clone(), &Py::Int(1i64))?;
        }
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("first_signal"), v_sig.clone()), (Py::str("c_plus"), v_ups.clone()), (Py::str("c_minus"), v_downs.clone()), (Py::str("final_c_plus"), v_cp.clone()), (Py::str("final_c_minus"), v_cm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_takt_time(mut v_available_time: Py, mut v_demand: Py, mut v_total_task_time: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t4922 = (Py::tuple(vec![f__pos(Py::str("available_time"), v_available_time.clone())?, f__pos(Py::str("demand"), v_demand.clone())?])).items()?;
    if t4922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t4922[0].clone();
    v_D = t4922[1].clone();
    v_c = py::truediv(&v_T.clone(), &v_D.clone())?;
    v_out = Py::dict(vec![(Py::str("cycle_time"), v_c.clone()), (Py::str("output_rate"), py::truediv(&v_D.clone(), &v_T.clone())?)]);
    if ({ let t4923 = v_total_task_time.clone(); Py::Bool({ let t4924 = Py::None; if !py::is(&t4923, &t4924) { true } else { false } }) }).truthy() {
        v_w = f__pos(Py::str("total_task_time"), v_total_task_time.clone())?;
        py::setitem(&v_out.clone(), &Py::str("minimum_stations"), f__ceil(py::truediv(&v_w.clone(), &v_c.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("exact_stations"), py::truediv(&v_w.clone(), &v_c.clone())?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_time_study_size(mut v_mean: Py, mut v_sd: Py, mut v_accuracy: Py, mut v_confidence: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t4966 = (Py::tuple(vec![f__pos(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?])).items()?;
    if t4966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4966[0].clone();
    v_s = t4966[1].clone();
    let t4970 = (Py::tuple(vec![f__prob(Py::str("accuracy"), v_accuracy.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("confidence"), v_confidence.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t4970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t4970[0].clone();
    v_c = t4970[1].clone();
    v_z = f__zinv(py::sub(&Py::Int(1i64), &py::truediv(&py::sub(&Py::Int(1i64), &v_c.clone())?, &Py::Int(2i64))?)?)?;
    v_x = py::pow(&py::truediv(&py::mul(&v_z.clone(), &v_s.clone())?, &py::mul(&v_h.clone(), &v_m.clone())?)?, &Py::Int(2i64))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sample_size"), f__ceil(v_x.clone())?), (Py::str("exact"), v_x.clone()), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tracking_signal(mut v_actual: Py, mut v_forecast: Py, mut v_limit: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lim: Py = Py::None;
    let mut v_mad: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_sa: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t5028 = (f__pair(v_actual.clone(), v_forecast.clone())?).items()?;
    if t5028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5028[0].clone();
    v_f = t5028[1].clone();
    v_lim = f__pos(Py::str("limit"), v_limit.clone())?;
    let t5031 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t5031.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rs = t5031[0].clone();
    v_sa = t5031[1].clone();
    v_ts = t5031[2].clone();
    v_first = t5031[3].clone();
    for t5032 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) {
        v_i = t5032;
        v_e = py::sub(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::getitem(&v_f.clone(), &v_i.clone())?)?;
        v_rs = py::iadd(&v_rs, &v_e.clone())?;
        v_sa = py::iadd(&v_sa, &py::abs(&v_e.clone())?)?;
        v_mad = py::truediv(&v_sa.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?;
        let _ = f__need({ let t5037 = { let t5033 = v_mad.clone(); Py::Bool({ let t5034 = Py::Int(0i64); if py::gt(&t5033, &t5034)? { true } else { false } }) }; if t5037.truthy() { t5037 } else { { let t5035 = v_rs.clone(); Py::Bool({ let t5036 = Py::Int(0i64); if py::eq(&t5035, &t5036) { true } else { false } }) } } }, Py::str("the tracking signal is undefined"))?;
        v_v = (if ({ let t5038 = v_mad.clone(); Py::Bool({ let t5039 = Py::Int(0i64); if py::gt(&t5038, &t5039)? { true } else { false } }) }).truthy() { py::truediv(&v_rs.clone(), &v_mad.clone())? } else { Py::Float(0.0) });
        let _ = py::method(&v_ts.clone(), "append", &[v_v.clone()])?;
        if ({ let t5044 = { let t5040 = v_first.clone(); Py::Bool({ let t5041 = Py::Int(0i64); if py::eq(&t5040, &t5041) { true } else { false } }) }; if !t5044.truthy() { t5044 } else { { let t5042 = py::abs(&v_v.clone())?; Py::Bool({ let t5043 = v_lim.clone(); if py::gt(&t5042, &t5043)? { true } else { false } }) } } }).truthy() {
            v_first = py::add(&v_i.clone(), &Py::Int(1i64))?;
        }
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("final_signal"), py::getitem(&v_ts.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("signals"), v_ts.clone()), (Py::str("first_out_of_control"), v_first.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_u_chart_limits(mut v_u_bar: Py, mut v_units: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t5084 = (Py::tuple(vec![f__pos(Py::str("u_bar"), v_u_bar.clone())?, f__pos(Py::str("units"), v_units.clone())?])).items()?;
    if t5084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t5084[0].clone();
    v_n = t5084[1].clone();
    v_s = py::math1("sqrt", &py::truediv(&v_u.clone(), &v_n.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("ucl"), py::add(&v_u.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?), (Py::str("lcl"), py::minmax(&[Py::Float(0.0), py::sub(&v_u.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?), (Py::str("center"), v_u.clone()), (Py::str("sigma_u"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weber_location(mut v_x: Py, mut v_y: Py, mut v_weights: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nx: Py = Py::None;
    let mut v_ny: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t5144 = (f__xyw(v_x.clone(), v_y.clone(), v_weights.clone())?).items()?;
    if t5144.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t5144[0].clone();
    v_ys = t5144[1].clone();
    v_w = t5144[2].clone();
    let _ = f__need({ let t5145 = py::len(&v_xs.clone())?; Py::Bool({ let t5146 = Py::Int(2i64); if py::ge(&t5145, &t5146)? { true } else { false } }) }, Py::str("at least two points are needed"))?;
    for t5147 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_i = t5147;
        for t5148 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t5148;
            let _ = f__need({ let t5153 = { let t5149 = py::getitem(&v_xs.clone(), &v_i.clone())?; Py::Bool({ let t5150 = py::getitem(&v_xs.clone(), &v_j.clone())?; if py::ne(&t5149, &t5150) { true } else { false } }) }; if t5153.truthy() { t5153 } else { { let t5151 = py::getitem(&v_ys.clone(), &v_i.clone())?; Py::Bool({ let t5152 = py::getitem(&v_ys.clone(), &v_j.clone())?; if py::ne(&t5151, &t5152) { true } else { false } }) } } }, Py::str("the points must be distinct"))?;
        }
    }
    v_best = Py::None;
    for t5154 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_i = t5154;
        if (f__weber_vertex(v_xs.clone(), v_ys.clone(), v_w.clone(), v_i.clone())?).truthy() {
            v_best = Py::tuple(vec![py::getitem(&v_xs.clone(), &v_i.clone())?, py::getitem(&v_ys.clone(), &v_i.clone())?]);
        }
    }
    if ({ let t5163 = v_best.clone(); Py::Bool({ let t5164 = Py::None; if py::is(&t5163, &t5164) { true } else { false } }) }).truthy() {
        v_W = py::sum(&v_w.clone(), None)?;
        v_px = py::truediv(&py::sum(&{ let mut out = vec![]; for t5165 in py::iter(&(py::zip(&[v_xs.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5166 = (t5165).items()?;
if t5166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5166[0].clone();
v_b = t5166[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
        v_py = py::truediv(&py::sum(&{ let mut out = vec![]; for t5167 in py::iter(&(py::zip(&[v_ys.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5168 = (t5167).items()?;
if t5168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5168[0].clone();
v_b = t5168[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
        v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t5169 in py::iter(&(py::add(&v_xs.clone(), &v_ys.clone())?))? { let mut v_v: Py; 
v_v = t5169;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?;
        for t5170 in py::range_iter(&[(Py::Int(200000i64)).i()?]) {
            v__ = t5170;
            let t5171 = Py::Float(0.0);
            v_nx = t5171.clone();
            v_ny = t5171.clone();
            v_den = t5171.clone();
            for t5172 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
                v_k = t5172;
                v_d = py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_xs.clone(), &v_k.clone())?)?, py::sub(&v_py.clone(), &py::getitem(&v_ys.clone(), &v_k.clone())?)?])?;
                let _ = f__need({ let t5173 = v_d.clone(); Py::Bool({ let t5174 = Py::Float(1e-300); if py::gt(&t5173, &t5174)? { true } else { false } }) }, Py::str("the iteration reached a demand point"))?;
                v_nx = py::iadd(&v_nx, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::getitem(&v_xs.clone(), &v_k.clone())?)?, &v_d.clone())?)?;
                v_ny = py::iadd(&v_ny, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::getitem(&v_ys.clone(), &v_k.clone())?)?, &v_d.clone())?)?;
                v_den = py::iadd(&v_den, &py::truediv(&py::getitem(&v_w.clone(), &v_k.clone())?, &v_d.clone())?)?;
            }
            let t5175 = (Py::tuple(vec![py::truediv(&v_nx.clone(), &v_den.clone())?, py::truediv(&v_ny.clone(), &v_den.clone())?])).items()?;
            if t5175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_nx = t5175[0].clone();
            v_ny = t5175[1].clone();
            v_step = py::hypot(&[py::sub(&v_nx.clone(), &v_px.clone())?, py::sub(&v_ny.clone(), &v_py.clone())?])?;
            let t5176 = (Py::tuple(vec![v_nx.clone(), v_ny.clone()])).items()?;
            if t5176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_px = t5176[0].clone();
            v_py = t5176[1].clone();
            if ({ let t5177 = v_step.clone(); Py::Bool({ let t5178 = py::mul(&Py::Float(1e-15), &v_scale.clone())?; if py::le(&t5177, &t5178)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_best = Py::tuple(vec![v_px.clone(), v_py.clone()]);
    }
    v_cost = py::sum(&{ let mut out = vec![]; for t5179 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t5179;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::hypot(&[py::sub(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &py::getitem(&v_xs.clone(), &v_k.clone())?)?, py::sub(&py::getitem(&v_best.clone(), &Py::Int(1i64))?, &py::getitem(&v_ys.clone(), &v_k.clone())?)?])?)?); } Py::list(out) }, None)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("x"), py::getitem(&v_best.clone(), &Py::Int(0i64))?), (Py::str("y"), py::getitem(&v_best.clone(), &Py::Int(1i64))?), (Py::str("total_weighted_distance"), v_cost.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_regression(mut v_failure_times: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    v_t = f__vec(Py::str("failure_times"), v_failure_times.clone(), Py::Int(3i64), Py::Int(1000i64), Py::Bool(true), Py::Bool(false))?;
    v_t = py::sorted(&v_t.clone(), (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_t.clone())?;
    let t5233 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t5233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t5233[0].clone();
    v_ys = t5233[1].clone();
    for t5234 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5234;
        v_F = py::truediv(&py::sub(&py::add(&v_i.clone(), &Py::Int(1i64))?, &Py::Float(0.3))?, &py::add(&v_n.clone(), &Py::Float(0.4))?)?;
        let _ = py::method(&v_xs.clone(), "append", &[py::math1("log", &py::getitem(&v_t.clone(), &v_i.clone())?)?])?;
        let _ = py::method(&v_ys.clone(), "append", &[py::math1("log", &py::neg(&py::math1("log", &py::sub(&Py::Int(1i64), &v_F.clone())?)?)?)?])?;
    }
    let t5235 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t5235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t5235[0].clone();
    v_my = t5235[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t5236 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t5236;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t5237 = v_sxx.clone(); Py::Bool({ let t5238 = Py::Int(0i64); if py::gt(&t5237, &t5238)? { true } else { false } }) }, Py::str("the failure times must not all be equal"))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t5239 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5240 = (t5239).items()?;
if t5240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5240[0].clone();
v_y = t5240[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t5241 in py::iter(&(v_ys.clone()))? { let mut v_y: Py; 
v_y = t5241;
out.push(py::pow(&py::sub(&v_y.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("shape"), v_b.clone()), (Py::str("scale"), py::math1("exp", &py::truediv(&py::neg(&v_a.clone())?, &v_b.clone())?)?), (Py::str("r_squared"), py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_reliability(mut v_shape: Py, mut v_scale: Py, mut v_time: Py, mut v_location: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t5281 = (Py::tuple(vec![f__pos(Py::str("shape"), v_shape.clone())?, f__pos(Py::str("scale"), v_scale.clone())?])).items()?;
    if t5281.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t5281[0].clone();
    v_eta = t5281[1].clone();
    v_g = f__nonneg(Py::str("location"), v_location.clone())?;
    v_t = f__fin(Py::str("time"), v_time.clone())?;
    let _ = f__need({ let t5284 = v_t.clone(); Py::Bool({ let t5285 = v_g.clone(); if py::ge(&t5284, &t5285)? { true } else { false } }) }, Py::str("time must not be before the location (failure-free time)"))?;
    v_x = py::truediv(&py::sub(&v_t.clone(), &v_g.clone())?, &v_eta.clone())?;
    v_R = py::math1("exp", &py::neg(&py::pow(&v_x.clone(), &v_b.clone())?)?)?;
    v_h = (if ({ let t5286 = v_x.clone(); Py::Bool({ let t5287 = Py::Int(0i64); if py::gt(&t5286, &t5287)? { true } else { false } }) }).truthy() { py::mul(&py::truediv(&v_b.clone(), &v_eta.clone())?, &py::pow(&v_x.clone(), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)? } else { (if ({ let t5288 = v_b.clone(); Py::Bool({ let t5289 = Py::Int(1i64); if py::gt(&t5288, &t5289)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t5290 = v_b.clone(); Py::Bool({ let t5291 = Py::Int(1i64); if py::eq(&t5290, &t5291) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &v_eta.clone())? } else { Py::Float(f64::INFINITY) }) }) });
    let _ = f__need(py::math1("isfinite", &v_h.clone())?, Py::str("the hazard rate is infinite at the location when shape < 1"))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("reliability"), v_R.clone()), (Py::str("unreliability"), py::neg(&py::math1("expm1", &py::neg(&py::pow(&v_x.clone(), &v_b.clone())?)?)?)?), (Py::str("hazard_rate"), v_h.clone()), (Py::str("mttf"), py::add(&v_g.clone(), &py::mul(&v_eta.clone(), &py::math1("gamma", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?)?)?), (Py::str("b10_life"), py::add(&v_g.clone(), &py::mul(&v_eta.clone(), &py::pow(&py::neg(&py::math1("log", &Py::Float(0.9))?)?, &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weighted_moving_average(mut v_data: Py, mut v_weights: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_tw: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("data"), v_data.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?;
    v_w = f__vec(Py::str("weights"), v_weights.clone(), Py::Int(1i64), Py::Int(1000i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t5345 = py::len(&v_w.clone())?; Py::Bool({ let t5346 = py::len(&v_x.clone())?; if py::le(&t5345, &t5346)? { true } else { false } }) }, Py::str("there must be at least as many observations as weights"))?;
    v_tw = py::sum(&v_w.clone(), None)?;
    let _ = f__need({ let t5347 = v_tw.clone(); Py::Bool({ let t5348 = Py::Int(0i64); if py::gt(&t5347, &t5348)? { true } else { false } }) }, Py::str("the weights must not all be 0"))?;
    v_k = py::len(&v_w.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("forecast"), py::truediv(&py::sum(&{ let mut out = vec![]; for t5361 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t5361;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &py::add(&py::sub(&py::len(&v_x.clone())?, &v_k.clone())?, &v_i.clone())?)?)?); } Py::list(out) }, None)?, &v_tw.clone())?), (Py::str("weight_sum"), v_tw.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_work_sampling_size(mut v_proportion: Py, mut v_accuracy: Py, mut v_confidence: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t5390 = (Py::tuple(vec![f__prob(Py::str("proportion"), v_proportion.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("accuracy"), v_accuracy.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("confidence"), v_confidence.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t5390.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5390[0].clone();
    v_h = t5390[1].clone();
    v_c = t5390[2].clone();
    v_z = f__zinv(py::sub(&Py::Int(1i64), &py::truediv(&py::sub(&Py::Int(1i64), &v_c.clone())?, &Py::Int(2i64))?)?)?;
    v_x = py::truediv(&py::mul(&py::mul(&py::mul(&v_z.clone(), &v_z.clone())?, &v_p.clone())?, &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &py::mul(&v_h.clone(), &v_h.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sample_size"), f__ceil(v_x.clone())?), (Py::str("exact"), v_x.clone()), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wright_average(mut v_first_unit_time: Py, mut v_learning_rate: Py, mut v_units: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_T1: Py = Py::None;
    let mut v_avg: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let t1865 = (Py::tuple(vec![f__pos(Py::str("first_unit_time"), v_first_unit_time.clone())?, f__prob(Py::str("learning_rate"), v_learning_rate.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t1865.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t1865[0].clone();
    v_r = t1865[1].clone();
    v_N = f__int(Py::str("units"), v_units.clone(), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    v_b = f__b(v_r.clone())?;
    v_avg = py::mul(&v_T1.clone(), &py::pow(&v_N.clone(), &v_b.clone())?)?;
    v_tot = py::mul(&v_avg.clone(), &v_N.clone())?;
    v_prev = (if ({ let t1878 = v_N.clone(); Py::Bool({ let t1879 = Py::Int(1i64); if py::gt(&t1878, &t1879)? { true } else { false } }) }).truthy() { py::mul(&v_T1.clone(), &py::pow(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &py::add(&v_b.clone(), &Py::Int(1i64))?)?)? } else { Py::Float(0.0) });
    return Ok(f__rd(Py::dict(vec![(Py::str("cumulative_average_time"), v_avg.clone()), (Py::str("total_time"), v_tot.clone()), (Py::str("unit_time"), py::sub(&v_tot.clone(), &v_prev.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_xbar_r_limits(mut v_grand_mean: Py, mut v_mean_range: Py, mut v_subgroup_size: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let t5432 = (Py::tuple(vec![f__fin(Py::str("grand_mean"), v_grand_mean.clone())?, f__pos(Py::str("mean_range"), v_mean_range.clone())?])).items()?;
    if t5432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t5432[0].clone();
    v_rb = t5432[1].clone();
    v_n = f__int(Py::str("subgroup_size"), v_subgroup_size.clone(), Py::Int(2i64), Py::Int(10i64))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("xbar_ucl"), py::add(&v_xb.clone(), &py::mul(&py::getitem(&c__A2()?, &v_n.clone())?, &v_rb.clone())?)?), (Py::str("xbar_lcl"), py::sub(&v_xb.clone(), &py::mul(&py::getitem(&c__A2()?, &v_n.clone())?, &v_rb.clone())?)?), (Py::str("center"), v_xb.clone()), (Py::str("r_ucl"), py::mul(&py::getitem(&c__D4()?, &v_n.clone())?, &v_rb.clone())?), (Py::str("r_lcl"), py::mul(&py::getitem(&c__D3()?, &v_n.clone())?, &v_rb.clone())?), (Py::str("A2"), py::getitem(&c__A2()?, &v_n.clone())?), (Py::str("D3"), py::getitem(&c__D3()?, &v_n.clone())?), (Py::str("D4"), py::getitem(&c__D4()?, &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_xbar_s_limits(mut v_grand_mean: Py, mut v_mean_sd: Py, mut v_subgroup_size: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_sb: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let t5484 = (Py::tuple(vec![f__fin(Py::str("grand_mean"), v_grand_mean.clone())?, f__pos(Py::str("mean_sd"), v_mean_sd.clone())?])).items()?;
    if t5484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t5484[0].clone();
    v_sb = t5484[1].clone();
    v_n = f__int(Py::str("subgroup_size"), v_subgroup_size.clone(), Py::Int(2i64), Py::Int(10i64))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("xbar_ucl"), py::add(&v_xb.clone(), &py::mul(&py::getitem(&c__A3()?, &v_n.clone())?, &v_sb.clone())?)?), (Py::str("xbar_lcl"), py::sub(&v_xb.clone(), &py::mul(&py::getitem(&c__A3()?, &v_n.clone())?, &v_sb.clone())?)?), (Py::str("center"), v_xb.clone()), (Py::str("s_ucl"), py::mul(&py::getitem(&c__B4()?, &v_n.clone())?, &v_sb.clone())?), (Py::str("s_lcl"), py::mul(&py::getitem(&c__B3()?, &v_n.clone())?, &v_sb.clone())?), (Py::str("A3"), py::getitem(&c__A3()?, &v_n.clone())?), (Py::str("B3"), py::getitem(&c__B3()?, &v_n.clone())?), (Py::str("B4"), py::getitem(&c__B4()?, &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1048(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1070 = v_inputs.clone(); f_exp_smoothing(py::getitem(&t1070, &Py::str("data"))?, py::getitem(&t1070, &Py::str("alpha"))?, py::method(&t1070, "get", &[Py::str("initial_forecast"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_110(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t132 = v_inputs.clone(); f_acceptance_sampling(py::getitem(&t132, &Py::str("sample_size"))?, py::getitem(&t132, &Py::str("acceptance_number"))?, py::getitem(&t132, &Py::str("fraction_defective"))?, py::getitem(&t132, &Py::str("lot_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1127(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1149 = v_inputs.clone(); f_factor_rating(py::getitem(&t1149, &Py::str("weights"))?, py::getitem(&t1149, &Py::str("scores"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1195(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1217 = v_inputs.clone(); f_flow_shop_makespan(py::getitem(&t1217, &Py::str("times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1282 = v_inputs.clone(); f_forecast_errors(py::getitem(&t1282, &Py::str("actual"))?, py::getitem(&t1282, &Py::str("forecast"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1316(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1338 = v_inputs.clone(); f_fraction_nonconforming(py::getitem(&t1338, &Py::str("mean"))?, py::getitem(&t1338, &Py::str("sd"))?, py::method(&t1338, "get", &[Py::str("lsl"), Py::None])?, py::method(&t1338, "get", &[Py::str("usl"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1387(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1409 = v_inputs.clone(); f_holt(py::getitem(&t1409, &Py::str("data"))?, py::getitem(&t1409, &Py::str("alpha"))?, py::getitem(&t1409, &Py::str("beta"))?, py::getitem(&t1409, &Py::str("initial_level"))?, py::getitem(&t1409, &Py::str("initial_trend"))?, py::method(&t1409, "get", &[Py::str("horizon"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1431(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1453 = v_inputs.clone(); f_inventory_total_cost(py::getitem(&t1453, &Py::str("demand"))?, py::getitem(&t1453, &Py::str("order_cost"))?, py::getitem(&t1453, &Py::str("holding_cost"))?, py::getitem(&t1453, &Py::str("order_quantity"))?, py::method(&t1453, "get", &[Py::str("unit_cost"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1471(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1493 = v_inputs.clone(); f_inventory_turnover(py::getitem(&t1493, &Py::str("cost_of_goods_sold"))?, py::getitem(&t1493, &Py::str("average_inventory"))?, py::method(&t1493, "get", &[Py::str("days_per_year"), Py::Int(365i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1543(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1565 = v_inputs.clone(); f_johnson_three(py::getitem(&t1565, &Py::str("machine1_times"))?, py::getitem(&t1565, &Py::str("machine2_times"))?, py::getitem(&t1565, &Py::str("machine3_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1607(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1629 = v_inputs.clone(); f_johnson_two(py::getitem(&t1629, &Py::str("machine1_times"))?, py::getitem(&t1629, &Py::str("machine2_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_165(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t187 = v_inputs.clone(); f_arl_shewhart(py::getitem(&t187, &Py::str("shift"))?, py::method(&t187, "get", &[Py::str("subgroup_size"), Py::Int(1i64)])?, py::method(&t187, "get", &[Py::str("limit_sigma"), Py::Float(3.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1663(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1685 = v_inputs.clone(); f_k_out_of_n(py::getitem(&t1685, &Py::str("component_reliability"))?, py::getitem(&t1685, &Py::str("k"))?, py::getitem(&t1685, &Py::str("n"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1708(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1730 = v_inputs.clone(); f_kanban_count(py::getitem(&t1730, &Py::str("demand_rate"))?, py::getitem(&t1730, &Py::str("lead_time"))?, py::getitem(&t1730, &Py::str("container_size"))?, py::method(&t1730, "get", &[Py::str("safety_factor"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1835(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1857 = v_inputs.clone(); f_largest_candidate(py::getitem(&t1857, &Py::str("task_times"))?, py::getitem(&t1857, &Py::str("precedence"))?, py::getitem(&t1857, &Py::str("cycle_time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1914 = v_inputs.clone(); f_wright_average(py::getitem(&t1914, &Py::str("first_unit_time"))?, py::getitem(&t1914, &Py::str("learning_rate"))?, py::getitem(&t1914, &Py::str("units"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1959(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1981 = v_inputs.clone(); f_learning_fit(py::getitem(&t1981, &Py::str("units"))?, py::getitem(&t1981, &Py::str("times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2015(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2037 = v_inputs.clone(); f_learning_cumulative(py::getitem(&t2037, &Py::str("first_unit_time"))?, py::getitem(&t2037, &Py::str("learning_rate"))?, py::getitem(&t2037, &Py::str("units"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_205(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t227 = v_inputs.clone(); f_availability(py::getitem(&t227, &Py::str("mtbf"))?, py::getitem(&t227, &Py::str("mttr"))?, py::method(&t227, "get", &[Py::str("hours_per_year"), Py::Float(8760.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2070(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2092 = v_inputs.clone(); f_learning_unit(py::getitem(&t2092, &Py::str("first_unit_time"))?, py::getitem(&t2092, &Py::str("learning_rate"))?, py::getitem(&t2092, &Py::str("unit"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2135 = v_inputs.clone(); f_learning_two_points(py::getitem(&t2135, &Py::str("unit1"))?, py::getitem(&t2135, &Py::str("time1"))?, py::getitem(&t2135, &Py::str("unit2"))?, py::getitem(&t2135, &Py::str("time2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2187(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2209 = v_inputs.clone(); f_line_efficiency(py::getitem(&t2209, &Py::str("task_times"))?, py::getitem(&t2209, &Py::str("cycle_time"))?, py::getitem(&t2209, &Py::str("stations"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2259(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2281 = v_inputs.clone(); f_linear_trend(py::getitem(&t2281, &Py::str("data"))?, py::method(&t2281, "get", &[Py::str("horizon"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2312(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2334 = v_inputs.clone(); f_littles_law(py::method(&t2334, "get", &[Py::str("arrival_rate"), Py::None])?, py::method(&t2334, "get", &[Py::str("number_in_system"), Py::None])?, py::method(&t2334, "get", &[Py::str("time_in_system"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2373(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2395 = v_inputs.clone(); f_location_cost_volume(py::getitem(&t2395, &Py::str("fixed_costs"))?, py::getitem(&t2395, &Py::str("variable_costs"))?, py::getitem(&t2395, &Py::str("volume"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2417(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2439 = v_inputs.clone(); f_md1(py::getitem(&t2439, &Py::str("arrival_rate"))?, py::getitem(&t2439, &Py::str("service_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2463(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2485 = v_inputs.clone(); f_mg1(py::getitem(&t2485, &Py::str("arrival_rate"))?, py::getitem(&t2485, &Py::str("mean_service_time"))?, py::getitem(&t2485, &Py::str("sd_service_time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2505(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2527 = v_inputs.clone(); f_mm1(py::getitem(&t2527, &Py::str("arrival_rate"))?, py::getitem(&t2527, &Py::str("service_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_251(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t273 = v_inputs.clone(); f_break_even(py::getitem(&t273, &Py::str("fixed_cost"))?, py::getitem(&t273, &Py::str("price"))?, py::getitem(&t273, &Py::str("variable_cost"))?, py::method(&t273, "get", &[Py::str("target_profit"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2562(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2584 = v_inputs.clone(); f_mm1k(py::getitem(&t2584, &Py::str("arrival_rate"))?, py::getitem(&t2584, &Py::str("service_rate"))?, py::getitem(&t2584, &Py::str("capacity"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2618(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2640 = v_inputs.clone(); f_mmc(py::getitem(&t2640, &Py::str("arrival_rate"))?, py::getitem(&t2640, &Py::str("service_rate"))?, py::getitem(&t2640, &Py::str("servers"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2693(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2715 = v_inputs.clone(); f_moore_hodgson(py::getitem(&t2715, &Py::str("processing_times"))?, py::getitem(&t2715, &Py::str("due_dates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2761(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2783 = v_inputs.clone(); f_moving_average(py::getitem(&t2783, &Py::str("data"))?, py::getitem(&t2783, &Py::str("periods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2852(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2874 = v_inputs.clone(); f_mtbf_confidence(py::getitem(&t2874, &Py::str("total_time"))?, py::getitem(&t2874, &Py::str("failures"))?, py::getitem(&t2874, &Py::str("confidence"))?, py::method(&t2874, "get", &[Py::str("test"), Py::str("time")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_290(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t312 = v_inputs.clone(); f_c_chart_limits(py::getitem(&t312, &Py::str("c_bar"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2917(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2939 = v_inputs.clone(); f_multiproduct_break_even(py::getitem(&t2939, &Py::str("fixed_cost"))?, py::getitem(&t2939, &Py::str("prices"))?, py::getitem(&t2939, &Py::str("variable_costs"))?, py::getitem(&t2939, &Py::str("sales_mix"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2997(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3019 = v_inputs.clone(); f_newsvendor_discrete(py::getitem(&t3019, &Py::str("price"))?, py::getitem(&t3019, &Py::str("cost"))?, py::getitem(&t3019, &Py::str("demands"))?, py::getitem(&t3019, &Py::str("probabilities"))?, py::method(&t3019, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3042(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3064 = v_inputs.clone(); f_newsvendor_normal(py::getitem(&t3064, &Py::str("price"))?, py::getitem(&t3064, &Py::str("cost"))?, py::getitem(&t3064, &Py::str("mean_demand"))?, py::getitem(&t3064, &Py::str("sd_demand"))?, py::method(&t3064, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3096(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3118 = v_inputs.clone(); f_newsvendor_poisson(py::getitem(&t3118, &Py::str("price"))?, py::getitem(&t3118, &Py::str("cost"))?, py::getitem(&t3118, &Py::str("mean_demand"))?, py::method(&t3118, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3165 = v_inputs.clone(); f_newsvendor_uniform(py::getitem(&t3165, &Py::str("price"))?, py::getitem(&t3165, &Py::str("cost"))?, py::getitem(&t3165, &Py::str("low"))?, py::getitem(&t3165, &Py::str("high"))?, py::method(&t3165, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3196(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3218 = v_inputs.clone(); f_np_chart_limits(py::getitem(&t3218, &Py::str("p_bar"))?, py::getitem(&t3218, &Py::str("sample_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3268 = v_inputs.clone(); f_oee(py::getitem(&t3268, &Py::str("planned_time"))?, py::getitem(&t3268, &Py::str("downtime"))?, py::getitem(&t3268, &Py::str("ideal_cycle_time"))?, py::getitem(&t3268, &Py::str("total_count"))?, py::getitem(&t3268, &Py::str("good_count"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3294(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3316 = v_inputs.clone(); f_order_up_to(py::getitem(&t3316, &Py::str("mean_demand"))?, py::getitem(&t3316, &Py::str("sd_demand"))?, py::getitem(&t3316, &Py::str("review_period"))?, py::getitem(&t3316, &Py::str("lead_time"))?, py::getitem(&t3316, &Py::str("service_level"))?, py::method(&t3316, "get", &[Py::str("inventory_position"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_334(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t356 = v_inputs.clone(); f_capacity_measures(py::getitem(&t356, &Py::str("design_capacity"))?, py::getitem(&t356, &Py::str("effective_capacity"))?, py::getitem(&t356, &Py::str("actual_output"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3375(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3397 = v_inputs.clone(); f_p_chart_counts(py::getitem(&t3397, &Py::str("defectives"))?, py::getitem(&t3397, &Py::str("sample_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3428(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3450 = v_inputs.clone(); f_p_chart_limits(py::getitem(&t3450, &Py::str("p_bar"))?, py::getitem(&t3450, &Py::str("sample_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3481(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3503 = v_inputs.clone(); f_parallel_mttf(py::getitem(&t3503, &Py::str("failure_rate"))?, py::getitem(&t3503, &Py::str("n"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3539(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3561 = v_inputs.clone(); f_parallel_reliability(py::getitem(&t3561, &Py::str("reliabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3604 = v_inputs.clone(); f_capability(py::getitem(&t3604, &Py::str("usl"))?, py::getitem(&t3604, &Py::str("lsl"))?, py::getitem(&t3604, &Py::str("mean"))?, py::getitem(&t3604, &Py::str("sd"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3642(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3664 = v_inputs.clone(); f_performance(py::getitem(&t3664, &Py::str("data"))?, py::getitem(&t3664, &Py::str("usl"))?, py::getitem(&t3664, &Py::str("lsl"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3708(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3730 = v_inputs.clone(); f_server_cost(py::getitem(&t3730, &Py::str("arrival_rate"))?, py::getitem(&t3730, &Py::str("service_rate"))?, py::getitem(&t3730, &Py::str("server_cost_rate"))?, py::getitem(&t3730, &Py::str("waiting_cost_rate"))?, py::method(&t3730, "get", &[Py::str("max_servers"), Py::Int(50i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3843(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3865 = v_inputs.clone(); f_ranked_positional_weight(py::getitem(&t3865, &Py::str("task_times"))?, py::getitem(&t3865, &Py::str("precedence"))?, py::getitem(&t3865, &Py::str("cycle_time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3915(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3937 = v_inputs.clone(); f_rectilinear_location(py::getitem(&t3937, &Py::str("x"))?, py::getitem(&t3937, &Py::str("y"))?, py::getitem(&t3937, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3960(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3982 = v_inputs.clone(); f_redundancy_needed(py::getitem(&t3982, &Py::str("target_reliability"))?, py::getitem(&t3982, &Py::str("component_reliability"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_398(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t420 = v_inputs.clone(); f_center_of_gravity(py::getitem(&t420, &Py::str("x"))?, py::getitem(&t420, &Py::str("y"))?, py::getitem(&t420, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4029(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4051 = v_inputs.clone(); f_regression_forecast(py::getitem(&t4051, &Py::str("x"))?, py::getitem(&t4051, &Py::str("y"))?, py::method(&t4051, "get", &[Py::str("x_new"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4071(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4093 = v_inputs.clone(); f_reorder_point(py::getitem(&t4093, &Py::str("demand_rate"))?, py::getitem(&t4093, &Py::str("lead_time"))?, py::method(&t4093, "get", &[Py::str("safety_stock"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4129(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4151 = v_inputs.clone(); f_rolled_yield(py::getitem(&t4151, &Py::str("yields"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4203(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4225 = v_inputs.clone(); f_rq_fill_rate(py::getitem(&t4225, &Py::str("mean_ltd"))?, py::getitem(&t4225, &Py::str("sd_ltd"))?, py::getitem(&t4225, &Py::str("order_quantity"))?, py::getitem(&t4225, &Py::str("fill_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4247(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4269 = v_inputs.clone(); f_safety_stock_normal(py::getitem(&t4269, &Py::str("mean_demand"))?, py::getitem(&t4269, &Py::str("sd_demand"))?, py::getitem(&t4269, &Py::str("lead_time"))?, py::getitem(&t4269, &Py::str("service_level"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4298(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4320 = v_inputs.clone(); f_safety_stock_vlt(py::getitem(&t4320, &Py::str("mean_demand"))?, py::getitem(&t4320, &Py::str("sd_demand"))?, py::getitem(&t4320, &Py::str("mean_lead_time"))?, py::getitem(&t4320, &Py::str("sd_lead_time"))?, py::getitem(&t4320, &Py::str("service_level"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4367(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4389 = v_inputs.clone(); f_seasonal_index(py::getitem(&t4389, &Py::str("data"))?, py::method(&t4389, "get", &[Py::str("next_year_total"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_443(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t465 = v_inputs.clone(); f_cpm_index(py::getitem(&t465, &Py::str("usl"))?, py::getitem(&t465, &Py::str("lsl"))?, py::getitem(&t465, &Py::str("mean"))?, py::getitem(&t465, &Py::str("sd"))?, py::getitem(&t465, &Py::str("target"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4430(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4452 = v_inputs.clone(); f_series_parallel(py::getitem(&t4452, &Py::str("stage_reliabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4488(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4510 = v_inputs.clone(); f_series_reliability(py::getitem(&t4510, &Py::str("reliabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4530(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4552 = v_inputs.clone(); f_service_from_rop(py::getitem(&t4552, &Py::str("mean_ltd"))?, py::getitem(&t4552, &Py::str("sd_ltd"))?, py::getitem(&t4552, &Py::str("reorder_point"))?, py::method(&t4552, "get", &[Py::str("order_quantity"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4604 = v_inputs.clone(); f_sigma_from_range(py::getitem(&t4604, &Py::str("mean_range"))?, py::getitem(&t4604, &Py::str("subgroup_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4650(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4672 = v_inputs.clone(); f_spt_sequence(py::getitem(&t4672, &Py::str("processing_times"))?, py::method(&t4672, "get", &[Py::str("due_dates"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4713(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4735 = v_inputs.clone(); f_standard_time(py::getitem(&t4735, &Py::str("observed_times"))?, py::getitem(&t4735, &Py::str("rating"))?, py::getitem(&t4735, &Py::str("allowance"))?, py::getitem(&t4735, &Py::str("basis"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4769(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4791 = v_inputs.clone(); f_standby_redundancy(py::getitem(&t4791, &Py::str("failure_rate"))?, py::getitem(&t4791, &Py::str("time"))?, py::method(&t4791, "get", &[Py::str("units"), Py::Int(2i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4826(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4848 = v_inputs.clone(); f_system_failure_rate(py::getitem(&t4848, &Py::str("failure_rates"))?, py::method(&t4848, "get", &[Py::str("time"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4895(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4917 = v_inputs.clone(); f_tabular_cusum(py::getitem(&t4917, &Py::str("data"))?, py::getitem(&t4917, &Py::str("target"))?, py::getitem(&t4917, &Py::str("k"))?, py::getitem(&t4917, &Py::str("h"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4939(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4961 = v_inputs.clone(); f_takt_time(py::getitem(&t4961, &Py::str("available_time"))?, py::getitem(&t4961, &Py::str("demand"))?, py::method(&t4961, "get", &[Py::str("total_task_time"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5007 = v_inputs.clone(); f_time_study_size(py::getitem(&t5007, &Py::str("mean"))?, py::getitem(&t5007, &Py::str("sd"))?, py::getitem(&t5007, &Py::str("accuracy"))?, py::getitem(&t5007, &Py::str("confidence"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5079 = v_inputs.clone(); f_tracking_signal(py::getitem(&t5079, &Py::str("actual"))?, py::getitem(&t5079, &Py::str("forecast"))?, py::method(&t5079, "get", &[Py::str("limit"), Py::Float(4.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5097(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5119 = v_inputs.clone(); f_u_chart_limits(py::getitem(&t5119, &Py::str("u_bar"))?, py::getitem(&t5119, &Py::str("units"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_51(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t73 = v_inputs.clone(); f_abc_classification(py::getitem(&t73, &Py::str("annual_values"))?, py::method(&t73, "get", &[Py::str("a_share"), Py::Float(0.8)])?, py::method(&t73, "get", &[Py::str("b_share"), Py::Float(0.95)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5192(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5214 = v_inputs.clone(); f_weber_location(py::getitem(&t5214, &Py::str("x"))?, py::getitem(&t5214, &Py::str("y"))?, py::getitem(&t5214, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_521(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t543 = v_inputs.clone(); f_dispatch_rule(py::getitem(&t543, &Py::str("processing_times"))?, py::getitem(&t543, &Py::str("due_dates"))?, py::getitem(&t543, &Py::str("rule"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5254(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5276 = v_inputs.clone(); f_weibull_regression(py::getitem(&t5276, &Py::str("failure_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5304(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5326 = v_inputs.clone(); f_weibull_reliability(py::getitem(&t5326, &Py::str("shape"))?, py::getitem(&t5326, &Py::str("scale"))?, py::getitem(&t5326, &Py::str("time"))?, py::method(&t5326, "get", &[Py::str("location"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5362(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5384 = v_inputs.clone(); f_weighted_moving_average(py::getitem(&t5384, &Py::str("data"))?, py::getitem(&t5384, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5427 = v_inputs.clone(); f_work_sampling_size(py::getitem(&t5427, &Py::str("proportion"))?, py::getitem(&t5427, &Py::str("accuracy"))?, py::getitem(&t5427, &Py::str("confidence"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5479 = v_inputs.clone(); f_xbar_r_limits(py::getitem(&t5479, &Py::str("grand_mean"))?, py::getitem(&t5479, &Py::str("mean_range"))?, py::getitem(&t5479, &Py::str("subgroup_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5509(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5531 = v_inputs.clone(); f_xbar_s_limits(py::getitem(&t5531, &Py::str("grand_mean"))?, py::getitem(&t5531, &Py::str("mean_sd"))?, py::getitem(&t5531, &Py::str("subgroup_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_566(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t588 = v_inputs.clone(); f_dpmo_sigma(py::getitem(&t588, &Py::str("defects"))?, py::getitem(&t588, &Py::str("units"))?, py::getitem(&t588, &Py::str("opportunities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_634(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t656 = v_inputs.clone(); f_edd_sequence(py::getitem(&t656, &Py::str("processing_times"))?, py::getitem(&t656, &Py::str("due_dates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_674(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t696 = v_inputs.clone(); f_eoq(py::getitem(&t696, &Py::str("demand"))?, py::getitem(&t696, &Py::str("order_cost"))?, py::getitem(&t696, &Py::str("holding_cost"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_714(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t736 = v_inputs.clone(); f_eoq_backorders(py::getitem(&t736, &Py::str("demand"))?, py::getitem(&t736, &Py::str("order_cost"))?, py::getitem(&t736, &Py::str("holding_cost"))?, py::getitem(&t736, &Py::str("backorder_cost"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_796(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t818 = v_inputs.clone(); f_eoq_discount(py::getitem(&t818, &Py::str("demand"))?, py::getitem(&t818, &Py::str("order_cost"))?, py::getitem(&t818, &Py::str("holding_rate"))?, py::getitem(&t818, &Py::str("break_quantities"))?, py::getitem(&t818, &Py::str("unit_prices"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_839(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t861 = v_inputs.clone(); f_epq(py::getitem(&t861, &Py::str("demand"))?, py::getitem(&t861, &Py::str("production_rate"))?, py::getitem(&t861, &Py::str("order_cost"))?, py::getitem(&t861, &Py::str("holding_cost"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_891(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t913 = v_inputs.clone(); f_staffing(py::getitem(&t913, &Py::str("arrival_rate"))?, py::getitem(&t913, &Py::str("service_rate"))?, py::getitem(&t913, &Py::str("target_time"))?, py::getitem(&t913, &Py::str("target_level"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t969 = v_inputs.clone(); f_ewma_limits(py::getitem(&t969, &Py::str("target"))?, py::getitem(&t969, &Py::str("sigma"))?, py::getitem(&t969, &Py::str("smoothing"))?, py::getitem(&t969, &Py::str("limit_sigma"))?, py::getitem(&t969, &Py::str("sample_number"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_989(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1011 = v_inputs.clone(); f_exponential_reliability(py::getitem(&t1011, &Py::str("failure_rate"))?, py::getitem(&t1011, &Py::str("time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
