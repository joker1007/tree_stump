use crate::query::Query;
use crate::tree::Tree;
use crate::util::build_error;
use crate::LANG_LANGUAGES;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[magnus::wrap(class = "TreeHouse::Parser")]
pub struct Parser {
    raw_parser: RefCell<tree_sitter::Parser>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            raw_parser: RefCell::new(tree_sitter::Parser::new()),
        }
    }

    pub fn set_language(&self, lang: String) -> Result<bool, magnus::Error> {
        let languages = LANG_LANGUAGES.get_or_init(|| Mutex::new(HashMap::new()));
        let languages = languages.lock().unwrap();
        let language = languages.get(&lang);
        match language {
            Some(language) => {
                let result = self.raw_parser.borrow_mut().set_language(language);
                result.map_or_else(|e| Err(build_error(e.to_string())), |_| Ok(true))
            }
            None => Err(build_error(format!("Language {} is not registered", lang))),
        }
    }

    pub fn parse(&self, source: String) -> Result<Tree, magnus::Error> {
        let tree = self.raw_parser.borrow_mut().parse(source, None);

        match tree {
            Some(tree) => Ok(Tree::from(Arc::new(tree))),
            None => Err(build_error("Failed to parse")),
        }
    }

    pub fn reset(&self) {
        self.raw_parser.borrow_mut().reset();
    }

    pub fn timeout_micros(&self) -> u64 {
        self.raw_parser.borrow().timeout_micros()
    }

    pub fn set_timeout_micros(&self, timeout: u64) {
        self.raw_parser.borrow_mut().set_timeout_micros(timeout);
    }

    fn language(&self) -> Option<tree_sitter::Language> {
        self.raw_parser.borrow().language()
    }

    pub fn build_query(&self, source: String) -> Result<Query, magnus::Error> {
        let lang = self.language();
        lang.map_or_else(
            || Err(build_error("Failed to get language from parser")),
            |lang| Query::new(&lang, source),
        )
    }
}
