#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// TODO write tests
// TODO keep an eye on benchmarking tests
// TODO refactor into library
mod calculator;
mod convert_temperature;
mod scales_of_temperature;
mod shunting_yard_algorithm;
mod temperature_information;
mod tokenizer;
mod user_interface;

fn main() {
    user_interface::display_temperature_conversion(
        &temperature_information::TemperatureInformation::new(
            user_interface::enter_temperature(),
            &user_interface::display_temperature_units_list("Select Current Unit of Temperature"),
            &user_interface::display_temperature_units_list("Select Target Unit of Temperature"),
        ),
    );
}
