extern crate xlformula_engine;
use crate::scales_of_temperature;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types::Formula;
use xlformula_engine::types::Value;
use xlformula_engine::NoCustomFunction;
use xlformula_engine::NoReference;
// TODO rewrite without import using .chars().enumerate()
fn evaluate_expressions(formula: String, variable: &f32) -> f32 {
    let expression = formula.replace("x", &variable.to_string());
    let formula: Formula =
        parse_formula::parse_string_to_formula(&expression, None::<NoCustomFunction>);
    let result: Value = calculate::calculate_formula(formula, None::<NoReference>);
    calculate::result_to_string(result)
        .parse::<f32>()
        .expect("expression result could not be parsed into f32")
}

pub fn convert_temp(
    initial_temp_amount: &f32,
    initial_unit_of_temperature: &String,
    target_unit_of_temperature: &String,
) -> f32 {
    let formula = scales_of_temperature::get_expression(
        &initial_unit_of_temperature.to_string(),
        &target_unit_of_temperature.to_string(),
    );
    evaluate_expressions(formula, initial_temp_amount)
}
