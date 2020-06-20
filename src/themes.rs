use std::str::FromStr;

pub enum Theme {
    Nord,
    Solarized,
}

impl FromStr for Theme {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nord" => Ok(Theme::Nord),
            "solarized" => Ok(Theme::Solarized),
            _ => Err("Expected one of: 'nord', 'solarized'"),
        }
    }
}
