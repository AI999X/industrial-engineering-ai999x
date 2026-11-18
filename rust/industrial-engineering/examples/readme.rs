//! Industrial Engineering and Supply Chain: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p industrial-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&industrial_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "ie_break_even", "inputs": {"fixed_cost": "10000", "price": "25", "variable_cost": "15"}}));
    println!();
    show(json!({"key": "ie_after_tax_cash_flow", "inputs": {"before_tax_cash_flows": [5000, 5000, 5000], "depreciation": [3000, 3000, 3000], "tax_rate": "0.25", "investment": "9000", "sale_price": "1000"}}));
    println!();
    show(json!({"key": "ie_center_of_gravity", "inputs": {"x": [30, 90, 130, 60], "y": [120, 110, 130, 40], "weights": [2000, 1000, 1000, 2000]}}));
    println!();
    show(json!({"key": "ie_regression_forecast", "inputs": {"x": [1, 3, 4, 2, 1, 7], "y": [2, 3, 2.5, 2, 2, 3.5], "x_new": "6"}}));
    println!();
    show(json!({"key": "ie_rq_fill_rate", "inputs": {"mean_ltd": "500", "sd_ltd": "100", "order_quantity": "1000", "fill_rate": "0.99"}}));
    println!();
    show(json!({"key": "ie_learning_curve_cumulative_average", "inputs": {"first_unit_time": "100", "learning_rate": "0.8", "units": "4"}}));
    println!();
    show(json!({"key": "ie_line_efficiency", "inputs": {"task_times": [0.5, 1.2, 0.8, 0.4, 1, 0.6], "cycle_time": "1.5", "stations": "4"}}));
    println!();
    show(json!({"key": "ie_earned_value", "inputs": {"planned_value": "500", "earned_value": "400", "actual_cost": "450", "budget_at_completion": "1000"}}));
    println!();
    show(json!({"key": "ie_queue_server_cost", "inputs": {"arrival_rate": "20", "service_rate": "6", "server_cost_rate": "25", "waiting_cost_rate": "40"}}));
    println!();
    show(json!({"key": "ie_standby_redundancy", "inputs": {"failure_rate": "0.001", "time": "1000"}}));
    println!();
    show(json!({"key": "ie_edd_sequence", "inputs": {"processing_times": [4, 7, 2, 5], "due_dates": [10, 15, 5, 12]}}));
    println!();
    show(json!({"key": "ie_arl_shewhart", "inputs": {"shift": "1", "subgroup_size": "5"}}));
    println!();
    show(json!({"key": "ie_capacity_utilization", "inputs": {"design_capacity": "1200", "effective_capacity": "1000", "actual_output": "800"}}));
    println!();
    show(json!({"key": "ie_multiproduct_break_even", "inputs": {"fixed_cost": "3000", "prices": [2.95, 1.25, 0.8], "variable_costs": [1.25, 0.3, 0.2], "sales_mix": [0.5, 0.3, 0.2]}}));
    println!();
    show(json!({"key": "ie_depreciation_schedule", "inputs": {"cost": "10000", "salvage": "2000", "life": "5", "method": "straight-line"}}));
}
