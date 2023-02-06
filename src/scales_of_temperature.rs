// TODO convert scales_of_temperature into a struct or enum or both

use std::collections::BTreeMap;
pub fn scales_of_temperature() -> BTreeMap<String, [(String, String); 4]> {
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

pub fn get_unit_scale(
    unit_name: String,
    scales_map: fn() -> BTreeMap<String, [(String, String); 4]>,
) -> BTreeMap<String, String> {
    if let Some(scale) = scales_map().get(&unit_name) {
        BTreeMap::from(scale.to_owned())
    } else {
        panic!();
    }
}

pub fn get_unit_abbreviation(
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
        panic!();
    }
}

pub fn get_scales(scales_map: fn() -> BTreeMap<String, [(String, String); 4]>) -> Vec<String> {
    let scales: Vec<String> = scales_map().into_keys().collect();
    scales
}
