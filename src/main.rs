use std::collections::BTreeMap;
use std::io;
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types::Formula;
use xlformula_engine::types::Value;
use xlformula_engine::NoCustomFunction;
use xlformula_engine::NoReference;

fn scales_of_temperature() -> BTreeMap<String, [(String, String); 4]> {
    let mut scales: BTreeMap<String, [(String, String); 4]> = BTreeMap::new();
    scales.insert(
        String::from("Celsius"),
        [
            (String::from("Kelvin"), String::from("=(x+273.15)")),
            (String::from("Fahrenheit"), String::from("=(x*9/5+32)")),
            (String::from("Rankine"), String::from("=((x+273.15)*9/5)")),
            (String::from("abbreviation"), String::from("C")),
        ],
    );
    scales.insert(
        String::from("Kelvin"),
        [
            (String::from("Celsius"), String::from("=(x-273.15)")),
            (String::from("Fahrenheit"), String::from("=(x*9/5-459.67)")),
            (String::from("Rankine"), String::from("=(x*9/5)")),
            (String::from("abbreviation"), String::from("K")),
        ],
    );
    scales.insert(
        String::from("Fahrenheit"),
        [
            (String::from("Celsius"), String::from("=((x-32)*5/9)")),
            (String::from("Kelvin"), String::from("=((x+459.67)*5/9)")),
            (String::from("Rankine"), String::from("=(x+459.67)")),
            (String::from("abbreviation"), String::from("F")),
        ],
    );
    scales.insert(
        String::from("Rankine"),
        [
            (String::from("Celsius"), String::from("=((x-491.67)*5/9)")),
            (String::from("Kelvin"), String::from("=(x*5/9)")),
            (String::from("Fahrenheit"), String::from("=(x-459.67)")),
            (String::from("abbreviation"), String::from("R")),
        ],
    );

   scales
}

fn get_unit_scale(
    unit_name: String,
    scales_map: fn() -> BTreeMap<String, [(String, String); 4]>,
) -> BTreeMap<String, String> {
    BTreeMap::from(scales_map().get(&unit_name).unwrap().to_owned())
}

fn get_unit_abbreviation(
    unit_name: String,
    unit_scale_map: fn(
        String,
        fn() -> BTreeMap<String, [(String, String); 4]>,
    ) -> BTreeMap<String, String>,
    unit_scale_map_unit_name: String,
    scales_map: fn() -> BTreeMap<String, [(String, String); 4]>,
) -> String {
    if let Some(abbr) = unit_scale_map(unit_scale_map_unit_name, scales_map).get(&unit_name) {
        abbr.to_string()
    } else {
        String::from("failure")
    }
}

fn get_scales(scales_map: fn() -> BTreeMap<String, [(String, String); 4]>) -> Vec<String> {
    let scales: Vec<String> = scales_map().into_keys().collect();
    scales
}

#[derive(Debug)]
struct TemperatureInformation {
    temp_amount: f32,
    unit_of_temperature: String,
    unit_of_temperature_abbreviation: String,
    target_unit_of_temperature: Option<String>,
}

impl TemperatureInformation {
    fn instatiate_temperature_information(
        temp_amount: f32,
        unit_of_temperature: String,
        target_unit_of_temperature: String,
    ) -> Self {
        Self {
            temp_amount,
            unit_of_temperature: unit_of_temperature.to_string(),
            unit_of_temperature_abbreviation: get_unit_abbreviation(
                String::from("abbreviation"),
                get_unit_scale,
                unit_of_temperature.to_string(),
                scales_of_temperature,
            ),
            target_unit_of_temperature: Some(target_unit_of_temperature.to_string()),
        }
    }
    fn instatiate_final_temperature_information(
        temp_amount: f32,
        unit_of_temperature: String,
    ) -> Self {
        Self {
            temp_amount,
            unit_of_temperature: unit_of_temperature.to_string(),
            unit_of_temperature_abbreviation: get_unit_abbreviation(
                String::from("abbreviation"),
                get_unit_scale,
                unit_of_temperature.to_string(),
                scales_of_temperature,
            ),
            target_unit_of_temperature: None,
        }
    }
}

fn enter_temperature() -> f32 {
    loop{
    println!("Enter Temperature");
    let mut temperature: String = String::new();
    
        io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = match temperature.trim().parse::<f32>() {
        Ok(num) =>  num,
        Err(_) => {
            println!("Invalid Number");
            continue;
        },
    };
        break temperature
    }    
}

fn display_temperature_units_list(user_prompt: &str) -> String {
    
    loop{
    println!("{user_prompt}");
    let temp_units: Vec<String> = get_scales(scales_of_temperature);
    for (i, el) in temp_units.iter().enumerate() {
        println!("{i}: {el}");
    }
    let mut unit_selection: String = String::new();
    io::stdin()
        .read_line(&mut unit_selection)
        .expect("Failed to read line");
    let unit_selection: usize = match unit_selection.trim().parse() {
        // TODO: only allow # from selection
        Ok(num) if num <= (temp_units.len() - 1) => num,
        Ok(_) => {
            println!("Invalid Selection");
            continue;
        }
        Err(_) => {
            println!("Invalid Selection");
            continue;
        },
    };
    let unit_of_temperature: String = get_scales(scales_of_temperature)[unit_selection].to_string();
    break unit_of_temperature
}
}

fn evaluate_expressions(formula: &str, variable: f32) -> f32 {
    // todo finish expression evaluator 
    let expression = formula.replace("x", &variable.to_string());
    let formula: Formula =
        parse_formula::parse_string_to_formula(&expression, None::<NoCustomFunction>);
    let result: Value = calculate::calculate_formula(formula, None::<NoReference>);
    let result_float = calculate::result_to_string(result).parse::<f32>();
    if let Ok(float) = result_float{
        float
    }else{
        println!("Something is wrong with this thing");
        println!("Result_Float:{:?}", result_float);
        panic!();
    }
}

fn convert_temp(
    temp_data: TemperatureInformation,
    scales_map: fn() -> BTreeMap<String, [(String, String); 4]>,
) -> TemperatureInformation {
    let unit_scale: BTreeMap<String, String> =
        get_unit_scale(temp_data.unit_of_temperature, scales_map);
    let target_unit_expression: String = unit_scale
        .get(temp_data.target_unit_of_temperature.as_ref().unwrap())
        .unwrap()
        .to_owned();
    let final_temp_amount: f32 =
        evaluate_expressions(&target_unit_expression, temp_data.temp_amount);
    TemperatureInformation::instatiate_final_temperature_information(
        final_temp_amount,
        temp_data
            .target_unit_of_temperature
            .as_ref()
            .unwrap()
            .to_string(),
    )
}

fn display_tempurerature_conversion(temp_data: TemperatureInformation){
    println!(
        "Converted Initial Temperature {0}°{1} to Final Temperature {2}°{3}",
        temp_data.temp_amount,
        temp_data.unit_of_temperature_abbreviation,
        temp_data.temp_amount,
        temp_data.unit_of_temperature_abbreviation
    );
}
fn main() {
    let initial_temp_data: TemperatureInformation =
        TemperatureInformation::instatiate_temperature_information(
            enter_temperature(),
            display_temperature_units_list("Select Current Unit of Temperature"),
            display_temperature_units_list("Select Target Unit of Temperature"),
        );
    display_tempurerature_conversion(convert_temp(initial_temp_data, scales_of_temperature));
}