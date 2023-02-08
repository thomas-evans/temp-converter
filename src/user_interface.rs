use crate::scales_of_temperature;
use crate::temperature_information;
use std::io;

pub fn enter_temperature() -> f32 {
    loop {
        println!("Enter Temperature");
        let mut temperature: String = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Number");
                continue;
            }
        };
        break temperature;
    }
}
// TODO split into two functions (chaining?)
pub fn display_temperature_units_list(user_prompt: &str) -> String {
    loop {
        println!("{user_prompt}");
        let scales: Vec<scales_of_temperature::Scale> = scales_of_temperature::compose_scales();
        for (i, el) in scales.iter().enumerate() {
            println!("{i}: {0}", el.scale_name);
        }
        let mut unit_selection: String = String::new();
        io::stdin()
            .read_line(&mut unit_selection)
            .expect("Failed to read line");
        let unit_selection: usize = match unit_selection.trim().parse() {
            // TODO only allow # from selection
            // TODO do not allow the same selection
            Ok(num) if num <= (scales.len() - 1) => num,
            Ok(_) => {
                println!("Invalid Selection");
                continue;
            }
            Err(_) => {
                println!("Invalid Selection");
                continue;
            }
        };
        let unit_of_temperature: String = scales[unit_selection].scale_name.to_string();
        break unit_of_temperature;
    }
}

pub fn display_temperature_conversion(temp_data: temperature_information::TemperatureInformation) {
    println!(
        "Converted Initial Temperature {0}°{1} to Final Temperature {2}°{3}",
        temp_data.initial_degrees,
        temp_data.initial_unit_of_temperature_abbreviation,
        temp_data.final_degrees,
        temp_data.final_unit_of_temperature_abbreviation
    );
}
