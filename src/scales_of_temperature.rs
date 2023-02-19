use std::fmt;

#[derive(PartialEq, Eq, Debug)]
enum ScaleEntry {
    TargetScaleName(String),
    Expression(String),
}

impl fmt::Display for ScaleEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TargetScaleName(x) | Self::Expression(x) => write!(f, "{x}"),
        }
    }
}
#[derive(Debug)]
pub struct Scale {
    pub scale_name: String,
    scale_entries: Vec<(ScaleEntry, ScaleEntry)>,
    pub abbreviation: String,
    // TODO add description
    // TODO add boiling point of water
    // TODO add freezing point of water
    // TODO add absolute zero
    // TODO add other interesting information
}

impl Scale {
    pub fn new(scale_name: &str, scale_entry: [(&str, &str); 3], abbreviation: &str) -> Self {
        Self {
            scale_name: scale_name.to_string(),
            scale_entries: scale_entry[..]
                .iter()
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
// TODO create process to scrape info from wiki and generate file
//TODO change this into a file i.o
fn compose_scales() -> Vec<Scale> {
    let scales_of_temperature: Vec<Scale> = vec![
        Scale::new(
            "Celsius",
            [
                ("Kelvin", "=(x+273.15)"),
                ("Fahrenheit", "=(x*9/5+32)"),
                ("Rankine", "=((x+273.15)*9/5)"),
            ],
            "C",
        ),
        Scale::new(
            "Kelvin",
            [
                ("Celsius", "=(x-273.15)"),
                ("Fahrenheit", "=(x*9/5-459.67)"),
                ("Rankine", "=(x*9/5)"),
            ],
            "K",
        ),
        Scale::new(
            "Fahrenheit",
            [
                ("Celsius", "=((x-32)*5/9)"),
                ("Kelvin", "=((x+459.67)*5/9)"),
                ("Rankine", "=(x+459.67)"),
            ],
            "F",
        ),
        Scale::new(
            "Rankine",
            [
                ("Celsius", "=((x-491.67)*5/9)"),
                ("Kelvin", "=(x*5/9)"),
                ("Fahrenheit", "=(x-459.67)"),
            ],
            "R",
        ),
    ];
    scales_of_temperature
}

pub fn get_scale(scale_name: &str) -> Scale {
    compose_scales()
        .into_iter()
        .find(|x| x.scale_name == scale_name)
        .expect("scale does not exist")
}

pub fn get_scales(scale: Option<&str>) -> Vec<String> {
    let mut scales: Vec<String> = vec![];
    if let Some(s) = scale {
        for scale in get_scale(s).scale_entries {
            scales.push(scale.0.to_string());
        }
    } else {
        for scale in &compose_scales() {
            scales.push(scale.scale_name.to_string());
        }
    }
    scales
}

pub fn get_expression(
    initial_unit_of_temperature: &str,
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
