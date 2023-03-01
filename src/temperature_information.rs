use crate::convert_temperature;
use crate::scales_of_temperature;
pub struct TemperatureInformation {
    pub initial_degrees: f32,
    pub initial_unit_of_temperature: String,
    pub initial_unit_of_temperature_abbreviation: String,
    pub final_degrees: f32,
    pub final_unit_of_temperature: String,
    pub final_unit_of_temperature_abbreviation: String,
}

impl TemperatureInformation {
    pub fn new(
        initial_degrees: f32,
        initial_unit_of_temperature: &str,
        final_unit_of_temperature: &str,
    ) -> Self {
        Self {
            initial_degrees,
            initial_unit_of_temperature: initial_unit_of_temperature.to_string(),
            initial_unit_of_temperature_abbreviation: scales_of_temperature::get_scale(
                initial_unit_of_temperature,
            )
            .abbreviation,
            final_degrees: convert_temperature::convert_temp(
                initial_degrees,
                initial_unit_of_temperature,
                final_unit_of_temperature,
            ),
            final_unit_of_temperature: final_unit_of_temperature.to_string(),
            final_unit_of_temperature_abbreviation: scales_of_temperature::get_scale(
                final_unit_of_temperature,
            )
            .abbreviation,
        }
    }
}

#[cfg(test)]

mod new_temperature_information {
    use super::TemperatureInformation;
    // it should return a TemperatureInformation struct when given the correct information
    #[test]
    #[ignore = "pollutes test coverage"]
    fn new_temperature_information_given_correct_args_return_temperature_information_struct() {
        let temp_info = TemperatureInformation::new(32.0, "Fahrenheit", "Celsius");
        assert!((temp_info.initial_degrees - 32.0).abs() < f32::EPSILON);
        assert!(temp_info.initial_unit_of_temperature == "Fahrenheit");
        assert!(temp_info.initial_unit_of_temperature_abbreviation == "F");
        assert!((temp_info.final_degrees - 0.0).abs() < f32::EPSILON);
        assert!(temp_info.final_unit_of_temperature == "Celsius");
        assert!(temp_info.final_unit_of_temperature_abbreviation == "C");
    }
}
