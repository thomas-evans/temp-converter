use crate::convert_temperature;
use crate::scales_of_temperature;

pub struct TemperatureInformation {
    pub initial_temp_amount: f32,
    _initial_unit_of_temperature: String,
    pub initial_unit_of_temperature_abbreviation: String,
    pub final_temperature_amount: f32,
    _final_unit_of_temperature: String,
    pub final_unit_of_temperature_abbreviation: String,
}

impl TemperatureInformation {
    pub fn new(
        initial_temp_amount: f32,
        initial_unit_of_temperature: String,
        final_unit_of_temperature: String,
    ) -> Self {
        Self {
            initial_temp_amount,
            _initial_unit_of_temperature: initial_unit_of_temperature.to_string(),
            initial_unit_of_temperature_abbreviation: scales_of_temperature::get_unit_abbreviation(
                String::from("abbreviation"),
                scales_of_temperature::get_unit_scale,
                initial_unit_of_temperature.to_string(),
                scales_of_temperature::scales_of_temperature,
            ),
            final_temperature_amount: convert_temperature::convert_temp(
                &initial_temp_amount,
                &initial_unit_of_temperature,
                &final_unit_of_temperature,
                scales_of_temperature::scales_of_temperature,
            ),
            _final_unit_of_temperature: final_unit_of_temperature.to_string(),
            final_unit_of_temperature_abbreviation: scales_of_temperature::get_unit_abbreviation(
                String::from("abbreviation"),
                scales_of_temperature::get_unit_scale,
                final_unit_of_temperature.to_string(),
                scales_of_temperature::scales_of_temperature,
            ),
        }
    }
}
