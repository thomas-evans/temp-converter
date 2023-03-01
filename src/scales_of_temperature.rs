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
    // TODO make this return an result with either the scale name or failure
    compose_scales()
        .into_iter()
        .find(|x| x.scale_name == scale_name)
        .expect("scale does not exist")
}

pub fn get_scales(scale: Option<&str>) -> Vec<String> {
    // TODO look into making this have a default
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
    // TODO this should return a not found error
    match get_scale(initial_unit_of_temperature)
        .scale_entries
        .into_iter()
        .find(|x| x.0 == ScaleEntry::TargetScaleName(target_unit_of_temperature.to_string()))
    {
        Some((_x, y)) => y.to_string(),
        None => panic!("could not get expression"),
    }
}


#[cfg(test)]
mod scale_entry_display{
    use super::ScaleEntry;
    #[test]
    fn it_should_have_the_ability_to_be_cast_into_string(){
        let entry_target_scale_name = ScaleEntry::TargetScaleName(String::from("Celsius"));
        let entry_expression = ScaleEntry::Expression(String::from("x+23"));
        assert!(entry_target_scale_name.to_string() == "Celsius");
        assert!(entry_expression.to_string() == "x+23");
    }
}

#[cfg(test)]
mod scale_new{
    use super::Scale;
    use super::ScaleEntry;
    #[test]
    fn it_should_generate_a_scale_struct(){
        let scale = Scale::new(
            "Celsius",
            [
                ("Kelvin", "=(x+273.15)"),
                ("Fahrenheit", "=(x*9/5+32)"),
                ("Rankine", "=((x+273.15)*9/5)"),
            ],
            "C",
        );
        assert!(scale.scale_name == "Celsius");
        assert!(scale.scale_entries[0].0 == ScaleEntry::TargetScaleName(String::from("Kelvin")));
        assert!(scale.scale_entries[0].1 == ScaleEntry::Expression(String::from("=(x+273.15)")));
        assert!(scale.scale_entries[1].0 == ScaleEntry::TargetScaleName(String::from("Fahrenheit")));
        assert!(scale.scale_entries[1].1 == ScaleEntry::Expression(String::from("=(x*9/5+32)")));
        assert!(scale.scale_entries[2].0 == ScaleEntry::TargetScaleName(String::from("Rankine")));
        assert!(scale.scale_entries[2].1 == ScaleEntry::Expression(String::from("=((x+273.15)*9/5)")));
        assert!(scale.abbreviation == "C");
    }
}

#[cfg(test)]
mod compose_scales{
    use super::compose_scales;
    #[test]
    fn it_should_return_a_vector_of_scales(){
        let scale_vector = compose_scales();

    }
}

#[cfg(test)]
mod get_scale{
    // it should return a scale from it's name
    #[test]
    fn it_should_return_a_scale_given_a_scale_name(){}
    // it should panic when a non-existent scale name is requested
    #[test]
    fn it_should_panic_when_scale_does_not_exist(){}
}

#[cfg(test)]
mod get_scales{
    // it should return a vector of strings (scale names) when give a scale name
    #[test]
    fn it_should_return_a_specific_list_of_scales_given_a_scale_name(){}
    // it should return all possible scales when given no scale name
    #[test]
    fn it_should_return_all_possible_scales_given_no_scale_name(){}
}

#[cfg(test)]
mod get_expression{
    // it should return a string (expression) when given a matching initial scale name and target scale name
    #[test]
    fn it_should_return_an_expression_given_an_initial_and_target_scale_name(){}
    // it should panic when given a non matching initial scale name
    #[test]
    fn it_should_panic_given_a_non_matching_initial_scale_name(){}
    // it should panic when given a non matching target scale name
    #[test]
    fn it_should_panic_given_a_non_matching_target_scale_name(){}
}