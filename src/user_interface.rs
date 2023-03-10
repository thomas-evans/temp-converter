use crate::scales_of_temperature;
use crate::temperature_information;
use std::io;

pub fn enter_temperature() -> f32 {
    loop {
        println!("Input Degrees and Press Enter");
        let mut temperature: String = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        if let Ok(n) = temperature.trim().parse::<f32>() {
            break n;
        }
        println!("Invalid Number, try something like 32");
    }
}
pub fn display_temperature_units_list(
    user_prompt: &str,
    initial_temp_type: Option<&str>,
) -> String {
    loop {
        println!("{user_prompt}");

        let scales = scales_of_temperature::get_scales(initial_temp_type);
        for (i, el) in scales.iter().enumerate() {
            println!("{i}: {el}");
        }
        let mut unit_selection: String = String::new();
        io::stdin()
            .read_line(&mut unit_selection)
            .expect("Failed to read line");
        let unit_selection: usize = match unit_selection.trim().parse() {
            Ok(num) if num <= (scales.len() - 1) => num,
            Ok(_) | Err(_) => {
                println!("Invalid Selection");
                continue;
            }
        };
        let unit_of_temperature = scales[unit_selection].to_string();
        break unit_of_temperature;
    }
}

pub fn display_temperature_conversion(temp_data: &temperature_information::TemperatureInformation) {
    println!(
        "Converted Initial Temperature {0}°{1} to Final Temperature {2}°{3}",
        temp_data.initial_degrees,
        temp_data.initial_unit_of_temperature_abbreviation,
        temp_data.final_degrees,
        temp_data.final_unit_of_temperature_abbreviation
    );
}
