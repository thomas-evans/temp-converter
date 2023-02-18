use crate::calculator;
use crate::scales_of_temperature;

pub fn convert_temp(
    initial_temp_amount: f32,
    initial_unit_of_temperature: &str,
    target_unit_of_temperature: &str,
) -> f32 {
    let formula = scales_of_temperature::get_expression(
        initial_unit_of_temperature,
        &target_unit_of_temperature.to_string(),
    );
    calculator::evaluate_expression(&formula, initial_temp_amount)
}

#[cfg(test)]
mod convert_temp {
    use super::convert_temp;
    #[test]
    fn correct_conversion_result() {
        let error_margin = f32::EPSILON;
        let convert_temp_result =
            convert_temp(32.0, &String::from("Fahrenheit"), &String::from("Celsius"));
        assert!(
            (convert_temp_result - 0.0).abs() < error_margin,
            "32F to C should be zero - convert_temp"
        );
    }
}
