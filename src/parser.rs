#![allow(unused)]
use crate::flag::FlagValue;
use crate::ArgParser;
use anyhow::{bail, Result};
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
        }
        Ok(())
    }

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
        return Ok(true);
    }
}
