use crate::{flag::FlagValue, parser::Parser};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArgResult {
    pub flags: HashMap<String, FlagValue>,
    pub rest: Vec<String>,
}

impl ArgResult {
    pub(crate) fn new(parser: &Parser) -> ArgResult {
        let parser = parser.clone();
        ArgResult {
            flags: parser.results,
            rest: parser.rest,
        }
    }
}
