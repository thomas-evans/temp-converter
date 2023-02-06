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

pub fn display_temperature_units_list(user_prompt: &str) -> String {
    loop {
        println!("{user_prompt}");
        let temp_units: Vec<String> =
            scales_of_temperature::get_scales(scales_of_temperature::scales_of_temperature);
        for (i, el) in temp_units.iter().enumerate() {
            println!("{i}: {el}");
        }
        let mut unit_selection: String = String::new();
        io::stdin()
            .read_line(&mut unit_selection)
            .expect("Failed to read line");
        let unit_selection: usize = match unit_selection.trim().parse() {
            // TODO only allow # from selection
            Ok(num) if num <= (temp_units.len() - 1) => num,
            Ok(_) => {
                println!("Invalid Selection");
                continue;
            }
            Err(_) => {
                println!("Invalid Selection");
                continue;
            }
        };
        let unit_of_temperature: String =
            scales_of_temperature::get_scales(scales_of_temperature::scales_of_temperature)
                [unit_selection]
                .to_string();
        break unit_of_temperature;
    }
}

pub fn display_temperature_conversion(temp_data: temperature_information::TemperatureInformation) {
    println!(
        "Converted Initial Temperature {0}°{1} to Final Temperature {2}°{3}",
        temp_data.initial_temp_amount,
        temp_data.initial_unit_of_temperature_abbreviation,
        temp_data.final_temperature_amount,
        temp_data.final_unit_of_temperature_abbreviation
    );
}
