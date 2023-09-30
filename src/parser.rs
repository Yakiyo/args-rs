#![allow(unused)]
use crate::ArgParser;
use anyhow::Result;
use std::{collections::VecDeque, ops::Deref};

#[derive(Debug, Clone)]
pub(crate) struct Parser {
    pub name: Option<String>,
    pub parent: Option<Box<Parser>>,
    pub grammar: ArgParser,
    pub rest: Vec<String>,
    pub args: VecDeque<String>,
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

    fn parse_solo_option(&self) -> Result<bool> {
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

    fn handle_solo_option(&self, opt: char) -> Result<bool> {
        let flag = self.grammar.find_by_abbr(opt);
        if flag.is_none() && self.parent.is_some() {}
        return Ok(true);
    }
}
