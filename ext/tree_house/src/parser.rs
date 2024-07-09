use crate::tree::Tree;
use crate::LANG_LANGUAGES;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

struct Parser {
    raw_parser: tree_sitter::Parser,
}

#[magnus::wrap(class = "TreeHouse::Parser")]
pub struct MutParser(RefCell<Parser>);

impl MutParser {
    pub fn new() -> Self {
        Self {
            0: RefCell::new(Parser {
                raw_parser: tree_sitter::Parser::new(),
            }),
        }
    }

    pub fn set_language(&self, lang: String) -> bool {
        let languages = LANG_LANGUAGES.get_or_init(|| Mutex::new(HashMap::new()));
        let languages = languages.lock().unwrap();
        let language = languages.get(&lang).unwrap();
        let _ = self
            .0
            .borrow_mut()
            .raw_parser
            .set_language(language)
            .expect("Failed to set language");
        true
    }

    pub fn parse(&self, source: String) -> Tree {
        let tree = self
            .0
            .borrow_mut()
            .raw_parser
            .parse(source, None)
            .expect("Failed to parse");
        Tree { raw_tree: tree }
    }
}
