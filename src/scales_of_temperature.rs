use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ScaleEntry {
    TargetScaleName(String),
    Expression(String),
}

impl fmt::Display for ScaleEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaleEntry::TargetScaleName(x) => write!(f, "{x}"),
            ScaleEntry::Expression(x) => write!(f, "{x}"),
        }
    }
}
#[derive(Debug)]
pub struct Scale {
    pub scale_name: String,
    pub scale_entries: Vec<(ScaleEntry, ScaleEntry)>,
    pub abbreviation: String,
}

impl Scale {
    pub fn new(scale_name: &str, scale_entry: [(&str, &str); 3], abbreviation: &str) -> Self {
        Self {
            scale_name: scale_name.to_string(),
            scale_entries: scale_entry[..]
                .into_iter()
                .map(|x| {
                    (
                        ScaleEntry::TargetScaleName(x.0.to_string()),
                        ScaleEntry::Expression(x.1.to_string()),
                    )
                })
                .collect(),
            abbreviation: abbreviation.to_string(),
        }
    }
}
//TODO change this into a file i.o
// TODO multilingual?
pub fn compose_scales() -> Vec<Scale> {
    let mut scales_of_temperature: Vec<Scale> = Vec::new();
    scales_of_temperature.push(Scale::new(
        "Celsius",
        [
            ("Kelvin", "=(x+273.15)"),
            ("Fahrenheit", "=(x*9/5+32)"),
            ("Rankine", "=((x+273.15)*9/5)"),
        ],
        "C",
    ));
    scales_of_temperature.push(Scale::new(
        "Kelvin",
        [
            ("Celsius", "=(x-273.15)"),
            ("Fahrenheit", "=(x*9/5-459.67)"),
            ("Rankine", "=(x*9/5)"),
        ],
        "K",
    ));
    scales_of_temperature.push(Scale::new(
        "Fahrenheit",
        [
            ("Celsius", "=((x-32)*5/9)"),
            ("Kelvin", "=((x+459.67)*5/9)"),
            ("Rankine", "=(x+459.67)"),
        ],
        "F",
    ));
    scales_of_temperature.push(Scale::new(
        "Rankine",
        [
            ("Celsius", "=((x-491.67)*5/9)"),
            ("Kelvin", "=(x*5/9)"),
            ("Fahrenheit", "=(x-459.67)"),
        ],
        "R",
    ));
    scales_of_temperature
}

pub fn get_scale(scale_name: &String) -> Scale {
    if let Some(scale) = compose_scales()
        .into_iter()
        .find(|x| x.scale_name == scale_name.as_ref())
    {
        scale
    } else {
        panic!("scale does not exist");
    }
}

pub fn get_expression(
    initial_unit_of_temperature: &String,
    target_unit_of_temperature: &String,
) -> String {
    match get_scale(initial_unit_of_temperature)
        .scale_entries
        .into_iter()
        .find(|x| x.0 == ScaleEntry::TargetScaleName(target_unit_of_temperature.to_string()))
    {
        Some((_x, y)) => y.to_string(),
        None => panic!("could not get expression"),
    }
}
