#![allow(unused)]
use crate::flag::FlagValue;
use crate::ArgParser;
use anyhow::{bail, ensure, Context, Result};
use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
};

#[derive(Debug, Clone)]
pub(crate) struct Parser {
    pub name: Option<String>,
    pub parent: Option<Box<Parser>>,
    pub grammar: ArgParser,
    pub rest: Vec<String>,
    pub args: VecDeque<String>,
    pub results: HashMap<String, FlagValue>,
}

impl Parser {
    pub fn new<T: Into<String>>(
        name: Option<String>,
        grammar: ArgParser,
        args: Vec<T>,
        parent: Option<Parser>,
    ) -> Self {
        Parser {
            name,
            grammar,
            args: args.into_iter().map(|f| f.into()).collect(),
            parent: parent.map(Box::new),
            rest: Vec::new(),
            results: HashMap::new(),
        }
    }

    fn current(&self) -> Option<&String> {
        self.args.front()
    }

    pub fn parse(&mut self) -> Result<()> {
        while !self.args.is_empty() {
            let current = self.current().unwrap();
            if current == "--" {
                // Reached the argument terminator, so stop here.
                self.args.pop_front();
                break;
            }

            // TODO: handle command here

            if self.parse_solo_option()? {
                continue;
            }
            // if self.parse_abbreviation()? {
            //     continue;
            // }
            if self.parse_long_option()? {
                continue;
            }

            self.rest.push(
                self.args
                    .pop_front()
                    .context("Argument should be here but is missing")?,
            );
        }
        let opts = &self.grammar.flags;
        for (name, opt) in opts {
            let parsed = self.results.get(name);

            // if no value was passed and theres a default value, then set it
            if parsed.is_none() && opt.default.is_some() {
                self.results.insert(
                    name.to_string(),
                    FlagValue::String(opt.default.clone().unwrap()),
                );
            }
            let parsed = self.results.get(name);

            if opt.required && parsed.is_none() {
                bail!("Missing argument for required option {}", name);
            }
        }
        if !self.args.is_empty() {
            let mut args: Vec<String> = self.args.clone().into_iter().collect();;
            self.rest.append(&mut args);
        }
        Ok(())
    }

    /// parse single abbreviations, that is `-a abc` or just `-a`
    fn parse_solo_option(&mut self) -> Result<bool> {
        let current = self.current().unwrap();

        if current.len() != 2 || !current.starts_with('-') {
            return Ok(false);
        }
        let opt = current.chars().nth(1).unwrap();
        if !opt.is_alphabetic() {
            return Ok(false);
        }

        return self.handle_solo_option(opt);
    }

    pub(self) fn handle_solo_option(&mut self, opt: char) -> Result<bool> {
        let flag = self.grammar.find_by_abbr(opt);
        if flag.is_none() {
            if self.parent.is_none() {
                bail!("No flag found for `-{}`", opt)
            }
            return self.parent.clone().unwrap().handle_solo_option(opt);
        }
        self.args.pop_front();
        let flag = flag.unwrap();

        if flag.is_flag() {
            self.results.insert(flag.name, FlagValue::Bool(true));
        } else {
            if self.args.is_empty() {
                bail!("Missing argument for flag {}", flag.name)
            }
            self.results
                .insert(flag.name, FlagValue::String(self.current().unwrap().into()));
            self.args.pop_front();
        }
        Ok(true)
    }

    // fn parse_abbreviation(&self) -> Result<bool> {
    //     let current = self.current().unwrap();
    //     if current.len() < 2 || !current.starts_with('-') {
    //         return Ok(false);
    //     }
    //     let mut index = 1;
    //     let c: Vec<char> = current.chars().collect();

    //     Ok(true)
    // }

    fn parse_long_option(&mut self) -> Result<bool> {
        let current = self.current().unwrap();
        if !current.starts_with("--") {
            return Ok(false);
        }
        let current = current.strip_prefix("--").unwrap().to_string();
        let (name, value) = if current.contains('=') {
            let split: Vec<&str> = current.split('=').collect();
            if split.len() != 2 {
                bail!(
                    "Invalid usage of long option, did not receive value after = sign. Received --{}",
                    current
                );
            }
            (split[0], Some(split[1]))
        } else {
            (current.as_str(), None)
        };
        if value.is_some_and(|x| x == "\n" || x == "\r") {
            return Ok(false);
        }

        self.handle_long_option(name, value)
    }

    pub(crate) fn handle_long_option(&mut self, name: &str, value: Option<&str>) -> Result<bool> {
        if let Some(option) = self.grammar.find_by_name(name) {
            let fname = option.name.clone();
            self.args.pop_front();
            if option.is_flag() {
                if value.is_some() {
                    bail!(
                        "Option --{} does not require a value to be passed",
                        option.name
                    )
                }
                self.results.insert(fname, FlagValue::Bool(true));
            } else if value.is_some() {
                self.results
                    .insert(fname, FlagValue::String(value.unwrap().to_string()));
            } else {
                if self.args.is_empty() {
                    bail!("Missing argument for {}", option.name);
                }
                self.results
                    .insert(fname, FlagValue::String(self.current().unwrap().into()));
                self.args.pop_front();
            }
        } else if name.starts_with("no-") {
            let pos_name = name.strip_prefix("no-").unwrap();
            let option = self.grammar.find_by_name(pos_name);
            if option.is_none() {
                ensure!(self.parent.is_some(), "No option named {} found", name);
                return self.parent.clone().unwrap().handle_long_option(name, value);
            }
            self.args.pop_front();
            let option = option.unwrap();
            ensure!(option.is_flag(), "Cannot negate non-flag option {}", name);
            ensure!(option.is_negatable(), "Flag {} is non-negatable", name);

            self.results
                .insert(option.name.clone(), FlagValue::Bool(false));
        } else {
            ensure!(self.parent.is_some(), "No option named {} found", name);
            return return self.parent.clone().unwrap().handle_long_option(name, value);
        }
        Ok(true)
    }
}
