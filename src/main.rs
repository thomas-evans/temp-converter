// TODO Analyze ownership

pub mod convert_temperature;
pub mod scales_of_temperature;
pub mod temperature_information;
pub mod user_interface;
fn main() {
    user_interface::display_temperature_conversion(
        temperature_information::TemperatureInformation::new(
            user_interface::enter_temperature(),
            user_interface::display_temperature_units_list("Select Current Unit of Temperature"),
            user_interface::display_temperature_units_list("Select Target Unit of Temperature"),
        ),
    );
}
