use cucumber::Parameter;
use std::str::FromStr;

#[derive(Parameter)]
#[param(name = "nth", regex = "first|second")]
pub enum Nth {
    First = 0,
    Second = 1,
}

impl FromStr for Nth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "first" => Self::First,
            "second" => Self::Second,
            invalid => {
                return Err(format!(
                    "Value must be either 'first' or 'second' (actual value: {invalid})"
                ));
            }
        })
    }
}
