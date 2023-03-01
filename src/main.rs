#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// TODO Consider rewriting with traits and structs so that functionality can be easily mocked
// TODO Create Macro for test writing
// TODO write tests
// TODO make mocks available for testing
// TODO rewrite tests to mock imports other than what is being tested
// TODO refactor panics into errors
// TODO keep an eye on benchmarking tests and see if you can create them
// TODO refactor into library
mod calculator;
mod convert_temperature;
mod scales_of_temperature;
mod shunting_yard_algorithm;
mod temperature_information;
mod tokenizer;
mod user_interface;

fn main() {
    let degrees = user_interface::enter_temperature();
    let initial_unit_temperature =
        &user_interface::display_temperature_units_list("Select Current Unit of Temperature", None);
    let target_unit_temperature = &user_interface::display_temperature_units_list(
        "Select Target Unit of Temperature",
        Some(initial_unit_temperature),
    );
    user_interface::display_temperature_conversion(
        &temperature_information::TemperatureInformation::new(
            degrees,
            initial_unit_temperature,
            target_unit_temperature,
        ),
    );
}
