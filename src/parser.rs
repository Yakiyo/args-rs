#![allow(unused)]

use crate::ArgParser;
use anyhow::Result;

#[derive(Debug, Clone)]
pub(crate) struct Parser {
    pub name: Option<String>,
    pub parent: Option<Box<Parser>>,
    pub grammar: ArgParser,
    pub rest: Vec<String>,
    pub args: Vec<String>,
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
        self.args.first()
    }
}
