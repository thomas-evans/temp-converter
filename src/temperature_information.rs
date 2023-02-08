use crate::convert_temperature;
use crate::scales_of_temperature;

pub struct TemperatureInformation {
    pub initial_degrees: f32,
    _initial_unit_of_temperature: String,
    pub initial_unit_of_temperature_abbreviation: String,
    pub final_degrees: f32,
    _final_unit_of_temperature: String,
    pub final_unit_of_temperature_abbreviation: String,
}

impl TemperatureInformation {
    pub fn new(
        initial_degrees: f32,
        initial_unit_of_temperature: String,
        final_unit_of_temperature: String,
    ) -> Self {
        Self {
            initial_degrees,
            _initial_unit_of_temperature: initial_unit_of_temperature.to_string(),
            initial_unit_of_temperature_abbreviation: scales_of_temperature::get_scale(
                &initial_unit_of_temperature,
            )
            .abbreviation,
            final_degrees: convert_temperature::convert_temp(
                &initial_degrees,
                &initial_unit_of_temperature,
                &final_unit_of_temperature,
            ),
            _final_unit_of_temperature: final_unit_of_temperature.to_string(),
            final_unit_of_temperature_abbreviation: scales_of_temperature::get_scale(
                &final_unit_of_temperature,
            )
            .abbreviation,
        }
    }
}
