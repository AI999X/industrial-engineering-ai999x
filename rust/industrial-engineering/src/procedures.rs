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
    "ie_after_tax_cash_flow",
    "ie_arl_shewhart",
    "ie_availability",
    "ie_break_even",
    "ie_c_chart_limits",
    "ie_capacity_utilization",
    "ie_center_of_gravity",
    "ie_cpm_taguchi",
    "ie_depreciation_schedule",
    "ie_dispatching_rule",
    "ie_dpmo_sigma_level",
    "ie_earned_value",
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
    "ie_productivity",
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
        "ie_after_tax_cash_flow" => {
            let r = f_after_tax_cash_flow(match v.get("before_tax_cash_flows") { Some(x) => py::from_val(x), None => { return None } }, match v.get("depreciation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("investment") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("sale_price") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_191(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_arl_shewhart" => {
            let r = f_arl_shewhart(match v.get("shift") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("limit_sigma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_availability" => {
            let r = f_availability(match v.get("mtbf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mttr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hours_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(8760.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_break_even" => {
            let r = f_break_even(match v.get("fixed_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_profit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_332(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_c_chart_limits" => {
            let r = f_c_chart_limits(match v.get("c_bar") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_371(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_capacity_utilization" => {
            let r = f_capacity_measures(match v.get("design_capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("effective_capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_output") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_415(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_center_of_gravity" => {
            let r = f_center_of_gravity(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_479(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_cpm_taguchi" => {
            let r = f_cpm_index(match v.get("usl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_524(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_depreciation_schedule" => {
            let r = f_depreciation_schedule(match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("life") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("factor") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_610(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_dispatching_rule" => {
            let r = f_dispatch_rule(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rule") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_688(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_dpmo_sigma_level" => {
            let r = f_dpmo_sigma(match v.get("defects") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("opportunities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_733(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_earned_value" => {
            let r = f_earned_value_analysis(match v.get("planned_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("earned_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("budget_at_completion") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_edd_sequence" => {
            let r = f_edd_sequence(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_852(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_eoq" => {
            let r = f_eoq(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_eoq_backorders" => {
            let r = f_eoq_backorders(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("backorder_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_932(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_eoq_quantity_discount" => {
            let r = f_eoq_discount(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("break_quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit_prices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1014(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_epq" => {
            let r = f_epq(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("production_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_erlang_c_staffing" => {
            let r = f_staffing(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_ewma_limits" => {
            let r = f_ewma_limits(match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("smoothing") { Some(x) => py::from_val(x), None => { return None } }, match v.get("limit_sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_number") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1165(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_exponential_reliability" => {
            let r = f_exponential_reliability(match v.get("failure_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_exponential_smoothing" => {
            let r = f_exp_smoothing(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_forecast") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1266(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_factor_rating" => {
            let r = f_factor_rating(match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1345(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_flow_shop_makespan" => {
            let r = f_flow_shop_makespan(match v.get("times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_forecast_errors" => {
            let r = f_forecast_errors(match v.get("actual") { Some(x) => py::from_val(x), None => { return None } }, match v.get("forecast") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1478(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_fraction_nonconforming" => {
            let r = f_fraction_nonconforming(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("usl") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1534(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_holt_trend" => {
            let r = f_holt(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_level") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_trend") { Some(x) => py::from_val(x), None => { return None } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1605(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_inventory_total_cost" => {
            let r = f_inventory_total_cost(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("holding_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit_cost") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1649(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_inventory_turnover" => {
            let r = f_inventory_turnover(match v.get("cost_of_goods_sold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("average_inventory") { Some(x) => py::from_val(x), None => { return None } }, match v.get("days_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(365i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1689(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_johnson_three_machine" => {
            let r = f_johnson_three(match v.get("machine1_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("machine2_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("machine3_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1761(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_johnson_two_machine" => {
            let r = f_johnson_two(match v.get("machine1_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("machine2_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1825(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_k_out_of_n_reliability" => {
            let r = f_k_out_of_n(match v.get("component_reliability") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1881(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_kanban_count" => {
            let r = f_kanban_count(match v.get("demand_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("container_size") { Some(x) => py::from_val(x), None => { return None } }, match v.get("safety_factor") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1926(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_largest_candidate_rule" => {
            let r = f_largest_candidate(match v.get("task_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precedence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cycle_time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_cumulative_average" => {
            let r = f_wright_average(match v.get("first_unit_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("learning_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2110(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_fit" => {
            let r = f_learning_fit(match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_total" => {
            let r = f_learning_cumulative(match v.get("first_unit_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("learning_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_curve_unit" => {
            let r = f_learning_unit(match v.get("first_unit_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("learning_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2288(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_learning_rate_from_two_units" => {
            let r = f_learning_two_points(match v.get("unit1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unit2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_line_efficiency" => {
            let r = f_line_efficiency(match v.get("task_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cycle_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stations") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_linear_trend_forecast" => {
            let r = f_linear_trend(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2477(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_littles_law" => {
            let r = f_littles_law(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("number_in_system") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("time_in_system") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2530(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_location_cost_volume" => {
            let r = f_location_cost_volume(match v.get("fixed_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("volume") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2591(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_md1_queue" => {
            let r = f_md1(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2635(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mg1_queue" => {
            let r = f_mg1(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_service_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_service_time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2681(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mm1_queue" => {
            let r = f_mm1(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2723(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mm1k_queue" => {
            let r = f_mm1k(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2780(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mmc_queue" => {
            let r = f_mmc(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("servers") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2836(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_moore_hodgson" => {
            let r = f_moore_hodgson(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2911(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_moving_average_forecast" => {
            let r = f_moving_average(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2979(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_mtbf_confidence" => {
            let r = f_mtbf_confidence(match v.get("total_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("failures") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("test") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("time")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3070(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_multiproduct_break_even" => {
            let r = f_multiproduct_break_even(match v.get("fixed_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sales_mix") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3135(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_discrete" => {
            let r = f_newsvendor_discrete(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demands") { Some(x) => py::from_val(x), None => { return None } }, match v.get("probabilities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3215(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_normal" => {
            let r = f_newsvendor_normal(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_poisson" => {
            let r = f_newsvendor_poisson(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_newsvendor_uniform" => {
            let r = f_newsvendor_uniform(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("low") { Some(x) => py::from_val(x), None => { return None } }, match v.get("high") { Some(x) => py::from_val(x), None => { return None } }, match v.get("salvage") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3361(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_np_chart_limits" => {
            let r = f_np_chart_limits(match v.get("p_bar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3414(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_oee" => {
            let r = f_oee(match v.get("planned_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("downtime") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ideal_cycle_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_count") { Some(x) => py::from_val(x), None => { return None } }, match v.get("good_count") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3464(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_order_up_to_level" => {
            let r = f_order_up_to(match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("review_period") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_level") { Some(x) => py::from_val(x), None => { return None } }, match v.get("inventory_position") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3512(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_p_chart_from_samples" => {
            let r = f_p_chart_counts(match v.get("defectives") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_p_chart_limits" => {
            let r = f_p_chart_limits(match v.get("p_bar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sample_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3646(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_parallel_mttf" => {
            let r = f_parallel_mttf(match v.get("failure_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3699(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_parallel_reliability" => {
            let r = f_parallel_reliability(match v.get("reliabilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3757(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_process_capability" => {
            let r = f_capability(match v.get("usl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3800(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_process_performance" => {
            let r = f_performance(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("usl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3860(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_productivity" => {
            let r = f_productivity(match v.get("output") { Some(x) => py::from_val(x), None => { return None } }, match v.get("input_amounts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("previous_output") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("previous_inputs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3931(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_queue_server_cost" => {
            let r = f_server_cost(match v.get("arrival_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("server_cost_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("waiting_cost_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("max_servers") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(50i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3997(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_ranked_positional_weight" => {
            let r = f_ranked_positional_weight(match v.get("task_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precedence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cycle_time") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_rectilinear_location" => {
            let r = f_rectilinear_location(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4204(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_redundancy_needed" => {
            let r = f_redundancy_needed(match v.get("target_reliability") { Some(x) => py::from_val(x), None => { return None } }, match v.get("component_reliability") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4249(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_regression_forecast" => {
            let r = f_regression_forecast(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_new") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4318(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_reorder_point" => {
            let r = f_reorder_point(match v.get("demand_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("safety_stock") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4360(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_rolled_throughput_yield" => {
            let r = f_rolled_yield(match v.get("yields") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4418(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_rq_fill_rate" => {
            let r = f_rq_fill_rate(match v.get("mean_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fill_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4492(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_safety_stock" => {
            let r = f_safety_stock_normal(match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4536(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_safety_stock_variable_lead_time" => {
            let r = f_safety_stock_vlt(match v.get("mean_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_lead_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service_level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4587(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_seasonal_index" => {
            let r = f_seasonal_index(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("next_year_total") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4656(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_series_parallel_reliability" => {
            let r = f_series_parallel(match v.get("stage_reliabilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_series_reliability" => {
            let r = f_series_reliability(match v.get("reliabilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4777(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_service_level_from_reorder_point" => {
            let r = f_service_from_rop(match v.get("mean_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_ltd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reorder_point") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order_quantity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4819(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_sigma_from_range" => {
            let r = f_sigma_from_range(match v.get("mean_range") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4871(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_spt_sequence" => {
            let r = f_spt_sequence(match v.get("processing_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("due_dates") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4939(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_standard_time" => {
            let r = f_standard_time(match v.get("observed_times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rating") { Some(x) => py::from_val(x), None => { return None } }, match v.get("allowance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("basis") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5002(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_standby_redundancy" => {
            let r = f_standby_redundancy(match v.get("failure_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5058(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_system_failure_rate" => {
            let r = f_system_failure_rate(match v.get("failure_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_tabular_cusum" => {
            let r = f_tabular_cusum(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_takt_time" => {
            let r = f_takt_time(match v.get("available_time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_task_time") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5228(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_time_study_sample_size" => {
            let r = f_time_study_size(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("accuracy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5274(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_tracking_signal" => {
            let r = f_tracking_signal(match v.get("actual") { Some(x) => py::from_val(x), None => { return None } }, match v.get("forecast") { Some(x) => py::from_val(x), None => { return None } }, match v.get("limit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(4.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5346(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_u_chart_limits" => {
            let r = f_u_chart_limits(match v.get("u_bar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weber_location" => {
            let r = f_weber_location(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5481(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weibull_median_rank_regression" => {
            let r = f_weibull_regression(match v.get("failure_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5543(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weibull_reliability" => {
            let r = f_weibull_reliability(match v.get("shape") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scale") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("location") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_weighted_moving_average" => {
            let r = f_weighted_moving_average(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5651(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_work_sampling_sample_size" => {
            let r = f_work_sampling_size(match v.get("proportion") { Some(x) => py::from_val(x), None => { return None } }, match v.get("accuracy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5694(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_xbar_r_limits" => {
            let r = f_xbar_r_limits(match v.get("grand_mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_range") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5746(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ie_xbar_s_limits" => {
            let r = f_xbar_s_limits(match v.get("grand_mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_sd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("subgroup_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5798(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
fn c__DEP_METHODS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("straight-line"), Py::str("declining-balance"), Py::str("sum-of-years-digits"), Py::str("macrs")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MACRS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(3i64), Py::list(vec![Py::Float(0.3333), Py::Float(0.4445), Py::Float(0.1481), Py::Float(0.0741)])), (Py::Int(5i64), Py::list(vec![Py::Float(0.2), Py::Float(0.32), Py::Float(0.192), Py::Float(0.1152), Py::Float(0.1152), Py::Float(0.0576)])), (Py::Int(7i64), Py::list(vec![Py::Float(0.1429), Py::Float(0.2449), Py::Float(0.1749), Py::Float(0.1249), Py::Float(0.0893), Py::Float(0.0892), Py::Float(0.0893), Py::Float(0.0446)])), (Py::Int(10i64), Py::list(vec![Py::Float(0.1), Py::Float(0.18), Py::Float(0.144), Py::Float(0.1152), Py::Float(0.0922), Py::Float(0.0737), Py::Float(0.0655), Py::Float(0.0655), Py::Float(0.0656), Py::Float(0.0655), Py::Float(0.0328)])), (Py::Int(15i64), Py::list(vec![Py::Float(0.05), Py::Float(0.095), Py::Float(0.0855), Py::Float(0.077), Py::Float(0.0693), Py::Float(0.0623), Py::Float(0.059), Py::Float(0.059), Py::Float(0.0591), Py::Float(0.059), Py::Float(0.0591), Py::Float(0.059), Py::Float(0.0591), Py::Float(0.059), Py::Float(0.0591), Py::Float(0.0295)])), (Py::Int(20i64), Py::list(vec![Py::Float(0.0375), Py::Float(0.07219), Py::Float(0.06677), Py::Float(0.06177), Py::Float(0.05713), Py::Float(0.05285), Py::Float(0.04888), Py::Float(0.04522), Py::Float(0.04462), Py::Float(0.04461), Py::Float(0.04462), Py::Float(0.04461), Py::Float(0.04462), Py::Float(0.04461), Py::Float(0.04462), Py::Float(0.04461), Py::Float(0.04462), Py::Float(0.04461), Py::Float(0.04462), Py::Float(0.04461), Py::Float(0.02231)]))])) })());
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
    while ({ let t4116 = v_k.clone(); Py::Bool({ let t4117 = v_n.clone(); if py::lt(&t4116, &t4117)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_loads.clone(), "append", &[Py::Float(0.0)])?;
        v_s = py::len(&v_loads.clone())?;
        while (Py::Bool(true)).truthy() {
            v_best = py::neg(&Py::Int(1i64))?;
            for t4100 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_j = t4100;
                if ({ let t4103 = py::getitem(&v_assigned.clone(), &v_j.clone())?; if t4103.truthy() { t4103 } else { { let t4101 = py::add(&py::getitem(&v_loads.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_t.clone(), &v_j.clone())?)?; Py::Bool({ let t4102 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t4101, &t4102)? { true } else { false } }) } } }).truthy() {
                    continue;
                }
                if (py::not(&Py::Bool(({ let mut out = vec![]; for t4104 in py::iter(&(py::getitem(&v_preds.clone(), &v_j.clone())?))? { let mut v_p: Py; 
v_p = t4104;
out.push(py::getitem(&v_assigned.clone(), &v_p.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
                    continue;
                }
                if ({ let t4109 = { let t4105 = v_best.clone(); Py::Bool({ let t4106 = Py::Int(0i64); if py::lt(&t4105, &t4106)? { true } else { false } }) }; if t4109.truthy() { t4109 } else { { let t4107 = py::getitem(&v_priority.clone(), &v_j.clone())?; Py::Bool({ let t4108 = py::getitem(&v_priority.clone(), &v_best.clone())?; if py::gt(&t4107, &t4108)? { true } else { false } }) } } }).truthy() {
                    v_best = v_j.clone();
                }
            }
            if ({ let t4110 = v_best.clone(); Py::Bool({ let t4111 = Py::Int(0i64); if py::lt(&t4110, &t4111)? { true } else { false } }) }).truthy() {
                break;
            }
            py::setitem(&v_assigned.clone(), &v_best.clone(), Py::Bool(true))?;
            py::setitem(&v_station.clone(), &v_best.clone(), v_s.clone())?;
            let t4112 = v_loads.clone(); let t4113 = py::neg(&Py::Int(1i64))?;
            py::setitem(&t4112, &t4113, py::iadd(&py::getitem(&t4112, &t4113)?, &py::getitem(&v_t.clone(), &v_best.clone())?)?)?;
            v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        }
        let _ = f__need({ let t4114 = py::getitem(&v_loads.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t4115 = Py::Int(0i64); if py::gt(&t4114, &t4115)? { true } else { false } }) }, Py::str("a task does not fit in the cycle time"))?;
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
    let t4454 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t4454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t4454[0].clone();
    v_fb = t4454[1].clone();
    if ({ let t4455 = v_fa.clone(); Py::Bool({ let t4456 = Py::Int(0i64); if py::eq(&t4455, &t4456) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t4457 = v_fb.clone(); Py::Bool({ let t4458 = Py::Int(0i64); if py::eq(&t4457, &t4458) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t4459 = { let t4461 = v_fa.clone(); Py::Bool({ let t4462 = Py::Int(0i64); if py::lt(&t4461, &t4462)? { true } else { false } }) }; Py::Bool({ let t4460 = { let t4463 = v_fb.clone(); Py::Bool({ let t4464 = Py::Int(0i64); if py::lt(&t4463, &t4464)? { true } else { false } }) }; if py::ne(&t4459, &t4460) { true } else { false } }) }, Py::str("no solution in the admissible range"))?;
    for t4465 in py::range_iter(&[(v_iters.clone()).i()?]) {
        v__ = t4465;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4470 = { let t4466 = v_m.clone(); Py::Bool({ let t4467 = v_a.clone(); if py::eq(&t4466, &t4467) { true } else { false } }) }; if t4470.truthy() { t4470 } else { { let t4468 = v_m.clone(); Py::Bool({ let t4469 = v_b.clone(); if py::eq(&t4468, &t4469) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4471 = v_fm.clone(); Py::Bool({ let t4472 = Py::Int(0i64); if py::eq(&t4471, &t4472) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4473 = { let t4475 = v_fm.clone(); Py::Bool({ let t4476 = Py::Int(0i64); if py::lt(&t4475, &t4476)? { true } else { false } }) }; Py::Bool({ let t4474 = { let t4477 = v_fa.clone(); Py::Bool({ let t4478 = Py::Int(0i64); if py::lt(&t4477, &t4478)? { true } else { false } }) }; if py::eq(&t4473, &t4474) { true } else { false } }) }).truthy() {
            let t4479 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4479[0].clone();
            v_fa = t4479[1].clone();
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
    if ({ let t5692 = py::abs(&py::sub(&v_x.clone(), &v_k.clone())?)?; Py::Bool({ let t5693 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?; if py::le(&t5692, &t5693)? { true } else { false } }) }).truthy() {
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
    let t3040 = (Py::tuple(vec![py::math1("exp", &py::neg(&v_h.clone())?)?, Py::Float(0.0)])).items()?;
    if t3040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t3040[0].clone();
    v_s = t3040[1].clone();
    for t3041 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_j = t3041;
        if ({ let t3042 = v_j.clone(); Py::Bool({ let t3043 = Py::Int(0i64); if py::gt(&t3042, &t3043)? { true } else { false } }) }).truthy() {
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
    while ({ let t3038 = f__chi2_even_cdf(v_hi.clone(), v_m.clone())?; Py::Bool({ let t3039 = v_p.clone(); if py::lt(&t3038, &t3039)? { true } else { false } }) }).truthy() {
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
    if ({ let t5810 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t5810.truthy() { t5810 } else { Py::Bool(matches!(v_a.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t5811 = v_a.clone(); Py::Bool({ let t5812 = v_b.clone(); if py::eq(&t5811, &t5812) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    if ({ let t5814 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t5814.truthy() { t5814 } else { { let t5813 = py::not(&Py::Bool(matches!(v_a.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5813.truthy() { t5813 } else { py::not(&py::math1("isfinite", &v_a.clone())?) } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    return Ok((if ({ let t5815 = v_d.clone(); Py::Bool({ let t5816 = Py::Float(1e-15); if py::le(&t5815, &t5816)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? }));
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
    let t3974 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t3974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t3974[0].clone();
    v_s = t3974[1].clone();
    for t3975 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_c.clone()).i()?]) {
        v_k = t3975;
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
    let _ = f__need({ let t5770 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5770.truthy() { t5770 } else { { let t5769 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5769.truthy() { t5769 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let _ = f__need({ let t5778 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5778.truthy() { t5778 } else { { let t5777 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5777.truthy() { t5777 } else { { let t5776 = py::math1("isfinite", &v_v.clone())?; if !t5776.truthy() { t5776 } else { { let t5774 = py::float(&v_v.clone())?; Py::Bool({ let t5775 = py::int(&v_v.clone())?; if py::eq(&t5774, &t5775) { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))?;
    v_v = py::int(&v_v.clone())?;
    let _ = f__need({ let t5779 = v_v.clone(); Py::Bool({ let t5780 = v_lo.clone(); if py::ge(&t5779, &t5780)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
    let _ = f__need({ let t5785 = { let t5781 = v_hi.clone(); Py::Bool({ let t5782 = Py::None; if py::is(&t5781, &t5782) { true } else { false } }) }; if t5785.truthy() { t5785 } else { { let t5783 = v_v.clone(); Py::Bool({ let t5784 = v_hi.clone(); if py::le(&t5783, &t5784)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?].concat()))?;
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
    if ({ let t4912 = v_due_dates.clone(); Py::Bool({ let t4913 = Py::None; if !py::is(&t4912, &t4913) { true } else { false } }) }).truthy() {
        v_d = f__vec(Py::str("due_dates"), v_due_dates.clone(), Py::Int(1i64), v_n_max.clone(), Py::Bool(false), Py::Bool(true))?;
        let _ = f__need({ let t4914 = py::len(&v_d.clone())?; Py::Bool({ let t4915 = py::len(&v_p.clone())?; if py::eq(&t4914, &t4915) { true } else { false } }) }, Py::str("processing_times and due_dates must have the same number of values"))?;
    }
    return Ok(Py::tuple(vec![v_p.clone(), v_d.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__johnson_order(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_last: Py = Py::None;
    v_first = py::sorted_key(&{ let mut out = vec![]; for t1804 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1804;
if ({ let t1805 = py::getitem(&v_a.clone(), &v_j.clone())?; Py::Bool({ let t1806 = py::getitem(&v_b.clone(), &v_j.clone())?; if py::le(&t1805, &t1806)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }, &{ let v_a = v_a.clone(); let v_b = v_b.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_a.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_last = py::sorted_key(&{ let mut out = vec![]; for t1807 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1807;
if ({ let t1808 = py::getitem(&v_a.clone(), &v_j.clone())?; Py::Bool({ let t1809 = py::getitem(&v_b.clone(), &v_j.clone())?; if py::gt(&t1808, &t1809)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }, &{ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_first = v_first.clone(); py::func(move |a: &[Py]| -> R {
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
    let _ = f__need({ let t4681 = Py::Bool(matches!(v_m.clone(), Py::List(_) | Py::Tuple(_))); if !t4681.truthy() { t4681 } else { { let t4679 = py::len(&v_m.clone())?; Py::Bool({ let t4680 = v_rows_min.clone(); if py::ge(&t4679, &t4680)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_rows_min.clone(), "")?, " rows".to_string()].concat()))?;
    v_out = Py::list(vec![]);
    for t4682 in py::iter(&(v_m.clone()))? {
        v_r = t4682;
        let _ = f__need(Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_))), Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix (a list of rows)".to_string()].concat()))?;
        let _ = py::method(&v_out.clone(), "append", &[{ let mut out = vec![]; for t4683 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t4683;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) }])?;
    }
    v_w = py::len(&py::getitem(&v_out.clone(), &Py::Int(0i64))?)?;
    for t4686 in py::iter(&(v_out.clone()))? {
        v_r = t4686;
        let _ = f__need({ let t4687 = py::len(&v_r.clone())?; Py::Bool({ let t4688 = v_w.clone(); if py::eq(&t4687, &t4688) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular (rows of equal length)".to_string()].concat()))?;
    }
    let _ = f__need({ let t4693 = { let t4689 = v_cols.clone(); Py::Bool({ let t4690 = Py::None; if py::is(&t4689, &t4690) { true } else { false } }) }; if t4693.truthy() { t4693 } else { { let t4691 = v_w.clone(); Py::Bool({ let t4692 = v_cols.clone(); if py::eq(&t4691, &t4692) { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_cols.clone(), "")?, " columns".to_string()].concat()))?;
    let _ = f__need({ let t4701 = { let t4694 = v_w.clone(); Py::Bool({ let t4695 = v_cols_min.clone(); if py::ge(&t4694, &t4695)? { true } else { false } }) }; if !t4701.truthy() { t4701 } else { { let t4700 = { let t4696 = v_cols_max.clone(); Py::Bool({ let t4697 = Py::None; if py::is(&t4696, &t4697) { true } else { false } }) }; if t4700.truthy() { t4700 } else { { let t4698 = v_w.clone(); Py::Bool({ let t4699 = v_cols_max.clone(); if py::le(&t4698, &t4699)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " has the wrong number of columns".to_string()].concat()))?;
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
    let t4929 = (Py::tuple(vec![Py::Float(0.0), Py::list(vec![])])).items()?;
    if t4929.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t4929[0].clone();
    v_comp = t4929[1].clone();
    for t4930 in py::iter(&(v_order.clone()))? {
        v_j = t4930;
        v_t = py::iadd(&v_t, &py::getitem(&v_p.clone(), &v_j.clone())?)?;
        let _ = py::method(&v_comp.clone(), "append", &[v_t.clone()])?;
    }
    v_n = py::len(&v_order.clone())?;
    v_out = Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t4931 in py::iter(&(v_order.clone()))? { let mut v_j: Py; 
v_j = t4931;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("mean_flow_time"), py::truediv(&py::sum(&v_comp.clone(), None)?, &v_n.clone())?), (Py::str("makespan"), v_t.clone()), (Py::str("average_jobs_in_system"), py::truediv(&py::sum(&v_comp.clone(), None)?, &v_t.clone())?), (Py::str("utilization"), py::truediv(&v_t.clone(), &py::sum(&v_comp.clone(), None)?)?)]);
    if ({ let t4932 = v_due.clone(); Py::Bool({ let t4933 = Py::None; if !py::is(&t4932, &t4933) { true } else { false } }) }).truthy() {
        v_late = { let mut out = vec![]; for t4934 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4934;
out.push(py::sub(&py::getitem(&v_comp.clone(), &v_k.clone())?, &py::getitem(&v_due.clone(), &py::getitem(&v_order.clone(), &v_k.clone())?)?)?); } Py::list(out) };
        v_tard = { let mut out = vec![]; for t4935 in py::iter(&(v_late.clone()))? { let mut v_x: Py; 
v_x = t4935;
out.push(py::minmax(&[Py::Float(0.0), v_x.clone()], true)?); } Py::list(out) };
        let _ = py::method(&v_out.clone(), "update", &[Py::dict(vec![(Py::str("total_tardiness"), py::sum(&v_tard.clone(), None)?), (Py::str("mean_tardiness"), py::truediv(&py::sum(&v_tard.clone(), None)?, &v_n.clone())?), (Py::str("max_tardiness"), py::minmax(&[v_tard.clone()], true)?), (Py::str("max_lateness"), py::minmax(&[v_late.clone()], true)?), (Py::str("tardy_jobs"), py::sum(&{ let mut out = vec![]; for t4936 in py::iter(&(v_tard.clone()))? { let mut v_x: Py; 
v_x = t4936;
if ({ let t4937 = v_x.clone(); Py::Bool({ let t4938 = Py::Int(0i64); if py::gt(&t4937, &t4938)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)])])?;
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
    let _ = f__need({ let t2677 = v_rho.clone(); Py::Bool({ let t2678 = Py::Int(1i64); if py::lt(&t2677, &t2678)? { true } else { false } }) }, Py::str("the queue is stable only if arrival rate x mean service time is below 1"))?;
    v_Lq = (if ({ let t2679 = v_var.clone(); Py::Bool({ let t2680 = Py::Int(0i64); if py::gt(&t2679, &t2680)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&v_lam.clone(), &v_lam.clone())?, &py::add(&v_var.clone(), &py::mul(&v_es.clone(), &v_es.clone())?)?)?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)? } else { py::truediv(&py::mul(&py::mul(&py::mul(&v_lam.clone(), &v_lam.clone())?, &v_es.clone())?, &v_es.clone())?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)? });
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
    let _ = f__need({ let t5571 = v_v.clone(); Py::Bool({ let t5572 = Py::Int(0i64); if py::ge(&t5571, &t5572)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must not be negative".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nv_costs(mut v_price: Py, mut v_cost: Py, mut v_salvage: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3341 = (Py::tuple(vec![f__pos(Py::str("price"), v_price.clone())?, f__pos(Py::str("cost"), v_cost.clone())?, f__fin(Py::str("salvage"), v_salvage.clone())?])).items()?;
    if t3341.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3341[0].clone();
    v_c = t3341[1].clone();
    v_v = t3341[2].clone();
    let _ = f__need({ let t3342 = v_v.clone(); Py::Bool({ let t3343 = v_c.clone(); if py::lt(&t3342, &t3343)? { { let t3344 = v_p.clone(); if py::lt(&t3343, &t3344)? { true } else { false } } } else { false } }) }, Py::str("the newsvendor needs salvage < cost < price"))?;
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
    let _ = f__need({ let t5315 = py::len(&v_a.clone())?; Py::Bool({ let t5316 = py::len(&v_f.clone())?; if py::eq(&t5315, &t5316) { true } else { false } }) }, Py::str("actual and forecast must have the same number of values"))?;
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
    for t3294 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_kmax.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t3294;
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
    let _ = f__need({ let t5771 = v_v.clone(); Py::Bool({ let t5772 = Py::Int(0i64); if py::gt(&t5771, &t5772)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
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
    v_preds = { let mut out = vec![]; for t4063 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t4063;
out.push(Py::list(vec![])); } Py::list(out) };
    for t4064 in py::iter(&(v_m.clone()))? {
        let t4083 = (t4064).items()?;
        if t4083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t4083[0].clone();
        v_b = t4083[1].clone();
        let _ = f__need({ let t4080 = { let t4065 = v_a.clone(); Py::Bool({ let t4066 = py::int(&v_a.clone())?; if py::eq(&t4065, &t4066) { true } else { false } }) }; if !t4080.truthy() { t4080 } else { { let t4079 = { let t4067 = v_b.clone(); Py::Bool({ let t4068 = py::int(&v_b.clone())?; if py::eq(&t4067, &t4068) { true } else { false } }) }; if !t4079.truthy() { t4079 } else { { let t4078 = { let t4069 = Py::Int(1i64); Py::Bool({ let t4070 = v_a.clone(); if py::le(&t4069, &t4070)? { { let t4071 = v_n.clone(); if py::le(&t4070, &t4071)? { true } else { false } } } else { false } }) }; if !t4078.truthy() { t4078 } else { { let t4077 = { let t4072 = Py::Int(1i64); Py::Bool({ let t4073 = v_b.clone(); if py::le(&t4072, &t4073)? { { let t4074 = v_n.clone(); if py::le(&t4073, &t4074)? { true } else { false } } } else { false } }) }; if !t4077.truthy() { t4077 } else { { let t4075 = v_a.clone(); Py::Bool({ let t4076 = v_b.clone(); if py::ne(&t4075, &t4076) { true } else { false } }) } } } } } } } } }, Py::str("each precedence row [a, b] names two different task numbers (a before b)"))?;
        if ({ let t4081 = py::sub(&py::int(&v_a.clone())?, &Py::Int(1i64))?; Py::Bool({ let t4082 = py::getitem(&v_preds.clone(), &py::sub(&py::int(&v_b.clone())?, &Py::Int(1i64))?)?; if !py::contains(&t4082, &t4081)? { true } else { false } }) }).truthy() {
            let _ = py::method(&py::getitem(&v_preds.clone(), &py::sub(&py::int(&v_b.clone())?, &Py::Int(1i64))?)?, "append", &[py::sub(&py::int(&v_a.clone())?, &Py::Int(1i64))?])?;
        }
    }
    v_state = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
    v_left = v_n.clone();
    v_done = py::mul(&Py::list(vec![Py::Bool(false)]), &v_n.clone())?;
    while (v_left.clone()).truthy() {
        v_free = { let mut out = vec![]; for t4084 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4084;
if ({ let t4086 = py::not(&py::getitem(&v_done.clone(), &v_j.clone())?); if !t4086.truthy() { t4086 } else { Py::Bool(({ let mut out = vec![]; for t4085 in py::iter(&(py::getitem(&v_preds.clone(), &v_j.clone())?))? { let mut v_p: Py; 
v_p = t4085;
out.push(py::getitem(&v_done.clone(), &v_p.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
        let _ = f__need({ let t4087 = py::len(&v_free.clone())?; Py::Bool({ let t4088 = Py::Int(0i64); if py::gt(&t4087, &t4088)? { true } else { false } }) }, Py::str("the precedence relations contain a cycle"))?;
        for t4089 in py::iter(&(v_free.clone()))? {
            v_j = t4089;
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
    let _ = f__need({ let t5676 = v_lo.clone(); Py::Bool({ let t5677 = v_v.clone(); if py::lt(&t5676, &t5677)? { { let t5678 = v_hi.clone(); if py::lt(&t5677, &t5678)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie strictly between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?, " (write 95% or 0.95)".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    if ({ let t5790 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5790.truthy() { t5790 } else { { let t5789 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5789.truthy() { t5789 } else { { let t5787 = v_x.clone(); Py::Bool({ let t5788 = Py::None; if py::is(&t5787, &t5788) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5791 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5791;
out.push(f__r12(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5794 = py::not(&py::math1("isfinite", &v_x.clone())?); if t5794.truthy() { t5794 } else { { let t5792 = v_x.clone(); Py::Bool({ let t5793 = Py::Float(0.0); if py::eq(&t5792, &t5793) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5795 = v_x.clone(); Py::Bool({ let t5796 = Py::Float(0.0); if py::eq(&t5795, &t5796) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
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
    for t5786 in py::iter(&(py::method(&v_d.clone(), "items", &[])?))? {
        let t5797 = (t5786).items()?;
        if t5797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5797[0].clone();
        v_v = t5797[1].clone();
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
    for t4760 in py::iter(&(v_r.clone()))? {
        v_v = t4760;
        let _ = f__need({ let t4761 = Py::Int(0i64); Py::Bool({ let t4762 = v_v.clone(); if py::le(&t4761, &t4762)? { { let t4763 = Py::Int(1i64); if py::le(&t4762, &t4763)? { true } else { false } } } else { false } }) }, Py::str(&["every value of ".to_string(), py::format(&v_name.clone(), "")?, " must lie between 0 and 1".to_string()].concat()))?;
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
    for t5799 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5819 = (t5799).items()?;
        if t5819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5819[0].clone();
        v_w = t5819[1].clone();
        if ({ let t5800 = v_k.clone(); Py::Bool({ let t5801 = v_got.clone(); if !py::contains(&t5801, &t5800)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_o = py::getitem(&v_got.clone(), &v_k.clone())?;
        if (Py::Bool(matches!(v_w.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
            if ({ let t5804 = py::not(&Py::Bool(matches!(v_o.clone(), Py::List(_) | Py::Tuple(_)))); if t5804.truthy() { t5804 } else { { let t5802 = py::len(&v_o.clone())?; Py::Bool({ let t5803 = py::len(&v_w.clone())?; if py::ne(&t5802, &t5803) { true } else { false } }) } } }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            for t5805 in py::iter(&(py::zip(&[v_o.clone(), v_w.clone()])?))? {
                let t5818 = (t5805).items()?;
                if t5818.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_a = t5818[0].clone();
                v_b = t5818[1].clone();
                if (Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
                    if ({ let t5808 = py::not(&Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_)))); if t5808.truthy() { t5808 } else { { let t5806 = py::len(&v_a.clone())?; Py::Bool({ let t5807 = py::len(&v_b.clone())?; if py::ne(&t5806, &t5807) { true } else { false } }) } } }).truthy() {
                        return Ok(Py::Float(f64::INFINITY));
                    }
                    for t5809 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
                        let t5817 = (t5809).items()?;
                        if t5817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_a2 = t5817[0].clone();
                        v_b2 = t5817[1].clone();
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
    let t1759 = Py::Float(0.0);
    v_t1 = t1759.clone();
    v_t2 = t1759.clone();
    v_t3 = t1759.clone();
    for t1760 in py::iter(&(v_order.clone()))? {
        v_j = t1760;
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
    let t1810 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t1810.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t1810[0].clone();
    v_t2 = t1810[1].clone();
    for t1811 in py::iter(&(v_order.clone()))? {
        v_j = t1811;
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
    let _ = f__need({ let t5618 = Py::Bool(matches!(v_xs.clone(), Py::List(_) | Py::Tuple(_))); if !t5618.truthy() { t5618 } else { { let t5616 = py::len(&v_xs.clone())?; Py::Bool({ let t5617 = v_n_min.clone(); if py::ge(&t5616, &t5617)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " values".to_string()].concat()))?;
    let _ = f__need({ let t5623 = { let t5619 = v_n_max.clone(); Py::Bool({ let t5620 = Py::None; if py::is(&t5619, &t5620) { true } else { false } }) }; if t5623.truthy() { t5623 } else { { let t5621 = py::len(&v_xs.clone())?; Py::Bool({ let t5622 = v_n_max.clone(); if py::le(&t5621, &t5622)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " takes at most ".to_string(), py::format(&v_n_max.clone(), "")?, " values".to_string()].concat()))?;
    v_out = { let mut out = vec![]; for t5624 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t5624;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) };
    for t5627 in py::iter(&(v_out.clone()))? {
        v_x = t5627;
        let _ = f__need({ let t5630 = py::not(&v_positive.clone()); if t5630.truthy() { t5630 } else { { let t5628 = v_x.clone(); Py::Bool({ let t5629 = Py::Int(0i64); if py::gt(&t5628, &t5629)? { true } else { false } }) } } }, Py::str(&["every value of ".to_string(), py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
        let _ = f__need({ let t5633 = py::not(&v_nonneg.clone()); if t5633.truthy() { t5633 } else { { let t5631 = v_x.clone(); Py::Bool({ let t5632 = Py::Int(0i64); if py::ge(&t5631, &t5632)? { true } else { false } }) } } }, Py::str(&["no value of ".to_string(), py::format(&v_name.clone(), "")?, " may be negative".to_string()].concat()))?;
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
    let t5444 = Py::Float(0.0);
    v_gx = t5444.clone();
    v_gy = t5444.clone();
    for t5445 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_j = t5445;
        if ({ let t5446 = v_j.clone(); Py::Bool({ let t5447 = v_i.clone(); if py::eq(&t5446, &t5447) { true } else { false } }) }).truthy() {
            continue;
        }
        v_d = py::hypot(&[py::sub(&py::getitem(&v_xs.clone(), &v_i.clone())?, &py::getitem(&v_xs.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_ys.clone(), &v_i.clone())?, &py::getitem(&v_ys.clone(), &v_j.clone())?)?])?;
        if ({ let t5448 = v_d.clone(); Py::Bool({ let t5449 = Py::Int(0i64); if py::gt(&t5448, &t5449)? { true } else { false } }) }).truthy() {
            v_gx = py::iadd(&v_gx, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::sub(&py::getitem(&v_xs.clone(), &v_i.clone())?, &py::getitem(&v_xs.clone(), &v_j.clone())?)?)?, &v_d.clone())?)?;
            v_gy = py::iadd(&v_gy, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::sub(&py::getitem(&v_ys.clone(), &v_i.clone())?, &py::getitem(&v_ys.clone(), &v_j.clone())?)?)?, &v_d.clone())?)?;
        }
    }
    return Ok({ let t5450 = py::hypot(&[v_gx.clone(), v_gy.clone()])?; Py::Bool({ let t5451 = py::getitem(&v_w.clone(), &v_i.clone())?; if py::le(&t5450, &t5451)? { true } else { false } }) });
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
    let t4180 = (Py::tuple(vec![py::truediv(&py::sum(&v_w.clone(), None)?, &Py::Int(2i64))?, Py::Float(0.0)])).items()?;
    if t4180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_half = t4180[0].clone();
    v_acc = t4180[1].clone();
    for t4181 in py::range_iter(&[(py::len(&v_order.clone())?).i()?]) {
        v_idx = t4181;
        v_k = py::getitem(&v_order.clone(), &v_idx.clone())?;
        v_acc = py::iadd(&v_acc, &py::getitem(&v_w.clone(), &v_k.clone())?)?;
        if ({ let t4182 = v_acc.clone(); Py::Bool({ let t4183 = v_half.clone(); if py::eq(&t4182, &t4183) { true } else { false } }) }).truthy() {
            v_nxt = (if ({ let t4184 = py::add(&v_idx.clone(), &Py::Int(1i64))?; Py::Bool({ let t4185 = py::len(&v_order.clone())?; if py::lt(&t4184, &t4185)? { true } else { false } }) }).truthy() { py::getitem(&v_v.clone(), &py::getitem(&v_order.clone(), &py::add(&v_idx.clone(), &Py::Int(1i64))?)?)? } else { py::getitem(&v_v.clone(), &v_k.clone())? });
            let _ = f__need({ let t4186 = v_nxt.clone(); Py::Bool({ let t4187 = py::getitem(&v_v.clone(), &v_k.clone())?; if py::eq(&t4186, &t4187) { true } else { false } }) }, Py::str(&["every value between ".to_string(), py::format(&py::getitem(&v_v.clone(), &v_k.clone())?, "g")?, " and ".to_string(), py::format(&v_nxt.clone(), "g")?, " is optimal (the median is not unique)".to_string()].concat()))?;
        }
        if ({ let t4188 = v_acc.clone(); Py::Bool({ let t4189 = v_half.clone(); if py::ge(&t4188, &t4189)? { true } else { false } }) }).truthy() {
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
    let t5427 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t5427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t5427[0].clone();
    v_ys = t5427[1].clone();
    v_w = f__vec(Py::str("weights"), v_weights.clone(), Py::Int(1i64), Py::Int(10000i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t5428 = py::len(&v_xs.clone())?; Py::Bool({ let t5429 = py::len(&v_ys.clone())?; if py::eq(&t5428, &t5429) { { let t5430 = py::len(&v_w.clone())?; if py::eq(&t5429, &t5430) { true } else { false } } } else { false } }) }, Py::str("x, y and weights must have the same number of values"))?;
    let _ = f__need({ let t5431 = py::sum(&v_w.clone(), None)?; Py::Bool({ let t5432 = Py::Int(0i64); if py::gt(&t5431, &t5432)? { true } else { false } }) }, Py::str("the weights must not all be 0"))?;
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
fn f_after_tax_cash_flow(mut v_before_tax_cash_flows: Py, mut v_depreciation: Py, mut v_tax_rate: Py, mut v_investment: Py, mut v_sale_price: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_Sv: Py = Py::None;
    let mut v_atcf: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bv: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_flows: Py = Py::None;
    let mut v_gain: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tax: Py = Py::None;
    let mut v_ti: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_b = f__vec(Py::str("before_tax_cash_flows"), v_before_tax_cash_flows.clone(), Py::Int(1i64), Py::Int(100i64), Py::Bool(false), Py::Bool(false))?;
    v_d = f__vec(Py::str("depreciation"), v_depreciation.clone(), Py::Int(1i64), Py::Int(100i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t151 = py::len(&v_b.clone())?; Py::Bool({ let t152 = py::len(&v_d.clone())?; if py::eq(&t151, &t152) { true } else { false } }) }, Py::str("before_tax_cash_flows and depreciation must have one value per year each"))?;
    v_t = f__prob(Py::str("tax_rate"), v_tax_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?;
    let t156 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t156.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ti = t156[0].clone();
    v_tax = t156[1].clone();
    v_atcf = t156[2].clone();
    for t157 in py::range_iter(&[(py::len(&v_b.clone())?).i()?]) {
        v_i = t157;
        let _ = py::method(&v_ti.clone(), "append", &[py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::getitem(&v_d.clone(), &v_i.clone())?)?])?;
        let _ = py::method(&v_tax.clone(), "append", &[py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::getitem(&v_d.clone(), &v_i.clone())?)?)?])?;
        let _ = py::method(&v_atcf.clone(), "append", &[py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::getitem(&v_d.clone(), &v_i.clone())?)?)?)?])?;
    }
    v_out = Py::dict(vec![(Py::str("taxable_income"), v_ti.clone()), (Py::str("taxes"), v_tax.clone()), (Py::str("after_tax_cash_flows"), v_atcf.clone()), (Py::str("total_tax"), py::fsum(&v_tax.clone())?)]);
    if ({ let t158 = v_sale_price.clone(); Py::Bool({ let t159 = Py::None; if !py::is(&t158, &t159) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t160 = v_investment.clone(); Py::Bool({ let t161 = Py::None; if !py::is(&t160, &t161) { true } else { false } }) }, Py::str("the investment (first cost) is needed for the book value at the sale"))?;
    }
    if ({ let t162 = v_investment.clone(); Py::Bool({ let t163 = Py::None; if !py::is(&t162, &t163) { true } else { false } }) }).truthy() {
        v_I = f__pos(Py::str("investment"), v_investment.clone())?;
        v_D = py::fsum(&v_d.clone())?;
        let _ = f__need({ let t166 = v_D.clone(); Py::Bool({ let t167 = py::mul(&v_I.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t166, &t167)? { true } else { false } }) }, Py::str("the depreciation cannot add up to more than the investment"))?;
        v_bv = py::sub(&v_I.clone(), &v_D.clone())?;
        if ({ let t168 = py::abs(&v_bv.clone())?; Py::Bool({ let t169 = py::mul(&Py::Float(1e-12), &v_I.clone())?; if py::le(&t168, &t169)? { true } else { false } }) }).truthy() {
            v_bv = Py::Float(0.0);
        }
        py::setitem(&v_out.clone(), &Py::str("book_value_end"), v_bv.clone())?;
        v_flows = Py::list(vec![py::neg(&v_I.clone())?]);
        for t170 in py::iter(&(v_atcf.clone()))? {
            v_v = t170;
            let _ = py::method(&v_flows.clone(), "append", &[v_v.clone()])?;
        }
        if ({ let t171 = v_sale_price.clone(); Py::Bool({ let t172 = Py::None; if !py::is(&t171, &t172) { true } else { false } }) }).truthy() {
            v_Sv = f__nonneg(Py::str("sale_price"), v_sale_price.clone())?;
            let _ = f__need({ let t175 = v_Sv.clone(); Py::Bool({ let t176 = v_I.clone(); if py::le(&t175, &t176)? { true } else { false } }) }, Py::str("a sale above the first cost (a capital gain) is outside this capability"))?;
            v_gain = py::sub(&v_Sv.clone(), &v_bv.clone())?;
            if ({ let t177 = py::abs(&v_gain.clone())?; Py::Bool({ let t178 = py::mul(&Py::Float(1e-12), &v_I.clone())?; if py::le(&t177, &t178)? { true } else { false } }) }).truthy() {
                v_gain = Py::Float(0.0);
            }
            py::setitem(&v_out.clone(), &Py::str("gain_on_sale"), v_gain.clone())?;
            py::setitem(&v_out.clone(), &Py::str("tax_on_sale"), py::mul(&v_t.clone(), &v_gain.clone())?)?;
            py::setitem(&v_flows.clone(), &py::neg(&Py::Int(1i64))?, py::sub(&py::add(&py::getitem(&v_flows.clone(), &py::neg(&Py::Int(1i64))?)?, &v_Sv.clone())?, &py::mul(&v_t.clone(), &v_gain.clone())?)?)?;
        }
        py::setitem(&v_out.clone(), &Py::str("cash_flows_with_investment"), v_flows.clone())?;
    }
    return Ok(f__rd(v_out.clone())?);
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
    let _ = f__need({ let t232 = v_L.clone(); Py::Bool({ let t233 = Py::Int(6i64); if py::le(&t232, &t233)? { true } else { false } }) }, Py::str("limit_sigma must be at most 6"))?;
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
    let t273 = (Py::tuple(vec![f__pos(Py::str("mtbf"), v_mtbf.clone())?, f__pos(Py::str("mttr"), v_mttr.clone())?])).items()?;
    if t273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_up = t273[0].clone();
    v_down = t273[1].clone();
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
    let t315 = (Py::tuple(vec![f__nonneg(Py::str("fixed_cost"), v_fixed_cost.clone())?, f__pos(Py::str("price"), v_price.clone())?, f__nonneg(Py::str("variable_cost"), v_variable_cost.clone())?])).items()?;
    if t315.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t315[0].clone();
    v_p = t315[1].clone();
    v_v = t315[2].clone();
    let _ = f__need({ let t316 = v_p.clone(); Py::Bool({ let t317 = v_v.clone(); if py::gt(&t316, &t317)? { true } else { false } }) }, Py::str("the price must exceed the variable cost per unit"))?;
    v_cm = py::sub(&v_p.clone(), &v_v.clone())?;
    v_out = Py::dict(vec![(Py::str("break_even_units"), py::truediv(&v_F.clone(), &v_cm.clone())?), (Py::str("break_even_revenue"), py::truediv(&py::mul(&v_p.clone(), &v_F.clone())?, &v_cm.clone())?), (Py::str("contribution_margin"), v_cm.clone()), (Py::str("contribution_margin_ratio"), py::truediv(&v_cm.clone(), &v_p.clone())?)]);
    if ({ let t318 = v_target_profit.clone(); Py::Bool({ let t319 = Py::None; if !py::is(&t318, &t319) { true } else { false } }) }).truthy() {
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
    let t3784 = (Py::tuple(vec![f__fin(Py::str("usl"), v_usl.clone())?, f__fin(Py::str("lsl"), v_lsl.clone())?, f__fin(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?])).items()?;
    if t3784.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3784[0].clone();
    v_l = t3784[1].clone();
    v_m = t3784[2].clone();
    v_s = t3784[3].clone();
    let _ = f__need({ let t3785 = v_u.clone(); Py::Bool({ let t3786 = v_l.clone(); if py::gt(&t3785, &t3786)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    let t3787 = (Py::tuple(vec![py::truediv(&py::sub(&v_u.clone(), &v_m.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?, py::truediv(&py::sub(&v_m.clone(), &v_l.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?])).items()?;
    if t3787.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cpu = t3787[0].clone();
    v_cpl = t3787[1].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("cpk"), py::minmax(&[v_cpu.clone(), v_cpl.clone()], false)?), (Py::str("cp"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("cpu"), v_cpu.clone()), (Py::str("cpl"), v_cpl.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capacity_measures(mut v_design_capacity: Py, mut v_effective_capacity: Py, mut v_actual_output: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let t400 = (Py::tuple(vec![f__pos(Py::str("design_capacity"), v_design_capacity.clone())?, f__pos(Py::str("effective_capacity"), v_effective_capacity.clone())?, f__nonneg(Py::str("actual_output"), v_actual_output.clone())?])).items()?;
    if t400.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t400[0].clone();
    v_e = t400[1].clone();
    v_a = t400[2].clone();
    let _ = f__need({ let t401 = v_e.clone(); Py::Bool({ let t402 = v_d.clone(); if py::le(&t401, &t402)? { true } else { false } }) }, Py::str("effective capacity cannot exceed design capacity"))?;
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
    let t462 = (f__xyw(v_x.clone(), v_y.clone(), v_weights.clone())?).items()?;
    if t462.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t462[0].clone();
    v_ys = t462[1].clone();
    v_w = t462[2].clone();
    v_W = py::sum(&v_w.clone(), None)?;
    v_cx = py::truediv(&py::sum(&{ let mut out = vec![]; for t463 in py::iter(&(py::zip(&[v_xs.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t464 = (t463).items()?;
if t464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t464[0].clone();
v_b = t464[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
    v_cy = py::truediv(&py::sum(&{ let mut out = vec![]; for t465 in py::iter(&(py::zip(&[v_ys.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t466 = (t465).items()?;
if t466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t466[0].clone();
v_b = t466[1].clone();
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
    let t506 = (Py::tuple(vec![f__fin(Py::str("usl"), v_usl.clone())?, f__fin(Py::str("lsl"), v_lsl.clone())?, f__fin(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?, f__fin(Py::str("target"), v_target.clone())?])).items()?;
    if t506.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_u = t506[0].clone();
    v_l = t506[1].clone();
    v_m = t506[2].clone();
    v_s = t506[3].clone();
    v_t = t506[4].clone();
    let _ = f__need({ let t507 = v_u.clone(); Py::Bool({ let t508 = v_l.clone(); if py::gt(&t507, &t508)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    let _ = f__need({ let t509 = v_l.clone(); Py::Bool({ let t510 = v_t.clone(); if py::lt(&t509, &t510)? { { let t511 = v_u.clone(); if py::lt(&t510, &t511)? { true } else { false } } } else { false } }) }, Py::str("the target must lie between the specification limits"))?;
    v_tau = py::math1("sqrt", &py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::pow(&py::sub(&v_m.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("cpm"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_tau.clone())?)?), (Py::str("cp"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_depreciation_schedule(mut v_cost: Py, mut v_life: Py, mut v_method: Py, mut v_salvage: Py, mut v_factor: Py, mut v_year: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_book: Py = Py::None;
    let mut v_bv: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dep: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_syd: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_P = f__pos(Py::str("cost"), v_cost.clone())?;
    v_n = f__int(Py::str("life"), v_life.clone(), Py::Int(1i64), Py::Int(100i64))?;
    let _ = f__need({ let t563 = v_method.clone(); Py::Bool({ let t564 = c__DEP_METHODS()?; if py::contains(&t564, &t563)? { true } else { false } }) }, Py::str("method is straight-line, declining-balance, sum-of-years-digits or macrs"))?;
    if ({ let t565 = v_method.clone(); Py::Bool({ let t566 = Py::str("macrs"); if py::eq(&t565, &t566) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t567 = v_salvage.clone(); Py::Bool({ let t568 = Py::None; if py::is(&t567, &t568) { true } else { false } }) }, Py::str("MACRS ignores the salvage value: leave it out"))?;
        let _ = f__need({ let t569 = v_n.clone(); Py::Bool({ let t570 = c__MACRS()?; if py::contains(&t570, &t569)? { true } else { false } }) }, Py::str("a MACRS recovery period is 3, 5, 7, 10, 15 or 20 years"))?;
    }
    v_S = Py::Float(0.0);
    if ({ let t571 = v_salvage.clone(); Py::Bool({ let t572 = Py::None; if !py::is(&t571, &t572) { true } else { false } }) }).truthy() {
        v_S = f__nonneg(Py::str("salvage"), v_salvage.clone())?;
    }
    let _ = f__need({ let t575 = v_S.clone(); Py::Bool({ let t576 = v_P.clone(); if py::lt(&t575, &t576)? { true } else { false } }) }, Py::str("the salvage value must be less than the cost"))?;
    v_f = Py::Float(2.0);
    if ({ let t577 = v_factor.clone(); Py::Bool({ let t578 = Py::None; if !py::is(&t577, &t578) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t579 = v_method.clone(); Py::Bool({ let t580 = Py::str("declining-balance"); if py::eq(&t579, &t580) { true } else { false } }) }, Py::str("a declining-balance factor applies only to the declining-balance method"))?;
        v_f = f__pos(Py::str("factor"), v_factor.clone())?;
        let _ = f__need({ let t581 = py::truediv(&v_f.clone(), &v_n.clone())?; Py::Bool({ let t582 = Py::Int(1i64); if py::le(&t581, &t582)? { true } else { false } }) }, Py::str("the declining-balance rate factor/life must not exceed 1"))?;
    }
    v_dep = Py::list(vec![]);
    if ({ let t583 = v_method.clone(); Py::Bool({ let t584 = Py::str("straight-line"); if py::eq(&t583, &t584) { true } else { false } }) }).truthy() {
        for t585 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_t = t585;
            let _ = py::method(&v_dep.clone(), "append", &[py::truediv(&py::sub(&v_P.clone(), &v_S.clone())?, &v_n.clone())?])?;
        }
    } else {
        if ({ let t586 = v_method.clone(); Py::Bool({ let t587 = Py::str("sum-of-years-digits"); if py::eq(&t586, &t587) { true } else { false } }) }).truthy() {
            v_syd = py::truediv(&py::mul(&v_n.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?;
            for t588 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_t = t588;
                let _ = py::method(&v_dep.clone(), "append", &[py::truediv(&py::mul(&py::sub(&v_P.clone(), &v_S.clone())?, &py::add(&py::sub(&v_n.clone(), &v_t.clone())?, &Py::Int(1i64))?)?, &v_syd.clone())?])?;
            }
        } else {
            if ({ let t589 = v_method.clone(); Py::Bool({ let t590 = Py::str("declining-balance"); if py::eq(&t589, &t590) { true } else { false } }) }).truthy() {
                v_r = py::truediv(&v_f.clone(), &v_n.clone())?;
                v_bv = v_P.clone();
                for t591 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_t = t591;
                    v_d = py::minmax(&[py::mul(&v_r.clone(), &v_bv.clone())?, py::sub(&v_bv.clone(), &v_S.clone())?], false)?;
                    let _ = py::method(&v_dep.clone(), "append", &[v_d.clone()])?;
                    v_bv = py::sub(&v_bv.clone(), &v_d.clone())?;
                }
            } else {
                for t592 in py::iter(&(py::getitem(&c__MACRS()?, &v_n.clone())?))? {
                    v_p = t592;
                    let _ = py::method(&v_dep.clone(), "append", &[py::mul(&v_P.clone(), &v_p.clone())?])?;
                }
            }
        }
    }
    v_book = Py::list(vec![]);
    v_bv = v_P.clone();
    for t593 in py::iter(&(v_dep.clone()))? {
        v_d = t593;
        v_bv = py::sub(&v_bv.clone(), &v_d.clone())?;
        if ({ let t594 = py::abs(&py::sub(&v_bv.clone(), &v_S.clone())?)?; Py::Bool({ let t595 = py::mul(&Py::Float(1e-12), &v_P.clone())?; if py::le(&t594, &t595)? { true } else { false } }) }).truthy() {
            v_bv = v_S.clone();
        }
        let _ = py::method(&v_book.clone(), "append", &[v_bv.clone()])?;
    }
    v_out = Py::dict(vec![(Py::str("depreciation"), v_dep.clone()), (Py::str("book_value"), v_book.clone()), (Py::str("total_depreciation"), py::sub(&v_P.clone(), &py::getitem(&v_book.clone(), &py::neg(&Py::Int(1i64))?)?)?)]);
    if ({ let t596 = v_year.clone(); Py::Bool({ let t597 = Py::None; if !py::is(&t596, &t597) { true } else { false } }) }).truthy() {
        v_y = f__int(Py::str("year"), v_year.clone(), Py::Int(1i64), py::len(&v_dep.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("depreciation_in_year"), py::getitem(&v_dep.clone(), &py::sub(&v_y.clone(), &Py::Int(1i64))?)?)?;
        py::setitem(&v_out.clone(), &Py::str("book_value_end_of_year"), py::getitem(&v_book.clone(), &py::sub(&v_y.clone(), &Py::Int(1i64))?)?)?;
        py::setitem(&v_out.clone(), &Py::str("accumulated_depreciation"), py::sub(&v_P.clone(), &py::getitem(&v_book.clone(), &py::sub(&v_y.clone(), &Py::Int(1i64))?)?)?)?;
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dispatch_rule(mut v_processing_times: Py, mut v_due_dates: Py, mut v_rule: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t655 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t655[0].clone();
    v_d = t655[1].clone();
    let _ = f__need({ let t656 = v_rule.clone(); Py::Bool({ let t657 = c__RULES()?; if py::contains(&t657, &t656)? { true } else { false } }) }, Py::str("rule is one of fcfs, spt, lpt, edd, cr"))?;
    v_n = py::len(&v_p.clone())?;
    if ({ let t658 = v_rule.clone(); Py::Bool({ let t659 = Py::str("fcfs"); if py::eq(&t658, &t659) { true } else { false } }) }).truthy() {
        v_order = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    } else {
        if ({ let t660 = v_rule.clone(); Py::Bool({ let t661 = Py::str("spt"); if py::eq(&t660, &t661) { true } else { false } }) }).truthy() {
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
            if ({ let t662 = v_rule.clone(); Py::Bool({ let t663 = Py::str("lpt"); if py::eq(&t662, &t663) { true } else { false } }) }).truthy() {
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
                if ({ let t664 = v_rule.clone(); Py::Bool({ let t665 = Py::str("edd"); if py::eq(&t664, &t665) { true } else { false } }) }).truthy() {
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
    let t717 = (Py::tuple(vec![f__nonneg(Py::str("defects"), v_defects.clone())?, f__pos(Py::str("units"), v_units.clone())?])).items()?;
    if t717.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t717[0].clone();
    v_u = t717[1].clone();
    v_o = f__pos(Py::str("opportunities"), v_opportunities.clone())?;
    v_dpo = py::truediv(&v_d.clone(), &py::mul(&v_u.clone(), &v_o.clone())?)?;
    let _ = f__need({ let t718 = Py::Int(0i64); Py::Bool({ let t719 = v_dpo.clone(); if py::lt(&t718, &t719)? { { let t720 = Py::Int(1i64); if py::lt(&t719, &t720)? { true } else { false } } } else { false } }) }, Py::str("defects per opportunity must lie strictly between 0 and 1"))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("dpmo"), py::mul(&v_dpo.clone(), &Py::Float(1000000.0))?), (Py::str("dpu"), py::truediv(&v_d.clone(), &v_u.clone())?), (Py::str("dpo"), v_dpo.clone()), (Py::str("yield"), py::sub(&Py::Int(1i64), &v_dpo.clone())?), (Py::str("sigma_level"), py::add(&f__zinv(py::sub(&Py::Int(1i64), &v_dpo.clone())?)?, &Py::Float(1.5))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_earned_value_analysis(mut v_planned_value: Py, mut v_earned_value: Py, mut v_actual_cost: Py, mut v_budget_at_completion: Py) -> R {
    let mut v_AC: Py = Py::None;
    let mut v_BAC: Py = Py::None;
    let mut v_EV: Py = Py::None;
    let mut v_PV: Py = Py::None;
    let mut v_cpi: Py = Py::None;
    let mut v_eac: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_spi: Py = Py::None;
    let t762 = (Py::tuple(vec![f__pos(Py::str("planned_value"), v_planned_value.clone())?, f__nonneg(Py::str("earned_value"), v_earned_value.clone())?, f__pos(Py::str("actual_cost"), v_actual_cost.clone())?])).items()?;
    if t762.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PV = t762[0].clone();
    v_EV = t762[1].clone();
    v_AC = t762[2].clone();
    let t763 = (Py::tuple(vec![py::truediv(&v_EV.clone(), &v_AC.clone())?, py::truediv(&v_EV.clone(), &v_PV.clone())?])).items()?;
    if t763.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cpi = t763[0].clone();
    v_spi = t763[1].clone();
    v_out = Py::dict(vec![(Py::str("cost_variance"), py::sub(&v_EV.clone(), &v_AC.clone())?), (Py::str("schedule_variance"), py::sub(&v_EV.clone(), &v_PV.clone())?), (Py::str("cpi"), v_cpi.clone()), (Py::str("spi"), v_spi.clone())]);
    if ({ let t764 = v_budget_at_completion.clone(); Py::Bool({ let t765 = Py::None; if !py::is(&t764, &t765) { true } else { false } }) }).truthy() {
        v_BAC = f__pos(Py::str("budget_at_completion"), v_budget_at_completion.clone())?;
        let _ = f__need({ let t766 = v_EV.clone(); Py::Bool({ let t767 = py::mul(&v_BAC.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t766, &t767)? { true } else { false } }) }, Py::str("the earned value cannot exceed the budget at completion"))?;
        let _ = f__need({ let t768 = v_EV.clone(); Py::Bool({ let t769 = Py::Int(0i64); if py::gt(&t768, &t769)? { true } else { false } }) }, Py::str("with no earned value the cost performance index is 0 and no estimate at completion exists"))?;
        v_eac = py::truediv(&v_BAC.clone(), &v_cpi.clone())?;
        py::setitem(&v_out.clone(), &Py::str("eac"), v_eac.clone())?;
        py::setitem(&v_out.clone(), &Py::str("etc"), py::sub(&v_eac.clone(), &v_AC.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("vac"), py::sub(&v_BAC.clone(), &v_eac.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("eac_atypical"), py::sub(&py::add(&v_AC.clone(), &v_BAC.clone())?, &v_EV.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("eac_cpi_spi"), py::add(&v_AC.clone(), &py::truediv(&py::sub(&v_BAC.clone(), &v_EV.clone())?, &py::mul(&v_cpi.clone(), &v_spi.clone())?)?)?)?;
        py::setitem(&v_out.clone(), &Py::str("percent_complete"), py::truediv(&v_EV.clone(), &v_BAC.clone())?)?;
        if ({ let t770 = v_BAC.clone(); Py::Bool({ let t771 = v_AC.clone(); if py::gt(&t770, &t771)? { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("tcpi"), py::truediv(&py::sub(&v_BAC.clone(), &v_EV.clone())?, &py::sub(&v_BAC.clone(), &v_AC.clone())?)?)?;
        }
    }
    return Ok(f__rd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_edd_sequence(mut v_processing_times: Py, mut v_due_dates: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t829 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t829.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t829[0].clone();
    v_d = t829[1].clone();
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
    let t879 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t879.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t879[0].clone();
    v_S = t879[1].clone();
    v_H = t879[2].clone();
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
    let t919 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t919.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t919[0].clone();
    v_S = t919[1].clone();
    v_H = t919[2].clone();
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
    let t962 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__prob(Py::str("holding_rate"), v_holding_rate.clone(), Py::Float(0.0), Py::Float(10.0))?])).items()?;
    if t962.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t962[0].clone();
    v_S = t962[1].clone();
    v_i = t962[2].clone();
    v_qs = f__vec(Py::str("break_quantities"), v_break_quantities.clone(), Py::Int(1i64), Py::Int(50i64), Py::Bool(false), Py::Bool(true))?;
    v_ps = f__vec(Py::str("unit_prices"), v_unit_prices.clone(), Py::Int(1i64), Py::Int(50i64), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t979 = py::len(&v_qs.clone())?; Py::Bool({ let t980 = py::len(&v_ps.clone())?; if py::eq(&t979, &t980) { true } else { false } }) }, Py::str("break_quantities and unit_prices must have the same number of values"))?;
    let _ = f__need({ let t981 = py::getitem(&v_qs.clone(), &Py::Int(0i64))?; Py::Bool({ let t982 = Py::Int(0i64); if py::eq(&t981, &t982) { true } else { false } }) }, Py::str("the first break quantity must be 0 (the price for any quantity)"))?;
    for t983 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_qs.clone())?).i()?]) {
        v_k = t983;
        let _ = f__need({ let t984 = py::getitem(&v_qs.clone(), &v_k.clone())?; Py::Bool({ let t985 = py::getitem(&v_qs.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t984, &t985)? { true } else { false } }) }, Py::str("break quantities must increase"))?;
        let _ = f__need({ let t986 = py::getitem(&v_ps.clone(), &v_k.clone())?; Py::Bool({ let t987 = py::getitem(&v_ps.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::lt(&t986, &t987)? { true } else { false } }) }, Py::str("unit prices must fall as the break quantity rises"))?;
    }
    v_best = Py::None;
    for t988 in py::range_iter(&[(py::len(&v_qs.clone())?).i()?]) {
        v_k = t988;
        v_H = py::mul(&v_i.clone(), &py::getitem(&v_ps.clone(), &v_k.clone())?)?;
        v_q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?)?;
        if ({ let t993 = { let t989 = py::add(&v_k.clone(), &Py::Int(1i64))?; Py::Bool({ let t990 = py::len(&v_qs.clone())?; if py::lt(&t989, &t990)? { true } else { false } }) }; if !t993.truthy() { t993 } else { { let t991 = v_q.clone(); Py::Bool({ let t992 = py::getitem(&v_qs.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::ge(&t991, &t992)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        if ({ let t994 = v_q.clone(); Py::Bool({ let t995 = py::getitem(&v_qs.clone(), &v_k.clone())?; if py::lt(&t994, &t995)? { true } else { false } }) }).truthy() {
            v_q = py::getitem(&v_qs.clone(), &v_k.clone())?;
        }
        v_tc = py::add(&py::add(&py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_q.clone())?, &py::truediv(&py::mul(&v_H.clone(), &v_q.clone())?, &Py::Int(2i64))?)?, &py::mul(&v_D.clone(), &py::getitem(&v_ps.clone(), &v_k.clone())?)?)?;
        if ({ let t1000 = { let t996 = v_best.clone(); Py::Bool({ let t997 = Py::None; if py::is(&t996, &t997) { true } else { false } }) }; if t1000.truthy() { t1000 } else { { let t998 = v_tc.clone(); Py::Bool({ let t999 = py::sub(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &py::mul(&Py::Float(1e-12), &v_tc.clone())?)?; if py::lt(&t998, &t999)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_tc.clone(), v_q.clone(), py::getitem(&v_ps.clone(), &v_k.clone())?, py::add(&v_k.clone(), &Py::Int(1i64))?]);
        }
    }
    let t1001 = (v_best.clone()).items()?;
    if t1001.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_tc = t1001[0].clone();
    v_q = t1001[1].clone();
    v_p = t1001[2].clone();
    v_tier = t1001[3].clone();
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
    let t1041 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("production_rate"), v_production_rate.clone())?])).items()?;
    if t1041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1041[0].clone();
    v_P = t1041[1].clone();
    let t1042 = (Py::tuple(vec![f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t1042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1042[0].clone();
    v_H = t1042[1].clone();
    let _ = f__need({ let t1043 = v_P.clone(); Py::Bool({ let t1044 = v_D.clone(); if py::gt(&t1043, &t1044)? { true } else { false } }) }, Py::str("the production rate must exceed the demand rate"))?;
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
    let t1136 = (Py::tuple(vec![f__fin(Py::str("target"), v_target.clone())?, f__pos(Py::str("sigma"), v_sigma.clone())?])).items()?;
    if t1136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1136[0].clone();
    v_s = t1136[1].clone();
    let t1140 = (Py::tuple(vec![f__prob(Py::str("smoothing"), v_smoothing.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?, f__pos(Py::str("limit_sigma"), v_limit_sigma.clone())?])).items()?;
    if t1140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t1140[0].clone();
    v_L = t1140[1].clone();
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
    v_f = (if ({ let t1251 = v_initial_forecast.clone(); Py::Bool({ let t1252 = Py::None; if py::is(&t1251, &t1252) { true } else { false } }) }).truthy() { py::getitem(&v_x.clone(), &Py::Int(0i64))? } else { f__fin(Py::str("initial_forecast"), v_initial_forecast.clone())? });
    v_fc = Py::list(vec![v_f.clone()]);
    for t1253 in py::iter(&(v_x.clone()))? {
        v_v = t1253;
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
    let t1194 = (Py::tuple(vec![f__pos(Py::str("failure_rate"), v_failure_rate.clone())?, f__nonneg(Py::str("time"), v_time.clone())?])).items()?;
    if t1194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t1194[0].clone();
    v_t = t1194[1].clone();
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
    v_tot = { let mut out = vec![]; for t1328 in py::iter(&(v_s.clone()))? { let mut v_row: Py; 
v_row = t1328;
out.push(py::sum(&{ let mut out = vec![]; for t1329 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1329;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&v_row.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_best = Py::Int(0i64);
    for t1330 in py::range_iter(&[(py::len(&v_tot.clone())?).i()?]) {
        v_i = t1330;
        if ({ let t1331 = py::getitem(&v_tot.clone(), &v_i.clone())?; Py::Bool({ let t1332 = py::add(&py::getitem(&v_tot.clone(), &v_best.clone())?, &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_tot.clone(), &v_best.clone())?)?], true)?)?)?; if py::gt(&t1331, &t1332)? { true } else { false } }) }).truthy() {
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
    for t1391 in py::iter(&(v_m.clone()))? {
        v_row = t1391;
        for t1392 in py::iter(&(v_row.clone()))? {
            v_v = t1392;
            let _ = f__need({ let t1393 = v_v.clone(); Py::Bool({ let t1394 = Py::Int(0i64); if py::ge(&t1393, &t1394)? { true } else { false } }) }, Py::str("processing times must not be negative"))?;
        }
    }
    let t1395 = (Py::tuple(vec![py::len(&v_m.clone())?, py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?])).items()?;
    if t1395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1395[0].clone();
    v_k = t1395[1].clone();
    v_prev = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?;
    v_last = Py::list(vec![]);
    for t1396 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1396;
        v_cur = Py::list(vec![]);
        for t1397 in py::range_iter(&[(v_k.clone()).i()?]) {
            v_j = t1397;
            v_ready = py::minmax(&[py::getitem(&v_prev.clone(), &v_j.clone())?, (if ({ let t1398 = v_j.clone(); Py::Bool({ let t1399 = Py::Int(0i64); if py::gt(&t1398, &t1399)? { true } else { false } }) }).truthy() { py::getitem(&v_cur.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)? } else { Py::Float(0.0) })], true)?;
            let _ = py::method(&v_cur.clone(), "append", &[py::add(&v_ready.clone(), &py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_j.clone())?)?])?;
        }
        let _ = py::method(&v_last.clone(), "append", &[py::getitem(&v_cur.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?])?;
        v_prev = v_cur.clone();
    }
    return Ok(f__rd(Py::dict(vec![(Py::str("makespan"), py::getitem(&v_last.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("completion_times"), v_last.clone()), (Py::str("idle_time_last_machine"), py::sub(&py::getitem(&v_last.clone(), &py::neg(&Py::Int(1i64))?)?, &py::sum(&{ let mut out = vec![]; for t1412 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1412;
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
    let t1456 = (f__pair(v_actual.clone(), v_forecast.clone())?).items()?;
    if t1456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1456[0].clone();
    v_f = t1456[1].clone();
    v_n = py::len(&v_a.clone())?;
    v_e = { let mut out = vec![]; for t1457 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1457;
out.push(py::sub(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::getitem(&v_f.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_mad = py::truediv(&py::sum(&{ let mut out = vec![]; for t1458 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t1458;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_mse = py::truediv(&py::sum(&{ let mut out = vec![]; for t1459 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t1459;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_out = Py::dict(vec![(Py::str("mad"), v_mad.clone()), (Py::str("mse"), v_mse.clone()), (Py::str("rmse"), py::math1("sqrt", &v_mse.clone())?), (Py::str("bias"), py::truediv(&py::sum(&v_e.clone(), None)?, &v_n.clone())?), (Py::str("running_sum_of_errors"), py::sum(&v_e.clone(), None)?)]);
    if (Py::Bool(({ let mut out = vec![]; for t1460 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1460;
out.push({ let t1461 = v_v.clone(); Py::Bool({ let t1462 = Py::Int(0i64); if py::ne(&t1461, &t1462) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        py::setitem(&v_out.clone(), &Py::str("mape"), py::truediv(&py::mul(&Py::Int(100i64), &py::sum(&{ let mut out = vec![]; for t1463 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1463;
out.push(py::abs(&py::truediv(&py::getitem(&v_e.clone(), &v_i.clone())?, &py::getitem(&v_a.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None)?)?, &v_n.clone())?)?;
    }
    if ({ let t1464 = v_mad.clone(); Py::Bool({ let t1465 = Py::Int(0i64); if py::gt(&t1464, &t1465)? { true } else { false } }) }).truthy() {
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
    let t1505 = (Py::tuple(vec![f__fin(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?])).items()?;
    if t1505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1505[0].clone();
    v_s = t1505[1].clone();
    let _ = f__need({ let t1510 = { let t1506 = v_lsl.clone(); Py::Bool({ let t1507 = Py::None; if !py::is(&t1506, &t1507) { true } else { false } }) }; if t1510.truthy() { t1510 } else { { let t1508 = v_usl.clone(); Py::Bool({ let t1509 = Py::None; if !py::is(&t1508, &t1509) { true } else { false } }) } } }, Py::str("give at least one specification limit"))?;
    v_below = (if ({ let t1511 = v_lsl.clone(); Py::Bool({ let t1512 = Py::None; if !py::is(&t1511, &t1512) { true } else { false } }) }).truthy() { f__cdf(py::truediv(&py::sub(&f__fin(Py::str("lsl"), v_lsl.clone())?, &v_m.clone())?, &v_s.clone())?)? } else { Py::Float(0.0) });
    v_above = (if ({ let t1513 = v_usl.clone(); Py::Bool({ let t1514 = Py::None; if !py::is(&t1513, &t1514) { true } else { false } }) }).truthy() { f__sf(py::truediv(&py::sub(&f__fin(Py::str("usl"), v_usl.clone())?, &v_m.clone())?, &v_s.clone())?)? } else { Py::Float(0.0) });
    if ({ let t1519 = { let t1515 = v_lsl.clone(); Py::Bool({ let t1516 = Py::None; if !py::is(&t1515, &t1516) { true } else { false } }) }; if !t1519.truthy() { t1519 } else { { let t1517 = v_usl.clone(); Py::Bool({ let t1518 = Py::None; if !py::is(&t1517, &t1518) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t1520 = v_usl.clone(); Py::Bool({ let t1521 = v_lsl.clone(); if py::gt(&t1520, &t1521)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
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
    let t1578 = (Py::tuple(vec![f__prob(Py::str("alpha"), v_alpha.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?, f__prob(Py::str("beta"), v_beta.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t1578.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1578[0].clone();
    v_b = t1578[1].clone();
    let t1579 = (Py::tuple(vec![f__fin(Py::str("initial_level"), v_initial_level.clone())?, f__fin(Py::str("initial_trend"), v_initial_trend.clone())?])).items()?;
    if t1579.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lv = t1579[0].clone();
    v_tr = t1579[1].clone();
    v_h = f__int(Py::str("horizon"), v_horizon.clone(), Py::Int(1i64), Py::Int(1000i64))?;
    for t1592 in py::iter(&(v_x.clone()))? {
        v_v = t1592;
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
    let t1632 = (Py::tuple(vec![f__pos(Py::str("demand"), v_demand.clone())?, f__pos(Py::str("order_cost"), v_order_cost.clone())?, f__pos(Py::str("holding_cost"), v_holding_cost.clone())?])).items()?;
    if t1632.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1632[0].clone();
    v_S = t1632[1].clone();
    v_H = t1632[2].clone();
    let t1635 = (Py::tuple(vec![f__pos(Py::str("order_quantity"), v_order_quantity.clone())?, f__nonneg(Py::str("unit_cost"), v_unit_cost.clone())?])).items()?;
    if t1635.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t1635[0].clone();
    v_c = t1635[1].clone();
    let t1636 = (Py::tuple(vec![py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Q.clone())?, py::truediv(&py::mul(&v_H.clone(), &v_Q.clone())?, &Py::Int(2i64))?])).items()?;
    if t1636.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_oc = t1636[0].clone();
    v_hc = t1636[1].clone();
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
    let t1676 = (Py::tuple(vec![f__pos(Py::str("cost_of_goods_sold"), v_cost_of_goods_sold.clone())?, f__pos(Py::str("average_inventory"), v_average_inventory.clone())?])).items()?;
    if t1676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cogs = t1676[0].clone();
    v_inv = t1676[1].clone();
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
    let _ = f__need({ let t1730 = py::len(&v_a.clone())?; Py::Bool({ let t1731 = py::len(&v_b.clone())?; if py::eq(&t1730, &t1731) { { let t1732 = py::len(&v_c.clone())?; if py::eq(&t1731, &t1732) { true } else { false } } } else { false } }) }, Py::str("every machine needs a time for every job"))?;
    let _ = f__need({ let t1737 = { let t1733 = py::minmax(&[v_a.clone()], false)?; Py::Bool({ let t1734 = py::minmax(&[v_b.clone()], true)?; if py::ge(&t1733, &t1734)? { true } else { false } }) }; if t1737.truthy() { t1737 } else { { let t1735 = py::minmax(&[v_c.clone()], false)?; Py::Bool({ let t1736 = py::minmax(&[v_b.clone()], true)?; if py::ge(&t1735, &t1736)? { true } else { false } }) } } }, Py::str("Johnson's extension needs min(machine 1) >= max(machine 2) or min(machine 3) >= max(machine 2)"))?;
    v_order = f__johnson_order({ let mut out = vec![]; for t1744 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1744;
out.push(py::add(&py::getitem(&v_a.clone(), &v_j.clone())?, &py::getitem(&v_b.clone(), &v_j.clone())?)?); } Py::list(out) }, { let mut out = vec![]; for t1745 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1745;
out.push(py::add(&py::getitem(&v_b.clone(), &v_j.clone())?, &py::getitem(&v_c.clone(), &v_j.clone())?)?); } Py::list(out) })?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t1758 in py::iter(&(v_order.clone()))? { let mut v_j: Py; 
v_j = t1758;
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
    let _ = f__need({ let t1802 = py::len(&v_a.clone())?; Py::Bool({ let t1803 = py::len(&v_b.clone())?; if py::eq(&t1802, &t1803) { true } else { false } }) }, Py::str("both machines need a time for every job"))?;
    v_order = f__johnson_order(v_a.clone(), v_b.clone())?;
    v_span = f__two_machine_span(v_order.clone(), v_a.clone(), v_b.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t1824 in py::iter(&(v_order.clone()))? { let mut v_j: Py; 
v_j = t1824;
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
    let _ = f__need({ let t1850 = Py::Int(0i64); Py::Bool({ let t1851 = v_r.clone(); if py::le(&t1850, &t1851)? { { let t1852 = Py::Int(1i64); if py::le(&t1851, &t1852)? { true } else { false } } } else { false } }) }, Py::str("component_reliability must lie between 0 and 1"))?;
    let t1865 = (Py::tuple(vec![f__int(Py::str("n"), v_n.clone(), Py::Int(1i64), Py::Int(1000i64))?, f__int(Py::str("k"), v_k.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t1865.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nn = t1865[0].clone();
    v_kk = t1865[1].clone();
    let _ = f__need({ let t1866 = v_kk.clone(); Py::Bool({ let t1867 = v_nn.clone(); if py::le(&t1866, &t1867)? { true } else { false } }) }, Py::str("k must not exceed n"))?;
    v_s = Py::Float(0.0);
    for t1868 in py::range_iter(&[(v_kk.clone()).i()?, (py::add(&v_nn.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t1868;
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
    let t1908 = (Py::tuple(vec![f__pos(Py::str("demand_rate"), v_demand_rate.clone())?, f__pos(Py::str("lead_time"), v_lead_time.clone())?])).items()?;
    if t1908.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1908[0].clone();
    v_L = t1908[1].clone();
    let t1911 = (Py::tuple(vec![f__pos(Py::str("container_size"), v_container_size.clone())?, f__nonneg(Py::str("safety_factor"), v_safety_factor.clone())?])).items()?;
    if t1911.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1911[0].clone();
    v_a = t1911[1].clone();
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
    let _ = f__need({ let t1969 = py::minmax(&[v_t.clone()], true)?; Py::Bool({ let t1970 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t1969, &t1970)? { true } else { false } }) }, Py::str("every task must fit within the cycle time"))?;
    v_preds = f__precedence(py::len(&v_t.clone())?, v_precedence.clone())?;
    v_n = py::len(&v_t.clone())?;
    v_pr = { let mut out = vec![]; for t2019 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2019;
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
    for t2020 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_pos = t2020;
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
    let t2207 = (Py::tuple(vec![f__pos(Py::str("first_unit_time"), v_first_unit_time.clone())?, f__prob(Py::str("learning_rate"), v_learning_rate.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t2207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t2207[0].clone();
    v_r = t2207[1].clone();
    v_N = f__int(Py::str("units"), v_units.clone(), Py::Int(1i64), Py::Int(200000i64))?;
    v_b = f__b(v_r.clone())?;
    v_tot = Py::Float(0.0);
    for t2220 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t2220;
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
    let _ = f__need({ let t2151 = py::len(&v_n_.clone())?; Py::Bool({ let t2152 = py::len(&v_t.clone())?; if py::eq(&t2151, &t2152) { true } else { false } }) }, Py::str("units and times must have the same number of values"))?;
    let t2155 = (Py::tuple(vec![{ let mut out = vec![]; for t2153 in py::iter(&(v_n_.clone()))? { let mut v_v: Py; 
v_v = t2153;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2154 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t2154;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }])).items()?;
    if t2155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t2155[0].clone();
    v_ys = t2155[1].clone();
    v_k = py::len(&v_xs.clone())?;
    let t2156 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_k.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_k.clone())?])).items()?;
    if t2156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2156[0].clone();
    v_my = t2156[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2157 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2157;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2158 = v_sxx.clone(); Py::Bool({ let t2159 = Py::Int(0i64); if py::gt(&t2158, &t2159)? { true } else { false } }) }, Py::str("the unit numbers must not all be equal"))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t2160 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2161 = (t2160).items()?;
if t2161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2161[0].clone();
v_y = t2161[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t2162 in py::iter(&(v_ys.clone()))? { let mut v_y: Py; 
v_y = t2162;
out.push(py::pow(&py::sub(&v_y.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("learning_rate"), py::pow(&Py::Float(2.0), &v_b.clone())?), (Py::str("exponent"), v_b.clone()), (Py::str("first_unit_time"), py::math1("exp", &v_a.clone())?), (Py::str("r_squared"), (if ({ let t2175 = v_syy.clone(); Py::Bool({ let t2176 = Py::Int(0i64); if py::gt(&t2175, &t2176)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)? } else { Py::Float(1.0) }))]))?);
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
    let t2315 = (Py::tuple(vec![f__pos(Py::str("unit1"), v_unit1.clone())?, f__pos(Py::str("unit2"), v_unit2.clone())?])).items()?;
    if t2315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t2315[0].clone();
    v_n2 = t2315[1].clone();
    let t2316 = (Py::tuple(vec![f__pos(Py::str("time1"), v_time1.clone())?, f__pos(Py::str("time2"), v_time2.clone())?])).items()?;
    if t2316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t2316[0].clone();
    v_t2 = t2316[1].clone();
    let _ = f__need({ let t2317 = v_n1.clone(); Py::Bool({ let t2318 = v_n2.clone(); if py::ne(&t2317, &t2318) { true } else { false } }) }, Py::str("the two unit numbers must differ"))?;
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
    let t2263 = (Py::tuple(vec![f__pos(Py::str("first_unit_time"), v_first_unit_time.clone())?, f__prob(Py::str("learning_rate"), v_learning_rate.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t2263[0].clone();
    v_r = t2263[1].clone();
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
    let t2386 = (Py::tuple(vec![f__pos(Py::str("cycle_time"), v_cycle_time.clone())?, f__int(Py::str("stations"), v_stations.clone(), Py::Int(1i64), Py::Int(10000i64))?])).items()?;
    if t2386.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2386[0].clone();
    v_n = t2386[1].clone();
    v_w = py::sum(&v_t.clone(), None)?;
    let _ = f__need({ let t2387 = py::minmax(&[v_t.clone()], true)?; Py::Bool({ let t2388 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t2387, &t2388)? { true } else { false } }) }, Py::str("every task must fit within the cycle time"))?;
    let _ = f__need({ let t2389 = v_w.clone(); Py::Bool({ let t2390 = py::mul(&py::mul(&v_n.clone(), &v_c.clone())?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t2389, &t2390)? { true } else { false } }) }, Py::str("the stations cannot hold the total work at this cycle time"))?;
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
    let t2458 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?])).items()?;
    if t2458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2458[0].clone();
    v_my = t2458[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2459 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2459;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_b = py::truediv(&py::sum(&{ let mut out = vec![]; for t2460 in py::iter(&(py::zip(&[v_xs.clone(), v_y.clone()])?))? { let mut v_v: Py; let mut v_x: Py; 
let t2461 = (t2460).items()?;
if t2461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2461[0].clone();
v_v = t2461[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_v.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    if ({ let t2462 = py::abs(&v_b.clone())?; Py::Bool({ let t2463 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t2464 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t2464;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::le(&t2462, &t2463)? { true } else { false } }) }).truthy() {
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
    v_given = { let mut out = vec![]; for t2500 in py::iter(&(Py::tuple(vec![v_arrival_rate.clone(), v_number_in_system.clone(), v_time_in_system.clone()])))? { let mut v_v: Py; 
v_v = t2500;
out.push({ let t2501 = v_v.clone(); Py::Bool({ let t2502 = Py::None; if !py::is(&t2501, &t2502) { true } else { false } }) }); } Py::list(out) };
    let _ = f__need({ let t2503 = py::sum(&v_given.clone(), None)?; Py::Bool({ let t2504 = Py::Int(2i64); if py::eq(&t2503, &t2504) { true } else { false } }) }, Py::str("give exactly two of arrival rate, number in system and time in system"))?;
    if ({ let t2505 = v_arrival_rate.clone(); Py::Bool({ let t2506 = Py::None; if py::is(&t2505, &t2506) { true } else { false } }) }).truthy() {
        let t2513 = (Py::tuple(vec![f__nonneg(Py::str("number_in_system"), v_number_in_system.clone())?, f__pos(Py::str("time_in_system"), v_time_in_system.clone())?])).items()?;
        if t2513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_L = t2513[0].clone();
        v_W = t2513[1].clone();
        v_lam = py::truediv(&v_L.clone(), &v_W.clone())?;
    } else {
        if ({ let t2514 = v_number_in_system.clone(); Py::Bool({ let t2515 = Py::None; if py::is(&t2514, &t2515) { true } else { false } }) }).truthy() {
            let t2516 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__nonneg(Py::str("time_in_system"), v_time_in_system.clone())?])).items()?;
            if t2516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lam = t2516[0].clone();
            v_W = t2516[1].clone();
            v_L = py::mul(&v_lam.clone(), &v_W.clone())?;
        } else {
            let t2517 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__nonneg(Py::str("number_in_system"), v_number_in_system.clone())?])).items()?;
            if t2517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lam = t2517[0].clone();
            v_L = t2517[1].clone();
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
    let _ = f__need({ let t2571 = py::len(&v_F.clone())?; Py::Bool({ let t2572 = py::len(&v_V.clone())?; if py::eq(&t2571, &t2572) { true } else { false } }) }, Py::str("fixed_costs and variable_costs must have the same number of values"))?;
    v_q = f__nonneg(Py::str("volume"), v_volume.clone())?;
    v_tot = { let mut out = vec![]; for t2575 in py::range_iter(&[(py::len(&v_F.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2575;
out.push(py::add(&py::getitem(&v_F.clone(), &v_i.clone())?, &py::mul(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_q.clone())?)?); } Py::list(out) };
    v_best = Py::Int(0i64);
    for t2576 in py::range_iter(&[(py::len(&v_tot.clone())?).i()?]) {
        v_i = t2576;
        if ({ let t2577 = py::getitem(&v_tot.clone(), &v_i.clone())?; Py::Bool({ let t2578 = py::sub(&py::getitem(&v_tot.clone(), &v_best.clone())?, &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_tot.clone(), &v_best.clone())?)?], true)?)?)?; if py::lt(&t2577, &t2578)? { true } else { false } }) }).truthy() {
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
    let t2618 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t2618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2618[0].clone();
    v_mu = t2618[1].clone();
    return Ok(f__rd(f__mg1_values(v_lam.clone(), py::truediv(&Py::Int(1i64), &v_mu.clone())?, Py::Float(0.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mg1(mut v_arrival_rate: Py, mut v_mean_service_time: Py, mut v_sd_service_time: Py) -> R {
    let mut v_es: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let t2664 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("mean_service_time"), v_mean_service_time.clone())?, f__nonneg(Py::str("sd_service_time"), v_sd_service_time.clone())?])).items()?;
    if t2664.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2664[0].clone();
    v_es = t2664[1].clone();
    v_sd = t2664[2].clone();
    return Ok(f__rd(f__mg1_values(v_lam.clone(), v_es.clone(), py::mul(&v_sd.clone(), &v_sd.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mm1(mut v_arrival_rate: Py, mut v_service_rate: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let t2708 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t2708.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2708[0].clone();
    v_mu = t2708[1].clone();
    let _ = f__need({ let t2709 = v_lam.clone(); Py::Bool({ let t2710 = v_mu.clone(); if py::lt(&t2709, &t2710)? { true } else { false } }) }, Py::str("the queue is stable only if the arrival rate is below the service rate"))?;
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
    let t2762 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?, f__int(Py::str("capacity"), v_capacity.clone(), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t2762.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2762[0].clone();
    v_mu = t2762[1].clone();
    v_K = t2762[2].clone();
    v_rho = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    let t2763 = (Py::tuple(vec![Py::list(vec![]), Py::Float(1.0)])).items()?;
    if t2763.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ps = t2763[0].clone();
    v_w = t2763[1].clone();
    for t2764 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t2764;
        let _ = py::method(&v_ps.clone(), "append", &[v_w.clone()])?;
        v_w = py::mul(&v_w, &v_rho.clone())?;
    }
    v_tot = py::sum(&v_ps.clone(), None)?;
    v_ps = { let mut out = vec![]; for t2765 in py::iter(&(v_ps.clone()))? { let mut v_p: Py; 
v_p = t2765;
out.push(py::truediv(&v_p.clone(), &v_tot.clone())?); } Py::list(out) };
    v_L = py::sum(&{ let mut out = vec![]; for t2766 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t2766;
out.push(py::mul(&v_n.clone(), &py::getitem(&v_ps.clone(), &v_n.clone())?)?); } Py::list(out) }, None)?;
    v_Lq = py::sum(&{ let mut out = vec![]; for t2767 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t2767;
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
    let t2819 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?, f__int(Py::str("servers"), v_servers.clone(), Py::Int(1i64), Py::Int(500i64))?])).items()?;
    if t2819.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2819[0].clone();
    v_mu = t2819[1].clone();
    v_c = t2819[2].clone();
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_rho = py::truediv(&v_a.clone(), &v_c.clone())?;
    let _ = f__need({ let t2820 = v_rho.clone(); Py::Bool({ let t2821 = Py::Int(1i64); if py::lt(&t2820, &t2821)? { true } else { false } }) }, Py::str("the queue is stable only if the arrival rate is below servers x service rate"))?;
    let t2822 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t2822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t2822[0].clone();
    v_s = t2822[1].clone();
    for t2823 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_c.clone()).i()?]) {
        v_k = t2823;
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
    let t2881 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t2881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2881[0].clone();
    v_d = t2881[1].clone();
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_due_dates = v_due_dates.clone(); let v_p = v_p.clone(); let v_processing_times = v_processing_times.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_d.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    let t2882 = (Py::tuple(vec![Py::list(vec![]), Py::Float(0.0), Py::list(vec![])])).items()?;
    if t2882.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_on = t2882[0].clone();
    v_t = t2882[1].clone();
    v_late = t2882[2].clone();
    for t2883 in py::iter(&(v_order.clone()))? {
        v_j = t2883;
        let _ = py::method(&v_on.clone(), "append", &[v_j.clone()])?;
        v_t = py::iadd(&v_t, &py::getitem(&v_p.clone(), &v_j.clone())?)?;
        if ({ let t2884 = v_t.clone(); Py::Bool({ let t2885 = py::getitem(&v_d.clone(), &v_j.clone())?; if py::gt(&t2884, &t2885)? { true } else { false } }) }).truthy() {
            v_k = py::getitem(&v_on.clone(), &Py::Int(0i64))?;
            for t2886 in py::iter(&(v_on.clone()))? {
                v_x = t2886;
                if ({ let t2894 = { let t2887 = py::getitem(&v_p.clone(), &v_x.clone())?; Py::Bool({ let t2888 = py::getitem(&v_p.clone(), &v_k.clone())?; if py::gt(&t2887, &t2888)? { true } else { false } }) }; if t2894.truthy() { t2894 } else { { let t2893 = { let t2889 = py::getitem(&v_p.clone(), &v_x.clone())?; Py::Bool({ let t2890 = py::getitem(&v_p.clone(), &v_k.clone())?; if py::eq(&t2889, &t2890) { true } else { false } }) }; if !t2893.truthy() { t2893 } else { { let t2891 = v_x.clone(); Py::Bool({ let t2892 = v_k.clone(); if py::lt(&t2891, &t2892)? { true } else { false } }) } } } } }).truthy() {
                    v_k = v_x.clone();
                }
            }
            let _ = py::method(&v_on.clone(), "remove", &[v_k.clone()])?;
            v_t = py::sub(&v_t, &py::getitem(&v_p.clone(), &v_k.clone())?)?;
            let _ = py::method(&v_late.clone(), "append", &[v_k.clone()])?;
        }
    }
    v_seq = py::add(&{ let mut out = vec![]; for t2895 in py::iter(&(v_on.clone()))? { let mut v_j: Py; 
v_j = t2895;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, &py::sorted(&{ let mut out = vec![]; for t2896 in py::iter(&(v_late.clone()))? { let mut v_j: Py; 
v_j = t2896;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, (Py::Bool(false)).truthy())?)?;
    return Ok(f__rd(Py::dict(vec![(Py::str("number_late"), py::len(&v_late.clone())?), (Py::str("late_jobs"), py::sorted(&{ let mut out = vec![]; for t2909 in py::iter(&(v_late.clone()))? { let mut v_j: Py; 
v_j = t2909;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, (Py::Bool(false)).truthy())?), (Py::str("on_time_jobs"), { let mut out = vec![]; for t2910 in py::iter(&(v_on.clone()))? { let mut v_j: Py; 
v_j = t2910;
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
    let _ = f__need({ let t2964 = v_n.clone(); Py::Bool({ let t2965 = py::len(&v_x.clone())?; if py::le(&t2964, &t2965)? { true } else { false } }) }, Py::str("periods must not exceed the number of observations"))?;
    v_fc = Py::list(vec![]);
    for t2966 in py::range_iter(&[(v_n.clone()).i()?, (py::add(&py::len(&v_x.clone())?, &Py::Int(1i64))?).i()?]) {
        v_t = t2966;
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
    let t3018 = (Py::tuple(vec![f__pos(Py::str("total_time"), v_total_time.clone())?, f__int(Py::str("failures"), v_failures.clone(), Py::Int(1i64), Py::Int(10000i64))?])).items()?;
    if t3018.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3018[0].clone();
    v_r = t3018[1].clone();
    v_c = f__prob(Py::str("confidence"), v_confidence.clone(), Py::Float(0.0), Py::Float(1.0))?;
    let _ = f__need({ let t3022 = v_test.clone(); Py::Bool({ let t3023 = Py::tuple(vec![Py::str("time"), Py::str("failure")]); if py::contains(&t3023, &t3022)? { true } else { false } }) }, Py::str("test is 'time' (time-terminated) or 'failure' (failure-terminated)"))?;
    v_dl = (if ({ let t3024 = v_test.clone(); Py::Bool({ let t3025 = Py::str("time"); if py::eq(&t3024, &t3025) { true } else { false } }) }).truthy() { py::add(&py::mul(&Py::Int(2i64), &v_r.clone())?, &Py::Int(2i64))? } else { py::mul(&Py::Int(2i64), &v_r.clone())? });
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
    let _ = f__need({ let t3113 = py::len(&v_p.clone())?; Py::Bool({ let t3114 = py::len(&v_v.clone())?; if py::eq(&t3113, &t3114) { { let t3115 = py::len(&v_w.clone())?; if py::eq(&t3114, &t3115) { true } else { false } } } else { false } }) }, Py::str("prices, variable_costs and sales_mix must have the same number of values"))?;
    let _ = f__need({ let t3116 = py::abs(&py::sub(&py::sum(&v_w.clone(), None)?, &Py::Int(1i64))?)?; Py::Bool({ let t3117 = Py::Float(1e-09); if py::le(&t3116, &t3117)? { true } else { false } }) }, Py::str("the sales mix fractions must add up to 1"))?;
    v_cmr = py::sum(&{ let mut out = vec![]; for t3118 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3118;
out.push(py::mul(&py::sub(&Py::Int(1i64), &py::truediv(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&v_p.clone(), &v_i.clone())?)?)?, &py::getitem(&v_w.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t3119 = v_cmr.clone(); Py::Bool({ let t3120 = Py::Int(0i64); if py::gt(&t3119, &t3120)? { true } else { false } }) }, Py::str("the weighted contribution must be positive"))?;
    v_R = py::truediv(&v_F.clone(), &v_cmr.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("break_even_revenue"), v_R.clone()), (Py::str("weighted_contribution_ratio"), v_cmr.clone()), (Py::str("revenue_by_product"), { let mut out = vec![]; for t3133 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t3133;
out.push(py::mul(&v_R.clone(), &v_x.clone())?); } Py::list(out) }), (Py::str("units_by_product"), { let mut out = vec![]; for t3134 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3134;
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
    let t3166 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3166.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3166[0].clone();
    v_c = t3166[1].clone();
    v_v = t3166[2].clone();
    v_cr = t3166[3].clone();
    v_d = f__vec(Py::str("demands"), v_demands.clone(), Py::Int(2i64), Py::Int(200i64), Py::Bool(false), Py::Bool(true))?;
    v_pr = f__vec(Py::str("probabilities"), v_probabilities.clone(), Py::Int(2i64), Py::Int(200i64), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t3183 = py::len(&v_d.clone())?; Py::Bool({ let t3184 = py::len(&v_pr.clone())?; if py::eq(&t3183, &t3184) { true } else { false } }) }, Py::str("demands and probabilities must have the same number of values"))?;
    for t3185 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_d.clone())?).i()?]) {
        v_k = t3185;
        let _ = f__need({ let t3186 = py::getitem(&v_d.clone(), &v_k.clone())?; Py::Bool({ let t3187 = py::getitem(&v_d.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t3186, &t3187)? { true } else { false } }) }, Py::str("demands must be listed in increasing order"))?;
    }
    v_tot = py::sum(&v_pr.clone(), None)?;
    let _ = f__need({ let t3188 = py::abs(&py::sub(&v_tot.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t3189 = Py::Float(1e-09); if py::le(&t3188, &t3189)? { true } else { false } }) }, Py::str("the probabilities must add up to 1"))?;
    let t3190 = (Py::tuple(vec![Py::Float(0.0), Py::None])).items()?;
    if t3190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3190[0].clone();
    v_Q = t3190[1].clone();
    for t3191 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
        v_k = t3191;
        v_F = py::iadd(&v_F, &py::getitem(&v_pr.clone(), &v_k.clone())?)?;
        if ({ let t3196 = { let t3192 = py::abs(&py::sub(&v_F.clone(), &v_cr.clone())?)?; Py::Bool({ let t3193 = Py::Float(1e-12); if py::le(&t3192, &t3193)? { true } else { false } }) }; if !t3196.truthy() { t3196 } else { { let t3194 = v_k.clone(); Py::Bool({ let t3195 = py::sub(&py::len(&v_d.clone())?, &Py::Int(1i64))?; if py::lt(&t3194, &t3195)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("two order quantities are equally good (the cumulative probability equals the critical ratio)"))));
        }
        if ({ let t3201 = { let t3197 = v_F.clone(); Py::Bool({ let t3198 = v_cr.clone(); if py::gt(&t3197, &t3198)? { true } else { false } }) }; if t3201.truthy() { t3201 } else { { let t3199 = v_k.clone(); Py::Bool({ let t3200 = py::sub(&py::len(&v_d.clone())?, &Py::Int(1i64))?; if py::eq(&t3199, &t3200) { true } else { false } }) } } }).truthy() {
            v_Q = py::getitem(&v_d.clone(), &v_k.clone())?;
            break;
        }
    }
    v_sales = Py::Float(0.0);
    for t3202 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
        v_k = t3202;
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
    let t3246 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3246.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3246[0].clone();
    v_c = t3246[1].clone();
    v_v = t3246[2].clone();
    v_cr = t3246[3].clone();
    let t3247 = (Py::tuple(vec![f__fin(Py::str("mean_demand"), v_mean_demand.clone())?, f__pos(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t3247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t3247[0].clone();
    v_sd = t3247[1].clone();
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
    let t3291 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3291.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3291[0].clone();
    v_c = t3291[1].clone();
    v_v = t3291[2].clone();
    v_cr = t3291[3].clone();
    v_lam = f__pos(Py::str("mean_demand"), v_mean_demand.clone())?;
    let _ = f__need({ let t3292 = v_lam.clone(); Py::Bool({ let t3293 = Py::Int(500i64); if py::le(&t3292, &t3293)? { true } else { false } }) }, Py::str("mean_demand must be at most 500 for the Poisson model"))?;
    v_kmax = py::int(&py::add(&py::add(&v_lam.clone(), &py::mul(&Py::Int(40i64), &py::math1("sqrt", &v_lam.clone())?)?)?, &Py::Int(40i64))?)?;
    v_pm = f__pois_pmfs(v_lam.clone(), v_kmax.clone())?;
    let t3295 = (Py::tuple(vec![Py::Float(0.0), Py::None])).items()?;
    if t3295.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3295[0].clone();
    v_Q = t3295[1].clone();
    for t3296 in py::range_iter(&[(py::add(&v_kmax.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t3296;
        v_F = py::iadd(&v_F, &py::getitem(&v_pm.clone(), &v_k.clone())?)?;
        if ({ let t3297 = py::abs(&py::sub(&v_F.clone(), &v_cr.clone())?)?; Py::Bool({ let t3298 = Py::Float(1e-12); if py::le(&t3297, &t3298)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("two order quantities are equally good (the cumulative probability equals the critical ratio)"))));
        }
        if ({ let t3299 = v_F.clone(); Py::Bool({ let t3300 = v_cr.clone(); if py::gt(&t3299, &t3300)? { true } else { false } }) }).truthy() {
            v_Q = v_k.clone();
            break;
        }
    }
    v_short = Py::Float(0.0);
    for t3301 in py::range_iter(&[(v_Q.clone()).i()?]) {
        v_k = t3301;
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
    let t3345 = (f__nv_costs(v_price.clone(), v_cost.clone(), v_salvage.clone())?).items()?;
    if t3345.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3345[0].clone();
    v_c = t3345[1].clone();
    v_v = t3345[2].clone();
    v_cr = t3345[3].clone();
    let t3346 = (Py::tuple(vec![f__fin(Py::str("low"), v_low.clone())?, f__fin(Py::str("high"), v_high.clone())?])).items()?;
    if t3346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3346[0].clone();
    v_b = t3346[1].clone();
    let _ = f__need({ let t3347 = v_b.clone(); Py::Bool({ let t3348 = v_a.clone(); if py::gt(&t3347, &t3348)? { true } else { false } }) }, Py::str("high must exceed low"))?;
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
    let t3401 = (Py::tuple(vec![f__prob(Py::str("p_bar"), v_p_bar.clone(), Py::Float(0.0), Py::Float(1.0))?, f__int(Py::str("sample_size"), v_sample_size.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t3401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3401[0].clone();
    v_n = t3401[1].clone();
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
    let t3443 = (Py::tuple(vec![f__pos(Py::str("planned_time"), v_planned_time.clone())?, f__nonneg(Py::str("downtime"), v_downtime.clone())?])).items()?;
    if t3443.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3443[0].clone();
    v_dt = t3443[1].clone();
    let t3444 = (Py::tuple(vec![f__pos(Py::str("ideal_cycle_time"), v_ideal_cycle_time.clone())?, f__pos(Py::str("total_count"), v_total_count.clone())?, f__nonneg(Py::str("good_count"), v_good_count.clone())?])).items()?;
    if t3444.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ct = t3444[0].clone();
    v_n = t3444[1].clone();
    v_g = t3444[2].clone();
    let _ = f__need({ let t3445 = v_dt.clone(); Py::Bool({ let t3446 = v_T.clone(); if py::lt(&t3445, &t3446)? { true } else { false } }) }, Py::str("downtime must be less than the planned time"))?;
    let _ = f__need({ let t3447 = v_g.clone(); Py::Bool({ let t3448 = v_n.clone(); if py::le(&t3447, &t3448)? { true } else { false } }) }, Py::str("good count cannot exceed the total count"))?;
    v_run_t = py::sub(&v_T.clone(), &v_dt.clone())?;
    v_perf = py::truediv(&py::mul(&v_ct.clone(), &v_n.clone())?, &v_run_t.clone())?;
    let _ = f__need({ let t3449 = v_perf.clone(); Py::Bool({ let t3450 = py::add(&Py::Int(1i64), &Py::Float(1e-09))?; if py::le(&t3449, &t3450)? { true } else { false } }) }, Py::str("the count exceeds what the ideal cycle time allows in the run time"))?;
    let t3451 = (Py::tuple(vec![py::truediv(&v_run_t.clone(), &v_T.clone())?, py::truediv(&v_g.clone(), &v_n.clone())?])).items()?;
    if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3451[0].clone();
    v_q = t3451[1].clone();
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
    let t3491 = (Py::tuple(vec![f__pos(Py::str("mean_demand"), v_mean_demand.clone())?, f__pos(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t3491.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t3491[0].clone();
    v_s = t3491[1].clone();
    let t3494 = (Py::tuple(vec![f__pos(Py::str("review_period"), v_review_period.clone())?, f__nonneg(Py::str("lead_time"), v_lead_time.clone())?])).items()?;
    if t3494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3494[0].clone();
    v_L = t3494[1].clone();
    v_csl = f__prob(Py::str("service_level"), v_service_level.clone(), Py::Float(0.5), Py::Float(1.0))?;
    v_z = f__zinv(v_csl.clone())?;
    v_sig = py::mul(&v_s.clone(), &py::math1("sqrt", &py::add(&v_T.clone(), &v_L.clone())?)?)?;
    v_S = py::add(&py::mul(&v_d.clone(), &py::add(&v_T.clone(), &v_L.clone())?)?, &py::mul(&v_z.clone(), &v_sig.clone())?)?;
    v_out = Py::dict(vec![(Py::str("order_up_to_level"), v_S.clone()), (Py::str("safety_stock"), py::mul(&v_z.clone(), &v_sig.clone())?), (Py::str("z"), v_z.clone()), (Py::str("sd_protection_demand"), v_sig.clone())]);
    if ({ let t3498 = v_inventory_position.clone(); Py::Bool({ let t3499 = Py::None; if !py::is(&t3498, &t3499) { true } else { false } }) }).truthy() {
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
    for t3565 in py::iter(&(v_d.clone()))? {
        v_x = t3565;
        let _ = f__need({ let t3570 = { let t3566 = v_x.clone(); Py::Bool({ let t3567 = py::int(&v_x.clone())?; if py::eq(&t3566, &t3567) { true } else { false } }) }; if !t3570.truthy() { t3570 } else { { let t3568 = v_x.clone(); Py::Bool({ let t3569 = v_n.clone(); if py::le(&t3568, &t3569)? { true } else { false } }) } } }, Py::str("each count of defectives must be a whole number no larger than the sample size"))?;
    }
    v_p = py::truediv(&py::sum(&v_d.clone(), None)?, &py::mul(&v_n.clone(), &py::len(&v_d.clone())?)?)?;
    let _ = f__need({ let t3571 = Py::Int(0i64); Py::Bool({ let t3572 = v_p.clone(); if py::lt(&t3571, &t3572)? { { let t3573 = Py::Int(1i64); if py::lt(&t3572, &t3573)? { true } else { false } } } else { false } }) }, Py::str("the average fraction defective must lie strictly between 0 and 1"))?;
    v_s = py::math1("sqrt", &py::truediv(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_n.clone())?)?;
    let t3574 = (Py::tuple(vec![py::minmax(&[Py::Float(1.0), py::add(&v_p.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], false)?, py::minmax(&[Py::Float(0.0), py::sub(&v_p.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?])).items()?;
    if t3574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ucl = t3574[0].clone();
    v_lcl = t3574[1].clone();
    v_out = Py::list(vec![]);
    for t3575 in py::range_iter(&[(py::len(&v_d.clone())?).i()?]) {
        v_k = t3575;
        v_f = py::truediv(&py::getitem(&v_d.clone(), &v_k.clone())?, &v_n.clone())?;
        if ({ let t3580 = { let t3576 = v_f.clone(); Py::Bool({ let t3577 = v_ucl.clone(); if py::gt(&t3576, &t3577)? { true } else { false } }) }; if t3580.truthy() { t3580 } else { { let t3578 = v_f.clone(); Py::Bool({ let t3579 = v_lcl.clone(); if py::lt(&t3578, &t3579)? { true } else { false } }) } } }).truthy() {
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
    let t3633 = (Py::tuple(vec![f__prob(Py::str("p_bar"), v_p_bar.clone(), Py::Float(0.0), Py::Float(1.0))?, f__int(Py::str("sample_size"), v_sample_size.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3633[0].clone();
    v_n = t3633[1].clone();
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
    let t3685 = (Py::tuple(vec![f__pos(Py::str("failure_rate"), v_failure_rate.clone())?, f__int(Py::str("n"), v_n.clone(), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t3685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t3685[0].clone();
    v_nn = t3685[1].clone();
    v_s = Py::Float(0.0);
    for t3686 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nn.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t3686;
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
    for t3744 in py::iter(&(v_r.clone()))? {
        v_v = t3744;
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
    let t3841 = (Py::tuple(vec![f__fin(Py::str("usl"), v_usl.clone())?, f__fin(Py::str("lsl"), v_lsl.clone())?])).items()?;
    if t3841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3841[0].clone();
    v_l = t3841[1].clone();
    let _ = f__need({ let t3842 = v_u.clone(); Py::Bool({ let t3843 = v_l.clone(); if py::gt(&t3842, &t3843)? { true } else { false } }) }, Py::str("the upper specification limit must exceed the lower one"))?;
    v_m = py::truediv(&py::sum(&v_x.clone(), None)?, &py::len(&v_x.clone())?)?;
    v_s = py::math1("sqrt", &py::truediv(&py::sum(&{ let mut out = vec![]; for t3844 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3844;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?, &py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?)?;
    let _ = f__need({ let t3845 = v_s.clone(); Py::Bool({ let t3846 = Py::Int(0i64); if py::gt(&t3845, &t3846)? { true } else { false } }) }, Py::str("the data must vary"))?;
    let t3847 = (Py::tuple(vec![py::truediv(&py::sub(&v_u.clone(), &v_m.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?, py::truediv(&py::sub(&v_m.clone(), &v_l.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?])).items()?;
    if t3847.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ppu = t3847[0].clone();
    v_ppl = t3847[1].clone();
    return Ok(f__rd(Py::dict(vec![(Py::str("ppk"), py::minmax(&[v_ppu.clone(), v_ppl.clone()], false)?), (Py::str("pp"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("mean"), v_m.clone()), (Py::str("sd"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_productivity(mut v_output: Py, mut v_input_amounts: Py, mut v_previous_output: Py, mut v_previous_inputs: Py) -> R {
    let mut v_O: Py = Py::None;
    let mut v_Op: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_mfp: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_pch: Py = Py::None;
    let mut v_pm: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xp: Py = Py::None;
    v_O = f__pos(Py::str("output"), v_output.clone())?;
    v_x = f__vec(Py::str("input_amounts"), v_input_amounts.clone(), Py::Int(1i64), Py::Int(50i64), Py::Bool(true), Py::Bool(false))?;
    v_part = { let mut out = vec![]; for t3903 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3903;
out.push(py::truediv(&v_O.clone(), &v_v.clone())?); } Py::list(out) };
    v_mfp = py::truediv(&v_O.clone(), &py::fsum(&v_x.clone())?)?;
    v_out = Py::dict(vec![(Py::str("partial_productivities"), v_part.clone()), (Py::str("multifactor_productivity"), v_mfp.clone())]);
    let _ = f__need({ let t3904 = { let t3906 = v_previous_output.clone(); Py::Bool({ let t3907 = Py::None; if py::is(&t3906, &t3907) { true } else { false } }) }; Py::Bool({ let t3905 = { let t3908 = v_previous_inputs.clone(); Py::Bool({ let t3909 = Py::None; if py::is(&t3908, &t3909) { true } else { false } }) }; if py::eq(&t3904, &t3905) { true } else { false } }) }, Py::str("give both previous_output and previous_inputs for the change"))?;
    if ({ let t3910 = v_previous_output.clone(); Py::Bool({ let t3911 = Py::None; if !py::is(&t3910, &t3911) { true } else { false } }) }).truthy() {
        v_Op = f__pos(Py::str("previous_output"), v_previous_output.clone())?;
        v_xp = f__vec(Py::str("previous_inputs"), v_previous_inputs.clone(), Py::Int(1i64), Py::Int(50i64), Py::Bool(true), Py::Bool(false))?;
        let _ = f__need({ let t3912 = py::len(&v_xp.clone())?; Py::Bool({ let t3913 = py::len(&v_x.clone())?; if py::eq(&t3912, &t3913) { true } else { false } }) }, Py::str("previous_inputs must list the same inputs as input_amounts"))?;
        v_pm = py::truediv(&v_Op.clone(), &py::fsum(&v_xp.clone())?)?;
        v_ch = py::truediv(&py::sub(&v_mfp.clone(), &v_pm.clone())?, &v_pm.clone())?;
        if ({ let t3914 = py::abs(&v_ch.clone())?; Py::Bool({ let t3915 = Py::Float(1e-12); if py::le(&t3914, &t3915)? { true } else { false } }) }).truthy() {
            v_ch = Py::Float(0.0);
        }
        py::setitem(&v_out.clone(), &Py::str("previous_multifactor_productivity"), v_pm.clone())?;
        py::setitem(&v_out.clone(), &Py::str("change_percent"), py::mul(&Py::Int(100i64), &v_ch.clone())?)?;
        v_pch = Py::list(vec![]);
        for t3916 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) {
            v_i = t3916;
            v_c = py::truediv(&py::sub(&py::truediv(&v_O.clone(), &py::getitem(&v_x.clone(), &v_i.clone())?)?, &py::truediv(&v_Op.clone(), &py::getitem(&v_xp.clone(), &v_i.clone())?)?)?, &py::truediv(&v_Op.clone(), &py::getitem(&v_xp.clone(), &v_i.clone())?)?)?;
            if ({ let t3917 = py::abs(&v_c.clone())?; Py::Bool({ let t3918 = Py::Float(1e-12); if py::le(&t3917, &t3918)? { true } else { false } }) }).truthy() {
                v_c = Py::Float(0.0);
            }
            let _ = py::method(&v_pch.clone(), "append", &[py::mul(&Py::Int(100i64), &v_c.clone())?])?;
        }
        py::setitem(&v_out.clone(), &Py::str("partial_change_percent"), v_pch.clone())?;
    }
    return Ok(f__rd(v_out.clone())?);
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
    let _ = f__need({ let t4040 = py::minmax(&[v_t.clone()], true)?; Py::Bool({ let t4041 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t4040, &t4041)? { true } else { false } }) }, Py::str("every task must fit within the cycle time"))?;
    v_n = py::len(&v_t.clone())?;
    v_preds = f__precedence(v_n.clone(), v_precedence.clone())?;
    v_succ = { let mut out = vec![]; for t4090 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t4090;
out.push(Py::list(vec![])); } Py::list(out) };
    for t4091 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t4091;
        for t4092 in py::iter(&(py::getitem(&v_preds.clone(), &v_j.clone())?))? {
            v_p = t4092;
            let _ = py::method(&py::getitem(&v_succ.clone(), &v_p.clone())?, "append", &[v_j.clone()])?;
        }
    }
    v_rpw = Py::list(vec![]);
    for t4093 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t4093;
        let t4094 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, Py::list(vec![v_j.clone()])])).items()?;
        if t4094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_seen = t4094[0].clone();
        v_stack = t4094[1].clone();
        while (v_stack.clone()).truthy() {
            v_u = py::method(&v_stack.clone(), "pop", &[])?;
            for t4095 in py::iter(&(py::getitem(&v_succ.clone(), &v_u.clone())?))? {
                v_v = t4095;
                if ({ let t4096 = v_v.clone(); Py::Bool({ let t4097 = v_seen.clone(); if !py::contains(&t4097, &t4096)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[v_v.clone()])?;
                    let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
                }
            }
        }
        let _ = py::method(&v_rpw.clone(), "append", &[py::add(&py::getitem(&v_t.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t4098 in py::iter(&(v_seen.clone()))? { let mut v_v: Py; 
v_v = t4098;
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
    for t4099 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_pos = t4099;
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
    let t4179 = (f__xyw(v_x.clone(), v_y.clone(), v_weights.clone())?).items()?;
    if t4179.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t4179[0].clone();
    v_ys = t4179[1].clone();
    v_w = t4179[2].clone();
    let t4190 = (Py::tuple(vec![f__wmedian(v_xs.clone(), v_w.clone())?, f__wmedian(v_ys.clone(), v_w.clone())?])).items()?;
    if t4190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t4190[0].clone();
    v_by = t4190[1].clone();
    v_cost = py::sum(&{ let mut out = vec![]; for t4191 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t4191;
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
    let t4232 = (Py::tuple(vec![Py::Int(1i64), py::sub(&Py::Int(1i64), &v_r.clone())?])).items()?;
    if t4232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4232[0].clone();
    v_q = t4232[1].clone();
    while ({ let t4235 = py::sub(&Py::Int(1i64), &v_q.clone())?; Py::Bool({ let t4236 = v_Rt.clone(); if py::lt(&t4235, &t4236)? { true } else { false } }) }).truthy() {
        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
        v_q = py::mul(&v_q, &py::sub(&Py::Int(1i64), &v_r.clone())?)?;
        let _ = f__need({ let t4233 = v_m.clone(); Py::Bool({ let t4234 = Py::Int(10000i64); if py::le(&t4233, &t4234)? { true } else { false } }) }, Py::str("more than 10000 components would be needed"))?;
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
    let t4290 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(3i64), Py::Int(100000i64), Py::Bool(false), Py::Bool(false))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64), Py::Int(100000i64), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t4290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t4290[0].clone();
    v_ys = t4290[1].clone();
    let _ = f__need({ let t4291 = py::len(&v_xs.clone())?; Py::Bool({ let t4292 = py::len(&v_ys.clone())?; if py::eq(&t4291, &t4292) { true } else { false } }) }, Py::str("x and y must have the same number of values"))?;
    v_n = py::len(&v_xs.clone())?;
    let t4293 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t4293.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4293[0].clone();
    v_my = t4293[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t4294 in py::iter(&(v_xs.clone()))? { let mut v_v: Py; 
v_v = t4294;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t4295 in py::iter(&(v_ys.clone()))? { let mut v_v: Py; 
v_v = t4295;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t4300 = { let t4296 = v_sxx.clone(); Py::Bool({ let t4297 = Py::Int(0i64); if py::gt(&t4296, &t4297)? { true } else { false } }) }; if !t4300.truthy() { t4300 } else { { let t4298 = v_syy.clone(); Py::Bool({ let t4299 = Py::Int(0i64); if py::gt(&t4298, &t4299)? { true } else { false } }) } } }, Py::str("x and y must both vary"))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t4301 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4302 = (t4301).items()?;
if t4302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4302[0].clone();
v_b = t4302[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    v_sse = py::sum(&{ let mut out = vec![]; for t4303 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4303;
out.push(py::pow(&py::sub(&py::sub(&py::getitem(&v_ys.clone(), &v_i.clone())?, &v_a.clone())?, &py::mul(&v_b.clone(), &py::getitem(&v_xs.clone(), &v_i.clone())?)?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_out = Py::dict(vec![(Py::str("intercept"), v_a.clone()), (Py::str("slope"), v_b.clone()), (Py::str("r"), py::truediv(&v_sxy.clone(), &py::math1("sqrt", &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)?), (Py::str("r_squared"), py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?), (Py::str("standard_error"), py::math1("sqrt", &py::truediv(&v_sse.clone(), &py::sub(&v_n.clone(), &Py::Int(2i64))?)?)?)]);
    if ({ let t4304 = v_x_new.clone(); Py::Bool({ let t4305 = Py::None; if !py::is(&t4304, &t4305) { true } else { false } }) }).truthy() {
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
    let t4347 = (Py::tuple(vec![f__pos(Py::str("demand_rate"), v_demand_rate.clone())?, f__nonneg(Py::str("lead_time"), v_lead_time.clone())?, f__nonneg(Py::str("safety_stock"), v_safety_stock.clone())?])).items()?;
    if t4347.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4347[0].clone();
    v_L = t4347[1].clone();
    v_ss = t4347[2].clone();
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
    for t4401 in py::iter(&(v_y.clone()))? {
        v_v = t4401;
        let _ = f__need({ let t4402 = Py::Int(0i64); Py::Bool({ let t4403 = v_v.clone(); if py::lt(&t4402, &t4403)? { { let t4404 = Py::Int(1i64); if py::le(&t4403, &t4404)? { true } else { false } } } else { false } }) }, Py::str("every yield must lie in (0, 1]"))?;
    }
    v_p = Py::Float(1.0);
    for t4405 in py::iter(&(v_y.clone()))? {
        v_v = t4405;
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
    let t4445 = (Py::tuple(vec![f__fin(Py::str("mean_ltd"), v_mean_ltd.clone())?, f__pos(Py::str("sd_ltd"), v_sd_ltd.clone())?])).items()?;
    if t4445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t4445[0].clone();
    v_s = t4445[1].clone();
    let t4449 = (Py::tuple(vec![f__pos(Py::str("order_quantity"), v_order_quantity.clone())?, f__prob(Py::str("fill_rate"), v_fill_rate.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t4449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t4449[0].clone();
    v_beta = t4449[1].clone();
    v_target = py::truediv(&py::mul(&py::sub(&Py::Int(1i64), &v_beta.clone())?, &v_Q.clone())?, &v_s.clone())?;
    let _ = f__need({ let t4450 = v_target.clone(); Py::Bool({ let t4451 = f__loss(py::neg(&Py::Float(8.0))?)?; if py::lt(&t4450, &t4451)? { true } else { false } }) }, Py::str("the fill rate is too low for this order quantity (the reorder point would be negative)"))?;
    let _ = f__need({ let t4452 = v_target.clone(); Py::Bool({ let t4453 = f__loss(Py::Float(8.0))?; if py::gt(&t4452, &t4453)? { true } else { false } }) }, Py::str("the fill rate is too close to 1 for this order quantity"))?;
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
    let t4519 = (Py::tuple(vec![f__pos(Py::str("mean_demand"), v_mean_demand.clone())?, f__pos(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t4519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4519[0].clone();
    v_s = t4519[1].clone();
    let t4523 = (Py::tuple(vec![f__pos(Py::str("lead_time"), v_lead_time.clone())?, f__prob(Py::str("service_level"), v_service_level.clone(), Py::Float(0.5), Py::Float(1.0))?])).items()?;
    if t4523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t4523[0].clone();
    v_csl = t4523[1].clone();
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
    let t4565 = (Py::tuple(vec![f__pos(Py::str("mean_demand"), v_mean_demand.clone())?, f__nonneg(Py::str("sd_demand"), v_sd_demand.clone())?])).items()?;
    if t4565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4565[0].clone();
    v_s = t4565[1].clone();
    let t4566 = (Py::tuple(vec![f__pos(Py::str("mean_lead_time"), v_mean_lead_time.clone())?, f__nonneg(Py::str("sd_lead_time"), v_sd_lead_time.clone())?])).items()?;
    if t4566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t4566[0].clone();
    v_sL = t4566[1].clone();
    v_csl = f__prob(Py::str("service_level"), v_service_level.clone(), Py::Float(0.5), Py::Float(1.0))?;
    let _ = f__need({ let t4574 = { let t4570 = v_s.clone(); Py::Bool({ let t4571 = Py::Int(0i64); if py::gt(&t4570, &t4571)? { true } else { false } }) }; if t4574.truthy() { t4574 } else { { let t4572 = v_sL.clone(); Py::Bool({ let t4573 = Py::Int(0i64); if py::gt(&t4572, &t4573)? { true } else { false } }) } } }, Py::str("demand or lead time must vary (a standard deviation must be positive)"))?;
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
    let t4633 = (Py::tuple(vec![py::len(&v_m.clone())?, py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_years = t4633[0].clone();
    v_s = t4633[1].clone();
    v_season = { let mut out = vec![]; for t4634 in py::range_iter(&[(v_s.clone()).i()?]) { let mut v_k: Py; 
v_k = t4634;
out.push(py::truediv(&py::sum(&{ let mut out = vec![]; for t4635 in py::range_iter(&[(v_years.clone()).i()?]) { let mut v_y: Py; 
v_y = t4635;
out.push(py::getitem(&py::getitem(&v_m.clone(), &v_y.clone())?, &v_k.clone())?); } Py::list(out) }, None)?, &v_years.clone())?); } Py::list(out) };
    v_overall = py::truediv(&py::sum(&v_season.clone(), None)?, &v_s.clone())?;
    let _ = f__need({ let t4636 = v_overall.clone(); Py::Bool({ let t4637 = Py::Int(0i64); if py::gt(&t4636, &t4637)? { true } else { false } }) }, Py::str("the average demand must be positive"))?;
    v_idx = { let mut out = vec![]; for t4638 in py::iter(&(v_season.clone()))? { let mut v_v: Py; 
v_v = t4638;
out.push(py::truediv(&v_v.clone(), &v_overall.clone())?); } Py::list(out) };
    v_out = Py::dict(vec![(Py::str("indices"), v_idx.clone()), (Py::str("season_averages"), v_season.clone()), (Py::str("overall_average"), v_overall.clone())]);
    if ({ let t4639 = v_next_year_total.clone(); Py::Bool({ let t4640 = Py::None; if !py::is(&t4639, &t4640) { true } else { false } }) }).truthy() {
        v_T = f__nonneg(Py::str("next_year_total"), v_next_year_total.clone())?;
        py::setitem(&v_out.clone(), &Py::str("forecasts"), { let mut out = vec![]; for t4643 in py::iter(&(v_idx.clone()))? { let mut v_v: Py; 
v_v = t4643;
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
    for t4702 in py::iter(&(v_m.clone()))? {
        v_row = t4702;
        v_q = Py::Float(1.0);
        for t4703 in py::iter(&(v_row.clone()))? {
            v_v = t4703;
            let _ = f__need({ let t4704 = Py::Int(0i64); Py::Bool({ let t4705 = v_v.clone(); if py::le(&t4704, &t4705)? { { let t4706 = Py::Int(1i64); if py::le(&t4705, &t4706)? { true } else { false } } } else { false } }) }, Py::str("every reliability must lie between 0 and 1"))?;
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
    for t4764 in py::iter(&(v_r.clone()))? {
        v_v = t4764;
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
    let t3958 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t3958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t3958[0].clone();
    v_mu = t3958[1].clone();
    let t3961 = (Py::tuple(vec![f__nonneg(Py::str("server_cost_rate"), v_server_cost_rate.clone())?, f__nonneg(Py::str("waiting_cost_rate"), v_waiting_cost_rate.clone())?])).items()?;
    if t3961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cs = t3961[0].clone();
    v_cw = t3961[1].clone();
    v_cmax = f__int(Py::str("max_servers"), v_max_servers.clone(), Py::Int(1i64), Py::Int(500i64))?;
    v_best = Py::None;
    v_c = py::add(&py::int(&py::truediv(&v_lam.clone(), &v_mu.clone())?)?, &Py::Int(1i64))?;
    while ({ let t3981 = v_c.clone(); Py::Bool({ let t3982 = v_cmax.clone(); if py::le(&t3981, &t3982)? { true } else { false } }) }).truthy() {
        v_C = f__erlang_c(v_lam.clone(), v_mu.clone(), v_c.clone())?;
        v_rho = py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?;
        v_L = py::add(&py::truediv(&py::mul(&v_C.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?, &py::truediv(&v_lam.clone(), &v_mu.clone())?)?;
        v_cost = py::add(&py::mul(&v_c.clone(), &v_cs.clone())?, &py::mul(&v_cw.clone(), &v_L.clone())?)?;
        if ({ let t3980 = { let t3976 = v_best.clone(); Py::Bool({ let t3977 = Py::None; if py::is(&t3976, &t3977) { true } else { false } }) }; if t3980.truthy() { t3980 } else { { let t3978 = v_cost.clone(); Py::Bool({ let t3979 = py::sub(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &py::mul(&Py::Float(1e-12), &v_cost.clone())?)?; if py::lt(&t3978, &t3979)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_cost.clone(), v_c.clone(), v_L.clone()]);
        }
        v_c = py::iadd(&v_c, &Py::Int(1i64))?;
    }
    let _ = f__need({ let t3983 = v_best.clone(); Py::Bool({ let t3984 = Py::None; if !py::is(&t3983, &t3984) { true } else { false } }) }, Py::str("max_servers is too small for a stable queue"))?;
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
    let t4804 = (Py::tuple(vec![f__fin(Py::str("mean_ltd"), v_mean_ltd.clone())?, f__pos(Py::str("sd_ltd"), v_sd_ltd.clone())?, f__fin(Py::str("reorder_point"), v_reorder_point.clone())?])).items()?;
    if t4804.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t4804[0].clone();
    v_s = t4804[1].clone();
    v_R = t4804[2].clone();
    v_z = py::truediv(&py::sub(&v_R.clone(), &v_mu.clone())?, &v_s.clone())?;
    v_esc = py::mul(&v_s.clone(), &f__loss(v_z.clone())?)?;
    v_out = Py::dict(vec![(Py::str("cycle_service_level"), f__cdf(v_z.clone())?), (Py::str("z"), v_z.clone()), (Py::str("safety_stock"), py::sub(&v_R.clone(), &v_mu.clone())?), (Py::str("expected_shortage_per_cycle"), v_esc.clone())]);
    if ({ let t4805 = v_order_quantity.clone(); Py::Bool({ let t4806 = Py::None; if !py::is(&t4805, &t4806) { true } else { false } }) }).truthy() {
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
    let t4858 = (Py::tuple(vec![f__pos(Py::str("mean_range"), v_mean_range.clone())?, f__int(Py::str("subgroup_size"), v_subgroup_size.clone(), Py::Int(2i64), Py::Int(10i64))?])).items()?;
    if t4858.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rb = t4858[0].clone();
    v_n = t4858[1].clone();
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
    let t4916 = (f__jobs(v_processing_times.clone(), v_due_dates.clone(), Py::Int(500i64))?).items()?;
    if t4916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4916[0].clone();
    v_d = t4916[1].clone();
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
    let t1084 = (Py::tuple(vec![f__pos(Py::str("arrival_rate"), v_arrival_rate.clone())?, f__pos(Py::str("service_rate"), v_service_rate.clone())?])).items()?;
    if t1084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t1084[0].clone();
    v_mu = t1084[1].clone();
    let t1090 = (Py::tuple(vec![f__nonneg(Py::str("target_time"), v_target_time.clone())?, f__prob(Py::str("target_level"), v_target_level.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t1090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t1090[0].clone();
    v_sl = t1090[1].clone();
    v_c = py::add(&py::int(&py::truediv(&v_lam.clone(), &v_mu.clone())?)?, &Py::Int(1i64))?;
    while (Py::Bool(true)).truthy() {
        let _ = f__need({ let t1091 = v_c.clone(); Py::Bool({ let t1092 = Py::Int(2000i64); if py::le(&t1091, &t1092)? { true } else { false } }) }, Py::str("more than 2000 servers would be needed"))?;
        v_C = f__erlang_c(v_lam.clone(), v_mu.clone(), v_c.clone())?;
        v_level = py::sub(&Py::Int(1i64), &py::mul(&v_C.clone(), &py::math1("exp", &py::mul(&py::neg(&py::sub(&py::mul(&v_c.clone(), &v_mu.clone())?, &v_lam.clone())?)?, &v_t.clone())?)?)?)?;
        if ({ let t1095 = v_level.clone(); Py::Bool({ let t1096 = v_sl.clone(); if py::ge(&t1095, &t1096)? { true } else { false } }) }).truthy() {
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
    let t4985 = (Py::tuple(vec![f__pos(Py::str("rating"), v_rating.clone())?, f__prob(Py::str("allowance"), v_allowance.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?])).items()?;
    if t4985.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t4985[0].clone();
    v_a = t4985[1].clone();
    let _ = f__need({ let t4986 = v_basis.clone(); Py::Bool({ let t4987 = Py::tuple(vec![Py::str("job"), Py::str("shift")]); if py::contains(&t4987, &t4986)? { true } else { false } }) }, Py::str("basis is 'job' (allowance as a fraction of normal time) or 'shift' (of the workday)"))?;
    v_mean = py::truediv(&py::sum(&v_x.clone(), None)?, &py::len(&v_x.clone())?)?;
    v_nt = py::mul(&v_mean.clone(), &v_r.clone())?;
    v_st = (if ({ let t4988 = v_basis.clone(); Py::Bool({ let t4989 = Py::str("job"); if py::eq(&t4988, &t4989) { true } else { false } }) }).truthy() { py::mul(&v_nt.clone(), &py::add(&Py::Int(1i64), &v_a.clone())?)? } else { py::truediv(&v_nt.clone(), &py::sub(&Py::Int(1i64), &v_a.clone())?)? });
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
    let t5043 = (Py::tuple(vec![f__pos(Py::str("failure_rate"), v_failure_rate.clone())?, f__nonneg(Py::str("time"), v_time.clone())?, f__int(Py::str("units"), v_units.clone(), Py::Int(1i64), Py::Int(200i64))?])).items()?;
    if t5043.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t5043[0].clone();
    v_t = t5043[1].clone();
    v_n = t5043[2].clone();
    v_x = py::mul(&v_lam.clone(), &v_t.clone())?;
    let t5044 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t5044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t5044[0].clone();
    v_s = t5044[1].clone();
    for t5045 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t5045;
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
    if ({ let t5099 = v_time.clone(); Py::Bool({ let t5100 = Py::None; if !py::is(&t5099, &t5100) { true } else { false } }) }).truthy() {
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
    let t5160 = (Py::tuple(vec![f__fin(Py::str("target"), v_target.clone())?, f__nonneg(Py::str("k"), v_k.clone())?, f__pos(Py::str("h"), v_h.clone())?])).items()?;
    if t5160.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5160[0].clone();
    v_kk = t5160[1].clone();
    v_hh = t5160[2].clone();
    let t5161 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Int(0i64)])).items()?;
    if t5161.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_cp = t5161[0].clone();
    v_cm = t5161[1].clone();
    v_sig = t5161[2].clone();
    let t5162 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t5162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ups = t5162[0].clone();
    v_downs = t5162[1].clone();
    for t5163 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) {
        v_i = t5163;
        v_cp = py::minmax(&[Py::Float(0.0), py::add(&py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::add(&v_m.clone(), &v_kk.clone())?)?, &v_cp.clone())?], true)?;
        v_cm = py::minmax(&[Py::Float(0.0), py::add(&py::sub(&py::sub(&v_m.clone(), &v_kk.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_cm.clone())?], true)?;
        let _ = py::method(&v_ups.clone(), "append", &[v_cp.clone()])?;
        let _ = py::method(&v_downs.clone(), "append", &[v_cm.clone()])?;
        if ({ let t5171 = { let t5164 = v_sig.clone(); Py::Bool({ let t5165 = Py::Int(0i64); if py::eq(&t5164, &t5165) { true } else { false } }) }; if !t5171.truthy() { t5171 } else { { let t5170 = { let t5166 = v_cp.clone(); Py::Bool({ let t5167 = v_hh.clone(); if py::gt(&t5166, &t5167)? { true } else { false } }) }; if t5170.truthy() { t5170 } else { { let t5168 = v_cm.clone(); Py::Bool({ let t5169 = v_hh.clone(); if py::gt(&t5168, &t5169)? { true } else { false } }) } } } } }).truthy() {
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
    let t5211 = (Py::tuple(vec![f__pos(Py::str("available_time"), v_available_time.clone())?, f__pos(Py::str("demand"), v_demand.clone())?])).items()?;
    if t5211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t5211[0].clone();
    v_D = t5211[1].clone();
    v_c = py::truediv(&v_T.clone(), &v_D.clone())?;
    v_out = Py::dict(vec![(Py::str("cycle_time"), v_c.clone()), (Py::str("output_rate"), py::truediv(&v_D.clone(), &v_T.clone())?)]);
    if ({ let t5212 = v_total_task_time.clone(); Py::Bool({ let t5213 = Py::None; if !py::is(&t5212, &t5213) { true } else { false } }) }).truthy() {
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
    let t5255 = (Py::tuple(vec![f__pos(Py::str("mean"), v_mean.clone())?, f__pos(Py::str("sd"), v_sd.clone())?])).items()?;
    if t5255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5255[0].clone();
    v_s = t5255[1].clone();
    let t5259 = (Py::tuple(vec![f__prob(Py::str("accuracy"), v_accuracy.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("confidence"), v_confidence.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t5259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t5259[0].clone();
    v_c = t5259[1].clone();
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
    let t5317 = (f__pair(v_actual.clone(), v_forecast.clone())?).items()?;
    if t5317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5317[0].clone();
    v_f = t5317[1].clone();
    v_lim = f__pos(Py::str("limit"), v_limit.clone())?;
    let t5320 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t5320.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rs = t5320[0].clone();
    v_sa = t5320[1].clone();
    v_ts = t5320[2].clone();
    v_first = t5320[3].clone();
    for t5321 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) {
        v_i = t5321;
        v_e = py::sub(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::getitem(&v_f.clone(), &v_i.clone())?)?;
        v_rs = py::iadd(&v_rs, &v_e.clone())?;
        v_sa = py::iadd(&v_sa, &py::abs(&v_e.clone())?)?;
        v_mad = py::truediv(&v_sa.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?;
        let _ = f__need({ let t5326 = { let t5322 = v_mad.clone(); Py::Bool({ let t5323 = Py::Int(0i64); if py::gt(&t5322, &t5323)? { true } else { false } }) }; if t5326.truthy() { t5326 } else { { let t5324 = v_rs.clone(); Py::Bool({ let t5325 = Py::Int(0i64); if py::eq(&t5324, &t5325) { true } else { false } }) } } }, Py::str("the tracking signal is undefined"))?;
        v_v = (if ({ let t5327 = v_mad.clone(); Py::Bool({ let t5328 = Py::Int(0i64); if py::gt(&t5327, &t5328)? { true } else { false } }) }).truthy() { py::truediv(&v_rs.clone(), &v_mad.clone())? } else { Py::Float(0.0) });
        let _ = py::method(&v_ts.clone(), "append", &[v_v.clone()])?;
        if ({ let t5333 = { let t5329 = v_first.clone(); Py::Bool({ let t5330 = Py::Int(0i64); if py::eq(&t5329, &t5330) { true } else { false } }) }; if !t5333.truthy() { t5333 } else { { let t5331 = py::abs(&v_v.clone())?; Py::Bool({ let t5332 = v_lim.clone(); if py::gt(&t5331, &t5332)? { true } else { false } }) } } }).truthy() {
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
    let t5373 = (Py::tuple(vec![f__pos(Py::str("u_bar"), v_u_bar.clone())?, f__pos(Py::str("units"), v_units.clone())?])).items()?;
    if t5373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t5373[0].clone();
    v_n = t5373[1].clone();
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
    let t5433 = (f__xyw(v_x.clone(), v_y.clone(), v_weights.clone())?).items()?;
    if t5433.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t5433[0].clone();
    v_ys = t5433[1].clone();
    v_w = t5433[2].clone();
    let _ = f__need({ let t5434 = py::len(&v_xs.clone())?; Py::Bool({ let t5435 = Py::Int(2i64); if py::ge(&t5434, &t5435)? { true } else { false } }) }, Py::str("at least two points are needed"))?;
    for t5436 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_i = t5436;
        for t5437 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t5437;
            let _ = f__need({ let t5442 = { let t5438 = py::getitem(&v_xs.clone(), &v_i.clone())?; Py::Bool({ let t5439 = py::getitem(&v_xs.clone(), &v_j.clone())?; if py::ne(&t5438, &t5439) { true } else { false } }) }; if t5442.truthy() { t5442 } else { { let t5440 = py::getitem(&v_ys.clone(), &v_i.clone())?; Py::Bool({ let t5441 = py::getitem(&v_ys.clone(), &v_j.clone())?; if py::ne(&t5440, &t5441) { true } else { false } }) } } }, Py::str("the points must be distinct"))?;
        }
    }
    v_best = Py::None;
    for t5443 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
        v_i = t5443;
        if (f__weber_vertex(v_xs.clone(), v_ys.clone(), v_w.clone(), v_i.clone())?).truthy() {
            v_best = Py::tuple(vec![py::getitem(&v_xs.clone(), &v_i.clone())?, py::getitem(&v_ys.clone(), &v_i.clone())?]);
        }
    }
    if ({ let t5452 = v_best.clone(); Py::Bool({ let t5453 = Py::None; if py::is(&t5452, &t5453) { true } else { false } }) }).truthy() {
        v_W = py::sum(&v_w.clone(), None)?;
        v_px = py::truediv(&py::sum(&{ let mut out = vec![]; for t5454 in py::iter(&(py::zip(&[v_xs.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5455 = (t5454).items()?;
if t5455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5455[0].clone();
v_b = t5455[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
        v_py = py::truediv(&py::sum(&{ let mut out = vec![]; for t5456 in py::iter(&(py::zip(&[v_ys.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5457 = (t5456).items()?;
if t5457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5457[0].clone();
v_b = t5457[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &v_W.clone())?;
        v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t5458 in py::iter(&(py::add(&v_xs.clone(), &v_ys.clone())?))? { let mut v_v: Py; 
v_v = t5458;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?;
        for t5459 in py::range_iter(&[(Py::Int(200000i64)).i()?]) {
            v__ = t5459;
            let t5460 = Py::Float(0.0);
            v_nx = t5460.clone();
            v_ny = t5460.clone();
            v_den = t5460.clone();
            for t5461 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) {
                v_k = t5461;
                v_d = py::hypot(&[py::sub(&v_px.clone(), &py::getitem(&v_xs.clone(), &v_k.clone())?)?, py::sub(&v_py.clone(), &py::getitem(&v_ys.clone(), &v_k.clone())?)?])?;
                let _ = f__need({ let t5462 = v_d.clone(); Py::Bool({ let t5463 = Py::Float(1e-300); if py::gt(&t5462, &t5463)? { true } else { false } }) }, Py::str("the iteration reached a demand point"))?;
                v_nx = py::iadd(&v_nx, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::getitem(&v_xs.clone(), &v_k.clone())?)?, &v_d.clone())?)?;
                v_ny = py::iadd(&v_ny, &py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::getitem(&v_ys.clone(), &v_k.clone())?)?, &v_d.clone())?)?;
                v_den = py::iadd(&v_den, &py::truediv(&py::getitem(&v_w.clone(), &v_k.clone())?, &v_d.clone())?)?;
            }
            let t5464 = (Py::tuple(vec![py::truediv(&v_nx.clone(), &v_den.clone())?, py::truediv(&v_ny.clone(), &v_den.clone())?])).items()?;
            if t5464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_nx = t5464[0].clone();
            v_ny = t5464[1].clone();
            v_step = py::hypot(&[py::sub(&v_nx.clone(), &v_px.clone())?, py::sub(&v_ny.clone(), &v_py.clone())?])?;
            let t5465 = (Py::tuple(vec![v_nx.clone(), v_ny.clone()])).items()?;
            if t5465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_px = t5465[0].clone();
            v_py = t5465[1].clone();
            if ({ let t5466 = v_step.clone(); Py::Bool({ let t5467 = py::mul(&Py::Float(1e-15), &v_scale.clone())?; if py::le(&t5466, &t5467)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_best = Py::tuple(vec![v_px.clone(), v_py.clone()]);
    }
    v_cost = py::sum(&{ let mut out = vec![]; for t5468 in py::range_iter(&[(py::len(&v_xs.clone())?).i()?]) { let mut v_k: Py; 
v_k = t5468;
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
    let t5522 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t5522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t5522[0].clone();
    v_ys = t5522[1].clone();
    for t5523 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5523;
        v_F = py::truediv(&py::sub(&py::add(&v_i.clone(), &Py::Int(1i64))?, &Py::Float(0.3))?, &py::add(&v_n.clone(), &Py::Float(0.4))?)?;
        let _ = py::method(&v_xs.clone(), "append", &[py::math1("log", &py::getitem(&v_t.clone(), &v_i.clone())?)?])?;
        let _ = py::method(&v_ys.clone(), "append", &[py::math1("log", &py::neg(&py::math1("log", &py::sub(&Py::Int(1i64), &v_F.clone())?)?)?)?])?;
    }
    let t5524 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t5524.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t5524[0].clone();
    v_my = t5524[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t5525 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t5525;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t5526 = v_sxx.clone(); Py::Bool({ let t5527 = Py::Int(0i64); if py::gt(&t5526, &t5527)? { true } else { false } }) }, Py::str("the failure times must not all be equal"))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t5528 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5529 = (t5528).items()?;
if t5529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5529[0].clone();
v_y = t5529[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t5530 in py::iter(&(v_ys.clone()))? { let mut v_y: Py; 
v_y = t5530;
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
    let t5570 = (Py::tuple(vec![f__pos(Py::str("shape"), v_shape.clone())?, f__pos(Py::str("scale"), v_scale.clone())?])).items()?;
    if t5570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t5570[0].clone();
    v_eta = t5570[1].clone();
    v_g = f__nonneg(Py::str("location"), v_location.clone())?;
    v_t = f__fin(Py::str("time"), v_time.clone())?;
    let _ = f__need({ let t5573 = v_t.clone(); Py::Bool({ let t5574 = v_g.clone(); if py::ge(&t5573, &t5574)? { true } else { false } }) }, Py::str("time must not be before the location (failure-free time)"))?;
    v_x = py::truediv(&py::sub(&v_t.clone(), &v_g.clone())?, &v_eta.clone())?;
    v_R = py::math1("exp", &py::neg(&py::pow(&v_x.clone(), &v_b.clone())?)?)?;
    v_h = (if ({ let t5575 = v_x.clone(); Py::Bool({ let t5576 = Py::Int(0i64); if py::gt(&t5575, &t5576)? { true } else { false } }) }).truthy() { py::mul(&py::truediv(&v_b.clone(), &v_eta.clone())?, &py::pow(&v_x.clone(), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)? } else { (if ({ let t5577 = v_b.clone(); Py::Bool({ let t5578 = Py::Int(1i64); if py::gt(&t5577, &t5578)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { (if ({ let t5579 = v_b.clone(); Py::Bool({ let t5580 = Py::Int(1i64); if py::eq(&t5579, &t5580) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &v_eta.clone())? } else { Py::Float(f64::INFINITY) }) }) });
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
    let _ = f__need({ let t5634 = py::len(&v_w.clone())?; Py::Bool({ let t5635 = py::len(&v_x.clone())?; if py::le(&t5634, &t5635)? { true } else { false } }) }, Py::str("there must be at least as many observations as weights"))?;
    v_tw = py::sum(&v_w.clone(), None)?;
    let _ = f__need({ let t5636 = v_tw.clone(); Py::Bool({ let t5637 = Py::Int(0i64); if py::gt(&t5636, &t5637)? { true } else { false } }) }, Py::str("the weights must not all be 0"))?;
    v_k = py::len(&v_w.clone())?;
    return Ok(f__rd(Py::dict(vec![(Py::str("forecast"), py::truediv(&py::sum(&{ let mut out = vec![]; for t5650 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t5650;
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
    let t5679 = (Py::tuple(vec![f__prob(Py::str("proportion"), v_proportion.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("accuracy"), v_accuracy.clone(), Py::Float(0.0), Py::Float(1.0))?, f__prob(Py::str("confidence"), v_confidence.clone(), Py::Float(0.0), Py::Float(1.0))?])).items()?;
    if t5679.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5679[0].clone();
    v_h = t5679[1].clone();
    v_c = t5679[2].clone();
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
    let t2083 = (Py::tuple(vec![f__pos(Py::str("first_unit_time"), v_first_unit_time.clone())?, f__prob(Py::str("learning_rate"), v_learning_rate.clone(), Py::Float(0.0), py::add(&Py::Float(1.0), &Py::Float(1e-12))?)?])).items()?;
    if t2083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t2083[0].clone();
    v_r = t2083[1].clone();
    v_N = f__int(Py::str("units"), v_units.clone(), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    v_b = f__b(v_r.clone())?;
    v_avg = py::mul(&v_T1.clone(), &py::pow(&v_N.clone(), &v_b.clone())?)?;
    v_tot = py::mul(&v_avg.clone(), &v_N.clone())?;
    v_prev = (if ({ let t2096 = v_N.clone(); Py::Bool({ let t2097 = Py::Int(1i64); if py::gt(&t2096, &t2097)? { true } else { false } }) }).truthy() { py::mul(&v_T1.clone(), &py::pow(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &py::add(&v_b.clone(), &Py::Int(1i64))?)?)? } else { Py::Float(0.0) });
    return Ok(f__rd(Py::dict(vec![(Py::str("cumulative_average_time"), v_avg.clone()), (Py::str("total_time"), v_tot.clone()), (Py::str("unit_time"), py::sub(&v_tot.clone(), &v_prev.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_xbar_r_limits(mut v_grand_mean: Py, mut v_mean_range: Py, mut v_subgroup_size: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let t5721 = (Py::tuple(vec![f__fin(Py::str("grand_mean"), v_grand_mean.clone())?, f__pos(Py::str("mean_range"), v_mean_range.clone())?])).items()?;
    if t5721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t5721[0].clone();
    v_rb = t5721[1].clone();
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
    let t5773 = (Py::tuple(vec![f__fin(Py::str("grand_mean"), v_grand_mean.clone())?, f__pos(Py::str("mean_sd"), v_mean_sd.clone())?])).items()?;
    if t5773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t5773[0].clone();
    v_sb = t5773[1].clone();
    v_n = f__int(Py::str("subgroup_size"), v_subgroup_size.clone(), Py::Int(2i64), Py::Int(10i64))?;
    return Ok(f__rd(Py::dict(vec![(Py::str("xbar_ucl"), py::add(&v_xb.clone(), &py::mul(&py::getitem(&c__A3()?, &v_n.clone())?, &v_sb.clone())?)?), (Py::str("xbar_lcl"), py::sub(&v_xb.clone(), &py::mul(&py::getitem(&c__A3()?, &v_n.clone())?, &v_sb.clone())?)?), (Py::str("center"), v_xb.clone()), (Py::str("s_ucl"), py::mul(&py::getitem(&c__B4()?, &v_n.clone())?, &v_sb.clone())?), (Py::str("s_lcl"), py::mul(&py::getitem(&c__B3()?, &v_n.clone())?, &v_sb.clone())?), (Py::str("A3"), py::getitem(&c__A3()?, &v_n.clone())?), (Py::str("B3"), py::getitem(&c__B3()?, &v_n.clone())?), (Py::str("B4"), py::getitem(&c__B4()?, &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1014(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1036 = v_inputs.clone(); f_eoq_discount(py::getitem(&t1036, &Py::str("demand"))?, py::getitem(&t1036, &Py::str("order_cost"))?, py::getitem(&t1036, &Py::str("holding_rate"))?, py::getitem(&t1036, &Py::str("break_quantities"))?, py::getitem(&t1036, &Py::str("unit_prices"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1079 = v_inputs.clone(); f_epq(py::getitem(&t1079, &Py::str("demand"))?, py::getitem(&t1079, &Py::str("production_rate"))?, py::getitem(&t1079, &Py::str("order_cost"))?, py::getitem(&t1079, &Py::str("holding_cost"))?)? })?);
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
fn s_1109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1131 = v_inputs.clone(); f_staffing(py::getitem(&t1131, &Py::str("arrival_rate"))?, py::getitem(&t1131, &Py::str("service_rate"))?, py::getitem(&t1131, &Py::str("target_time"))?, py::getitem(&t1131, &Py::str("target_level"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1165(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1187 = v_inputs.clone(); f_ewma_limits(py::getitem(&t1187, &Py::str("target"))?, py::getitem(&t1187, &Py::str("sigma"))?, py::getitem(&t1187, &Py::str("smoothing"))?, py::getitem(&t1187, &Py::str("limit_sigma"))?, py::getitem(&t1187, &Py::str("sample_number"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1229 = v_inputs.clone(); f_exponential_reliability(py::getitem(&t1229, &Py::str("failure_rate"))?, py::getitem(&t1229, &Py::str("time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1266(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1288 = v_inputs.clone(); f_exp_smoothing(py::getitem(&t1288, &Py::str("data"))?, py::getitem(&t1288, &Py::str("alpha"))?, py::method(&t1288, "get", &[Py::str("initial_forecast"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1345(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1367 = v_inputs.clone(); f_factor_rating(py::getitem(&t1367, &Py::str("weights"))?, py::getitem(&t1367, &Py::str("scores"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1435 = v_inputs.clone(); f_flow_shop_makespan(py::getitem(&t1435, &Py::str("times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1478(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1500 = v_inputs.clone(); f_forecast_errors(py::getitem(&t1500, &Py::str("actual"))?, py::getitem(&t1500, &Py::str("forecast"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1534(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1556 = v_inputs.clone(); f_fraction_nonconforming(py::getitem(&t1556, &Py::str("mean"))?, py::getitem(&t1556, &Py::str("sd"))?, py::method(&t1556, "get", &[Py::str("lsl"), Py::None])?, py::method(&t1556, "get", &[Py::str("usl"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1605(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1627 = v_inputs.clone(); f_holt(py::getitem(&t1627, &Py::str("data"))?, py::getitem(&t1627, &Py::str("alpha"))?, py::getitem(&t1627, &Py::str("beta"))?, py::getitem(&t1627, &Py::str("initial_level"))?, py::getitem(&t1627, &Py::str("initial_trend"))?, py::method(&t1627, "get", &[Py::str("horizon"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1649(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1671 = v_inputs.clone(); f_inventory_total_cost(py::getitem(&t1671, &Py::str("demand"))?, py::getitem(&t1671, &Py::str("order_cost"))?, py::getitem(&t1671, &Py::str("holding_cost"))?, py::getitem(&t1671, &Py::str("order_quantity"))?, py::method(&t1671, "get", &[Py::str("unit_cost"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1711 = v_inputs.clone(); f_inventory_turnover(py::getitem(&t1711, &Py::str("cost_of_goods_sold"))?, py::getitem(&t1711, &Py::str("average_inventory"))?, py::method(&t1711, "get", &[Py::str("days_per_year"), Py::Int(365i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1761(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1783 = v_inputs.clone(); f_johnson_three(py::getitem(&t1783, &Py::str("machine1_times"))?, py::getitem(&t1783, &Py::str("machine2_times"))?, py::getitem(&t1783, &Py::str("machine3_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1825(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1847 = v_inputs.clone(); f_johnson_two(py::getitem(&t1847, &Py::str("machine1_times"))?, py::getitem(&t1847, &Py::str("machine2_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1881(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1903 = v_inputs.clone(); f_k_out_of_n(py::getitem(&t1903, &Py::str("component_reliability"))?, py::getitem(&t1903, &Py::str("k"))?, py::getitem(&t1903, &Py::str("n"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_191(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t213 = v_inputs.clone(); f_after_tax_cash_flow(py::getitem(&t213, &Py::str("before_tax_cash_flows"))?, py::getitem(&t213, &Py::str("depreciation"))?, py::getitem(&t213, &Py::str("tax_rate"))?, py::method(&t213, "get", &[Py::str("investment"), Py::None])?, py::method(&t213, "get", &[Py::str("sale_price"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1926(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1948 = v_inputs.clone(); f_kanban_count(py::getitem(&t1948, &Py::str("demand_rate"))?, py::getitem(&t1948, &Py::str("lead_time"))?, py::getitem(&t1948, &Py::str("container_size"))?, py::method(&t1948, "get", &[Py::str("safety_factor"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2053(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2075 = v_inputs.clone(); f_largest_candidate(py::getitem(&t2075, &Py::str("task_times"))?, py::getitem(&t2075, &Py::str("precedence"))?, py::getitem(&t2075, &Py::str("cycle_time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2110(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2132 = v_inputs.clone(); f_wright_average(py::getitem(&t2132, &Py::str("first_unit_time"))?, py::getitem(&t2132, &Py::str("learning_rate"))?, py::getitem(&t2132, &Py::str("units"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2199 = v_inputs.clone(); f_learning_fit(py::getitem(&t2199, &Py::str("units"))?, py::getitem(&t2199, &Py::str("times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2255 = v_inputs.clone(); f_learning_cumulative(py::getitem(&t2255, &Py::str("first_unit_time"))?, py::getitem(&t2255, &Py::str("learning_rate"))?, py::getitem(&t2255, &Py::str("units"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2288(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2310 = v_inputs.clone(); f_learning_unit(py::getitem(&t2310, &Py::str("first_unit_time"))?, py::getitem(&t2310, &Py::str("learning_rate"))?, py::getitem(&t2310, &Py::str("unit"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2331(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2353 = v_inputs.clone(); f_learning_two_points(py::getitem(&t2353, &Py::str("unit1"))?, py::getitem(&t2353, &Py::str("time1"))?, py::getitem(&t2353, &Py::str("unit2"))?, py::getitem(&t2353, &Py::str("time2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2427 = v_inputs.clone(); f_line_efficiency(py::getitem(&t2427, &Py::str("task_times"))?, py::getitem(&t2427, &Py::str("cycle_time"))?, py::getitem(&t2427, &Py::str("stations"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t268 = v_inputs.clone(); f_arl_shewhart(py::getitem(&t268, &Py::str("shift"))?, py::method(&t268, "get", &[Py::str("subgroup_size"), Py::Int(1i64)])?, py::method(&t268, "get", &[Py::str("limit_sigma"), Py::Float(3.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2477(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2499 = v_inputs.clone(); f_linear_trend(py::getitem(&t2499, &Py::str("data"))?, py::method(&t2499, "get", &[Py::str("horizon"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2530(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2552 = v_inputs.clone(); f_littles_law(py::method(&t2552, "get", &[Py::str("arrival_rate"), Py::None])?, py::method(&t2552, "get", &[Py::str("number_in_system"), Py::None])?, py::method(&t2552, "get", &[Py::str("time_in_system"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2591(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2613 = v_inputs.clone(); f_location_cost_volume(py::getitem(&t2613, &Py::str("fixed_costs"))?, py::getitem(&t2613, &Py::str("variable_costs"))?, py::getitem(&t2613, &Py::str("volume"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2635(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2657 = v_inputs.clone(); f_md1(py::getitem(&t2657, &Py::str("arrival_rate"))?, py::getitem(&t2657, &Py::str("service_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2681(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2703 = v_inputs.clone(); f_mg1(py::getitem(&t2703, &Py::str("arrival_rate"))?, py::getitem(&t2703, &Py::str("mean_service_time"))?, py::getitem(&t2703, &Py::str("sd_service_time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2723(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2745 = v_inputs.clone(); f_mm1(py::getitem(&t2745, &Py::str("arrival_rate"))?, py::getitem(&t2745, &Py::str("service_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2780(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2802 = v_inputs.clone(); f_mm1k(py::getitem(&t2802, &Py::str("arrival_rate"))?, py::getitem(&t2802, &Py::str("service_rate"))?, py::getitem(&t2802, &Py::str("capacity"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2836(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2858 = v_inputs.clone(); f_mmc(py::getitem(&t2858, &Py::str("arrival_rate"))?, py::getitem(&t2858, &Py::str("service_rate"))?, py::getitem(&t2858, &Py::str("servers"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_286(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t308 = v_inputs.clone(); f_availability(py::getitem(&t308, &Py::str("mtbf"))?, py::getitem(&t308, &Py::str("mttr"))?, py::method(&t308, "get", &[Py::str("hours_per_year"), Py::Float(8760.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2911(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2933 = v_inputs.clone(); f_moore_hodgson(py::getitem(&t2933, &Py::str("processing_times"))?, py::getitem(&t2933, &Py::str("due_dates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2979(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3001 = v_inputs.clone(); f_moving_average(py::getitem(&t3001, &Py::str("data"))?, py::getitem(&t3001, &Py::str("periods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3070(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3092 = v_inputs.clone(); f_mtbf_confidence(py::getitem(&t3092, &Py::str("total_time"))?, py::getitem(&t3092, &Py::str("failures"))?, py::getitem(&t3092, &Py::str("confidence"))?, py::method(&t3092, "get", &[Py::str("test"), Py::str("time")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3135(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3157 = v_inputs.clone(); f_multiproduct_break_even(py::getitem(&t3157, &Py::str("fixed_cost"))?, py::getitem(&t3157, &Py::str("prices"))?, py::getitem(&t3157, &Py::str("variable_costs"))?, py::getitem(&t3157, &Py::str("sales_mix"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3215(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3237 = v_inputs.clone(); f_newsvendor_discrete(py::getitem(&t3237, &Py::str("price"))?, py::getitem(&t3237, &Py::str("cost"))?, py::getitem(&t3237, &Py::str("demands"))?, py::getitem(&t3237, &Py::str("probabilities"))?, py::method(&t3237, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3282 = v_inputs.clone(); f_newsvendor_normal(py::getitem(&t3282, &Py::str("price"))?, py::getitem(&t3282, &Py::str("cost"))?, py::getitem(&t3282, &Py::str("mean_demand"))?, py::getitem(&t3282, &Py::str("sd_demand"))?, py::method(&t3282, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3336 = v_inputs.clone(); f_newsvendor_poisson(py::getitem(&t3336, &Py::str("price"))?, py::getitem(&t3336, &Py::str("cost"))?, py::getitem(&t3336, &Py::str("mean_demand"))?, py::method(&t3336, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_332(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t354 = v_inputs.clone(); f_break_even(py::getitem(&t354, &Py::str("fixed_cost"))?, py::getitem(&t354, &Py::str("price"))?, py::getitem(&t354, &Py::str("variable_cost"))?, py::method(&t354, "get", &[Py::str("target_profit"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3361(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3383 = v_inputs.clone(); f_newsvendor_uniform(py::getitem(&t3383, &Py::str("price"))?, py::getitem(&t3383, &Py::str("cost"))?, py::getitem(&t3383, &Py::str("low"))?, py::getitem(&t3383, &Py::str("high"))?, py::method(&t3383, "get", &[Py::str("salvage"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3414(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3436 = v_inputs.clone(); f_np_chart_limits(py::getitem(&t3436, &Py::str("p_bar"))?, py::getitem(&t3436, &Py::str("sample_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3464(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3486 = v_inputs.clone(); f_oee(py::getitem(&t3486, &Py::str("planned_time"))?, py::getitem(&t3486, &Py::str("downtime"))?, py::getitem(&t3486, &Py::str("ideal_cycle_time"))?, py::getitem(&t3486, &Py::str("total_count"))?, py::getitem(&t3486, &Py::str("good_count"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3512(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3534 = v_inputs.clone(); f_order_up_to(py::getitem(&t3534, &Py::str("mean_demand"))?, py::getitem(&t3534, &Py::str("sd_demand"))?, py::getitem(&t3534, &Py::str("review_period"))?, py::getitem(&t3534, &Py::str("lead_time"))?, py::getitem(&t3534, &Py::str("service_level"))?, py::method(&t3534, "get", &[Py::str("inventory_position"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3615 = v_inputs.clone(); f_p_chart_counts(py::getitem(&t3615, &Py::str("defectives"))?, py::getitem(&t3615, &Py::str("sample_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3646(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3668 = v_inputs.clone(); f_p_chart_limits(py::getitem(&t3668, &Py::str("p_bar"))?, py::getitem(&t3668, &Py::str("sample_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3699(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3721 = v_inputs.clone(); f_parallel_mttf(py::getitem(&t3721, &Py::str("failure_rate"))?, py::getitem(&t3721, &Py::str("n"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_371(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t393 = v_inputs.clone(); f_c_chart_limits(py::getitem(&t393, &Py::str("c_bar"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3757(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3779 = v_inputs.clone(); f_parallel_reliability(py::getitem(&t3779, &Py::str("reliabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3800(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3822 = v_inputs.clone(); f_capability(py::getitem(&t3822, &Py::str("usl"))?, py::getitem(&t3822, &Py::str("lsl"))?, py::getitem(&t3822, &Py::str("mean"))?, py::getitem(&t3822, &Py::str("sd"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3860(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3882 = v_inputs.clone(); f_performance(py::getitem(&t3882, &Py::str("data"))?, py::getitem(&t3882, &Py::str("usl"))?, py::getitem(&t3882, &Py::str("lsl"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3931(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3953 = v_inputs.clone(); f_productivity(py::getitem(&t3953, &Py::str("output"))?, py::getitem(&t3953, &Py::str("input_amounts"))?, py::method(&t3953, "get", &[Py::str("previous_output"), Py::None])?, py::method(&t3953, "get", &[Py::str("previous_inputs"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3997(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4019 = v_inputs.clone(); f_server_cost(py::getitem(&t4019, &Py::str("arrival_rate"))?, py::getitem(&t4019, &Py::str("service_rate"))?, py::getitem(&t4019, &Py::str("server_cost_rate"))?, py::getitem(&t4019, &Py::str("waiting_cost_rate"))?, py::method(&t4019, "get", &[Py::str("max_servers"), Py::Int(50i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4154 = v_inputs.clone(); f_ranked_positional_weight(py::getitem(&t4154, &Py::str("task_times"))?, py::getitem(&t4154, &Py::str("precedence"))?, py::getitem(&t4154, &Py::str("cycle_time"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_415(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t437 = v_inputs.clone(); f_capacity_measures(py::getitem(&t437, &Py::str("design_capacity"))?, py::getitem(&t437, &Py::str("effective_capacity"))?, py::getitem(&t437, &Py::str("actual_output"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4204(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4226 = v_inputs.clone(); f_rectilinear_location(py::getitem(&t4226, &Py::str("x"))?, py::getitem(&t4226, &Py::str("y"))?, py::getitem(&t4226, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4249(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4271 = v_inputs.clone(); f_redundancy_needed(py::getitem(&t4271, &Py::str("target_reliability"))?, py::getitem(&t4271, &Py::str("component_reliability"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4318(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4340 = v_inputs.clone(); f_regression_forecast(py::getitem(&t4340, &Py::str("x"))?, py::getitem(&t4340, &Py::str("y"))?, py::method(&t4340, "get", &[Py::str("x_new"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4360(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4382 = v_inputs.clone(); f_reorder_point(py::getitem(&t4382, &Py::str("demand_rate"))?, py::getitem(&t4382, &Py::str("lead_time"))?, py::method(&t4382, "get", &[Py::str("safety_stock"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4418(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4440 = v_inputs.clone(); f_rolled_yield(py::getitem(&t4440, &Py::str("yields"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4492(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4514 = v_inputs.clone(); f_rq_fill_rate(py::getitem(&t4514, &Py::str("mean_ltd"))?, py::getitem(&t4514, &Py::str("sd_ltd"))?, py::getitem(&t4514, &Py::str("order_quantity"))?, py::getitem(&t4514, &Py::str("fill_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4536(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4558 = v_inputs.clone(); f_safety_stock_normal(py::getitem(&t4558, &Py::str("mean_demand"))?, py::getitem(&t4558, &Py::str("sd_demand"))?, py::getitem(&t4558, &Py::str("lead_time"))?, py::getitem(&t4558, &Py::str("service_level"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4587(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4609 = v_inputs.clone(); f_safety_stock_vlt(py::getitem(&t4609, &Py::str("mean_demand"))?, py::getitem(&t4609, &Py::str("sd_demand"))?, py::getitem(&t4609, &Py::str("mean_lead_time"))?, py::getitem(&t4609, &Py::str("sd_lead_time"))?, py::getitem(&t4609, &Py::str("service_level"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4656(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4678 = v_inputs.clone(); f_seasonal_index(py::getitem(&t4678, &Py::str("data"))?, py::method(&t4678, "get", &[Py::str("next_year_total"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4741 = v_inputs.clone(); f_series_parallel(py::getitem(&t4741, &Py::str("stage_reliabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4777(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4799 = v_inputs.clone(); f_series_reliability(py::getitem(&t4799, &Py::str("reliabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_479(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t501 = v_inputs.clone(); f_center_of_gravity(py::getitem(&t501, &Py::str("x"))?, py::getitem(&t501, &Py::str("y"))?, py::getitem(&t501, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4819(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4841 = v_inputs.clone(); f_service_from_rop(py::getitem(&t4841, &Py::str("mean_ltd"))?, py::getitem(&t4841, &Py::str("sd_ltd"))?, py::getitem(&t4841, &Py::str("reorder_point"))?, py::method(&t4841, "get", &[Py::str("order_quantity"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4871(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4893 = v_inputs.clone(); f_sigma_from_range(py::getitem(&t4893, &Py::str("mean_range"))?, py::getitem(&t4893, &Py::str("subgroup_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4939(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4961 = v_inputs.clone(); f_spt_sequence(py::getitem(&t4961, &Py::str("processing_times"))?, py::method(&t4961, "get", &[Py::str("due_dates"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5002(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5024 = v_inputs.clone(); f_standard_time(py::getitem(&t5024, &Py::str("observed_times"))?, py::getitem(&t5024, &Py::str("rating"))?, py::getitem(&t5024, &Py::str("allowance"))?, py::getitem(&t5024, &Py::str("basis"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5058(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5080 = v_inputs.clone(); f_standby_redundancy(py::getitem(&t5080, &Py::str("failure_rate"))?, py::getitem(&t5080, &Py::str("time"))?, py::method(&t5080, "get", &[Py::str("units"), Py::Int(2i64)])?)? })?);
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
fn s_5115(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5137 = v_inputs.clone(); f_system_failure_rate(py::getitem(&t5137, &Py::str("failure_rates"))?, py::method(&t5137, "get", &[Py::str("time"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5206 = v_inputs.clone(); f_tabular_cusum(py::getitem(&t5206, &Py::str("data"))?, py::getitem(&t5206, &Py::str("target"))?, py::getitem(&t5206, &Py::str("k"))?, py::getitem(&t5206, &Py::str("h"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5228(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5250 = v_inputs.clone(); f_takt_time(py::getitem(&t5250, &Py::str("available_time"))?, py::getitem(&t5250, &Py::str("demand"))?, py::method(&t5250, "get", &[Py::str("total_task_time"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_524(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t546 = v_inputs.clone(); f_cpm_index(py::getitem(&t546, &Py::str("usl"))?, py::getitem(&t546, &Py::str("lsl"))?, py::getitem(&t546, &Py::str("mean"))?, py::getitem(&t546, &Py::str("sd"))?, py::getitem(&t546, &Py::str("target"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5274(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5296 = v_inputs.clone(); f_time_study_size(py::getitem(&t5296, &Py::str("mean"))?, py::getitem(&t5296, &Py::str("sd"))?, py::getitem(&t5296, &Py::str("accuracy"))?, py::getitem(&t5296, &Py::str("confidence"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5346(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5368 = v_inputs.clone(); f_tracking_signal(py::getitem(&t5368, &Py::str("actual"))?, py::getitem(&t5368, &Py::str("forecast"))?, py::method(&t5368, "get", &[Py::str("limit"), Py::Float(4.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5408 = v_inputs.clone(); f_u_chart_limits(py::getitem(&t5408, &Py::str("u_bar"))?, py::getitem(&t5408, &Py::str("units"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5481(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5503 = v_inputs.clone(); f_weber_location(py::getitem(&t5503, &Py::str("x"))?, py::getitem(&t5503, &Py::str("y"))?, py::getitem(&t5503, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5543(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5565 = v_inputs.clone(); f_weibull_regression(py::getitem(&t5565, &Py::str("failure_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5615 = v_inputs.clone(); f_weibull_reliability(py::getitem(&t5615, &Py::str("shape"))?, py::getitem(&t5615, &Py::str("scale"))?, py::getitem(&t5615, &Py::str("time"))?, py::method(&t5615, "get", &[Py::str("location"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5651(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5673 = v_inputs.clone(); f_weighted_moving_average(py::getitem(&t5673, &Py::str("data"))?, py::getitem(&t5673, &Py::str("weights"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5694(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5716 = v_inputs.clone(); f_work_sampling_size(py::getitem(&t5716, &Py::str("proportion"))?, py::getitem(&t5716, &Py::str("accuracy"))?, py::getitem(&t5716, &Py::str("confidence"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5746(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5768 = v_inputs.clone(); f_xbar_r_limits(py::getitem(&t5768, &Py::str("grand_mean"))?, py::getitem(&t5768, &Py::str("mean_range"))?, py::getitem(&t5768, &Py::str("subgroup_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5798(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t5820 = v_inputs.clone(); f_xbar_s_limits(py::getitem(&t5820, &Py::str("grand_mean"))?, py::getitem(&t5820, &Py::str("mean_sd"))?, py::getitem(&t5820, &Py::str("subgroup_size"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_610(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t632 = v_inputs.clone(); f_depreciation_schedule(py::getitem(&t632, &Py::str("cost"))?, py::getitem(&t632, &Py::str("life"))?, py::getitem(&t632, &Py::str("method"))?, py::method(&t632, "get", &[Py::str("salvage"), Py::None])?, py::method(&t632, "get", &[Py::str("factor"), Py::None])?, py::method(&t632, "get", &[Py::str("year"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_688(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t710 = v_inputs.clone(); f_dispatch_rule(py::getitem(&t710, &Py::str("processing_times"))?, py::getitem(&t710, &Py::str("due_dates"))?, py::getitem(&t710, &Py::str("rule"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_733(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t755 = v_inputs.clone(); f_dpmo_sigma(py::getitem(&t755, &Py::str("defects"))?, py::getitem(&t755, &Py::str("units"))?, py::getitem(&t755, &Py::str("opportunities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t806 = v_inputs.clone(); f_earned_value_analysis(py::getitem(&t806, &Py::str("planned_value"))?, py::getitem(&t806, &Py::str("earned_value"))?, py::getitem(&t806, &Py::str("actual_cost"))?, py::method(&t806, "get", &[Py::str("budget_at_completion"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_852(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t874 = v_inputs.clone(); f_edd_sequence(py::getitem(&t874, &Py::str("processing_times"))?, py::getitem(&t874, &Py::str("due_dates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t914 = v_inputs.clone(); f_eoq(py::getitem(&t914, &Py::str("demand"))?, py::getitem(&t914, &Py::str("order_cost"))?, py::getitem(&t914, &Py::str("holding_cost"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_932(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t954 = v_inputs.clone(); f_eoq_backorders(py::getitem(&t954, &Py::str("demand"))?, py::getitem(&t954, &Py::str("order_cost"))?, py::getitem(&t954, &Py::str("holding_cost"))?, py::getitem(&t954, &Py::str("backorder_cost"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
