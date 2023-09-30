#[derive(Debug, Default)]
pub struct Flag {
    pub name: String,
    pub abbr: Option<char>,
    pub help: Option<String>,
    pub value_help: Option<String>,
    pub possible_values: Option<Vec<String>>,
    pub negatable: bool,
    pub flag_type: FlagType,
    pub required: bool,
    pub default: Option<String>,
}

impl Flag {
    /// if it is of type flag
    pub fn is_flag(&self) -> bool {
        matches!(self.flag_type, FlagType::Flag)
    }

    /// if it is of type option
    pub fn is_option(&self) -> bool {
        matches!(self.flag_type, FlagType::Option)
    }
}

/// The type of the flag
#[derive(Debug, Default)]
pub enum FlagType {
    /// is of bool value, as in takes no value
    #[default]
    Flag,
    /// takes a single value
    Option,
}
