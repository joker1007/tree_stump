use magnus::{function, method, prelude::*, Error, Ruby};

use libloading::Library;
use tree_sitter::ffi::TSLanguage;
use tree_sitter::Language;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

mod parser;
mod tree;

use crate::parser::MutParser;
use crate::tree::{Node, Tree, TreeCursor};

pub static LANG_LIBRARIES: OnceLock<Mutex<HashMap<String, Library>>> = OnceLock::new();
pub static LANG_LANGUAGES: OnceLock<Mutex<HashMap<String, Language>>> = OnceLock::new();

fn register_lang(lang: String, path: String) -> () {
    let func_name = String::from("tree_sitter_") + &lang;
    let language;

    let libraries = LANG_LIBRARIES.get_or_init(|| Mutex::new(HashMap::new()));
    let languages = LANG_LANGUAGES.get_or_init(|| Mutex::new(HashMap::new()));

    unsafe {
        let mut libraries = libraries.lock().unwrap();
        let lib = libraries.entry(lang.to_string()).or_insert_with(|| {
            let loaded = Library::new(path).expect("Failed to load library");
            loaded
        });

        let func: libloading::Symbol<unsafe extern "C" fn() -> *const TSLanguage> =
            lib.get(func_name.as_bytes()).unwrap();

        language = Language::from_raw(func());

        let mut languages = languages.lock().unwrap();
        languages.insert(lang.to_string(), language);
    };
}

fn available_langs() -> Vec<String> {
    let languages = LANG_LANGUAGES.get_or_init(|| Mutex::new(HashMap::new()));
    let languages = languages.lock().unwrap();
    languages.keys().cloned().collect()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let namespace = ruby.define_module("TreeHouse")?;
    namespace.define_singleton_method("register_lang", function!(register_lang, 2))?;
    namespace.define_singleton_method("available_langs", function!(available_langs, 0))?;

    let parser_class = namespace.define_class("Parser", ruby.class_object())?;
    parser_class.define_singleton_method("new", function!(MutParser::new, 0))?;
    parser_class.define_method("set_language", method!(MutParser::set_language, 1))?;
    parser_class.define_method("parse", method!(MutParser::parse, 1))?;

    let tree_class = namespace.define_class("Tree", ruby.class_object())?;
    tree_class.define_method("root_node", method!(Tree::root_node, 0))?;
    tree_class.define_method("walk", method!(Tree::walk, 0))?;

    let tree_cursor_class = namespace.define_class("TreeCursor", ruby.class_object())?;
    tree_cursor_class.define_method("node", method!(TreeCursor::node, 0))?;
    tree_cursor_class.define_method("field_id", method!(TreeCursor::field_id, 0))?;
    tree_cursor_class
        .define_method("goto_first_child", method!(TreeCursor::goto_first_child, 0))?;
    tree_cursor_class.define_method("goto_last_child", method!(TreeCursor::goto_last_child, 0))?;
    tree_cursor_class.define_method("goto_parent", method!(TreeCursor::goto_parent, 0))?;
    tree_cursor_class.define_method(
        "goto_next_sibling",
        method!(TreeCursor::goto_next_sibling, 0),
    )?;
    tree_cursor_class.define_method("goto_descendant", method!(TreeCursor::goto_descendant, 1))?;
    tree_cursor_class.define_method(
        "goto_previous_sibling",
        method!(TreeCursor::goto_previous_sibling, 0),
    )?;
    tree_cursor_class.define_method(
        "goto_first_child_for_byte",
        method!(TreeCursor::goto_first_child_for_byte, 1),
    )?;
    tree_cursor_class.define_method("reset", method!(TreeCursor::reset, 1))?;
    tree_cursor_class.define_method("reset_to", method!(TreeCursor::reset_to, 1))?;

    let node_class = namespace.define_class("Node", ruby.class_object())?;
    node_class.define_method("id", method!(Node::id, 0))?;
    node_class.define_method("kind", method!(Node::kind, 0))?;
    node_class.define_method("kind_id", method!(Node::kind_id, 0))?;
    node_class.define_method("start_byte", method!(Node::start_byte, 0))?;
    node_class.define_method("end_byte", method!(Node::end_byte, 0))?;
    node_class.define_method("grammar_id", method!(Node::grammar_id, 0))?;
    node_class.define_method("grammar_name", method!(Node::grammer_name, 0))?;
    node_class.define_method("is_named?", method!(Node::is_named, 0))?;
    node_class.define_method("is_extra?", method!(Node::is_extra, 0))?;
    node_class.define_method("has_changes?", method!(Node::has_changes, 0))?;
    node_class.define_method("has_error?", method!(Node::has_error, 0))?;
    node_class.define_method("is_error?", method!(Node::is_error, 0))?;
    node_class.define_method("byte_range", method!(Node::byte_range, 0))?;
    node_class.define_method("child", method!(Node::child, 1))?;
    node_class.define_method("parent", method!(Node::parent, 0))?;
    node_class.define_method(
        "children_with_cursor",
        method!(Node::children_with_cursor, 1),
    )?;

    node_class.define_method("to_sexp", method!(Node::to_sexp, 0))?;
    node_class.define_method("walk", method!(Node::walk, 0))?;

    Ok(())
}
