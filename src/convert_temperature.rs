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
    // TODO write test to see how accurate conversion is on either side of the decimal
    #[test]
    #[ignore = "pollutes test coverage"]
    fn correct_conversion_result() {
        assert!(
            (convert_temp(32.0, &String::from("Fahrenheit"), &String::from("Celsius")) - 0.0).abs()
                < f32::EPSILON,
            "32F should be 0C"
        );
        assert!(
            (convert_temp(212.0, &String::from("Fahrenheit"), &String::from("Celsius")) - 100.0)
                .abs()
                < f32::EPSILON,
            "212F should be 100C"
        );
    }
}
