use std::collections::HashMap;

use crate::flag::{Flag, FlagType};

/// A class for taking a list of raw command line arguments and parsing out
/// options and flags from them.
#[derive(Debug, Default)]
pub struct ArgParser {
    flags: HashMap<String, Flag>,
    allows_anything: bool,
}

impl ArgParser {
    /// initiate a new arg parser
    pub fn new() -> Self {
        ArgParser::default()
    }

    pub fn allow_anything(&mut self, allow: bool) -> &Self {
        self.allows_anything = allow;
        self
    }

    pub fn add_flag(
        &mut self,
        name: String,
        abbr: Option<char>,
        help: Option<String>,
        defaults_to: bool,
        negatable: bool,
    ) -> &Self {
        // for flag types, we consider default value `true` when option is some,
        // and default value `false` when option is none
        let default = if defaults_to {
            Some(String::new())
        } else {
            None
        };
        let flag = Flag {
            name: name.clone(),
            abbr,
            help,
            negatable,
            default,
            ..Default::default()
        };
        self.flags.insert(name, flag);
        self
    }

    pub fn add_option(
        &mut self,
        name: String,
        abbr: Option<char>,
        help: Option<String>,
        value_help: Option<String>,
        possible_values: Option<Vec<String>>,
        defaults_to: bool,
        required: bool,
    ) -> &Self {
        // for flag types, we consider default value `true` when option is some,
        // and default value `false` when option is none
        let default = if defaults_to {
            Some(String::new())
        } else {
            None
        };
        let flag = Flag {
            name: name.clone(),
            abbr,
            help,
            default,
            value_help,
            possible_values,
            required,
            flag_type: FlagType::Option,
            ..Default::default()
        };
        self.flags.insert(name, flag);
        self
    }
}