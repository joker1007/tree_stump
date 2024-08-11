use magnus::{function, method, prelude::*, typed_data, value::Lazy, Error, RClass, Ruby};

use libloading::Library;
use tree_sitter::ffi::TSLanguage;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

mod data;
mod language;
mod parser;
mod query;
mod tree;
mod util;

use crate::language::{LanguageRef, LookaheadIterator};
use crate::parser::Parser;
use crate::query::{Query, QueryCursor, QueryMatch};
use crate::tree::{Node, Tree, TreeCursor};

pub static LANG_LIBRARIES: OnceLock<Mutex<HashMap<String, Library>>> = OnceLock::new();
pub static LANG_LANGUAGES: OnceLock<Mutex<HashMap<String, tree_sitter::Language>>> =
    OnceLock::new();

pub static QUERY_CAPTURE_CLASS: Lazy<RClass> =
    Lazy::new(|ruby| ruby.define_struct(None, ("node", "index")).unwrap());

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

        language = tree_sitter::Language::from_raw(func());

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
    parser_class.define_singleton_method("new", function!(Parser::new, 0))?;
    parser_class.define_method("set_language", method!(Parser::set_language, 1))?;
    parser_class.define_method("parse", method!(Parser::parse, 1))?;
    parser_class.define_method("reset", method!(Parser::reset, 0))?;
    parser_class.define_method("timeout_micros", method!(Parser::timeout_micros, 0))?;
    parser_class.define_method("set_timeout_micros", method!(Parser::set_timeout_micros, 1))?;
    parser_class.define_method("build_query", method!(Parser::build_query, 1))?;

    let tree_class = namespace.define_class("Tree", ruby.class_object())?;
    tree_class.define_method("root_node", method!(Tree::root_node, 0))?;
    tree_class.define_method("language", method!(Tree::language, 0))?;
    tree_class.define_method("walk", method!(Tree::walk, 0))?;
    tree_class.define_method("print_dot_graph", method!(Tree::print_dot_graph, 1))?;
    tree_class.define_method("inspect", method!(Tree::inspect, 0))?;

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
    node_class.define_method("hash", method!(<Node as typed_data::Hash>::hash, 0))?;
    node_class.define_method("==", method!(<Node as typed_data::IsEql>::is_eql, 1))?;
    node_class.define_method("eql?", method!(<Node as typed_data::IsEql>::is_eql, 1))?;
    node_class.define_method("id", method!(Node::id, 0))?;
    node_class.define_method("kind", method!(Node::kind, 0))?;
    node_class.define_method("kind_id", method!(Node::kind_id, 0))?;
    node_class.define_method("grammar_id", method!(Node::grammar_id, 0))?;
    node_class.define_method("grammar_name", method!(Node::grammar_name, 0))?;
    node_class.define_method("language", method!(Node::language, 0))?;
    node_class.define_method("is_named?", method!(Node::is_named, 0))?;
    node_class.define_method("is_extra?", method!(Node::is_extra, 0))?;
    node_class.define_method("has_changes?", method!(Node::has_changes, 0))?;
    node_class.define_method("has_error?", method!(Node::has_error, 0))?;
    node_class.define_method("is_error?", method!(Node::is_error, 0))?;
    node_class.define_method("parse_state", method!(Node::parse_state, 0))?;
    node_class.define_method("next_parse_state", method!(Node::next_parse_state, 0))?;
    node_class.define_method("start_byte", method!(Node::start_byte, 0))?;
    node_class.define_method("end_byte", method!(Node::end_byte, 0))?;
    node_class.define_method("byte_range", method!(Node::byte_range, 0))?;
    node_class.define_method("range", method!(Node::range, 0))?;
    node_class.define_method("start_position", method!(Node::start_position, 0))?;
    node_class.define_method("end_position", method!(Node::end_position, 0))?;
    node_class.define_method("child", method!(Node::child, 1))?;
    node_class.define_method("child_count", method!(Node::child_count, 0))?;
    node_class.define_method("named_child", method!(Node::named_child, 1))?;
    node_class.define_method("named_child_count", method!(Node::named_child_count, 0))?;
    node_class.define_method("child_by_field_name", method!(Node::child_by_field_name, 1))?;
    node_class.define_method("child_by_field_id", method!(Node::child_by_field_id, 1))?;
    node_class.define_method(
        "field_name_for_child",
        method!(Node::field_name_for_child, 1),
    )?;
    node_class.define_method("parent", method!(Node::parent, 0))?;
    node_class.define_method("children", method!(Node::children, 0))?;
    node_class.define_method(
        "children_with_cursor",
        method!(Node::children_with_cursor, 1),
    )?;
    node_class.define_method(
        "named_children_with_cursor",
        method!(Node::named_children_with_cursor, 1),
    )?;
    node_class.define_method(
        "children_by_field_name_with_cursor",
        method!(Node::children_by_field_name_with_cursor, 2),
    )?;
    node_class.define_method(
        "children_by_field_id_with_cursor",
        method!(Node::children_by_field_id_with_cursor, 2),
    )?;
    node_class.define_method(
        "child_containing_descendant",
        method!(Node::child_containing_descendant, 1),
    )?;
    node_class.define_method("next_sibling", method!(Node::next_sibling, 0))?;
    node_class.define_method("prev_sibling", method!(Node::prev_sibling, 0))?;
    node_class.define_method("next_named_sibling", method!(Node::next_named_sibling, 0))?;
    node_class.define_method("prev_named_sibling", method!(Node::prev_named_sibling, 0))?;
    node_class.define_method("descendant_count", method!(Node::descendant_count, 0))?;
    node_class.define_method(
        "descendant_for_byte_range",
        method!(Node::descendant_for_byte_range, 2),
    )?;
    node_class.define_method(
        "named_descendant_for_byte_range",
        method!(Node::named_descendant_for_byte_range, 2),
    )?;
    node_class.define_method(
        "descendant_for_point_range",
        method!(Node::descendant_for_point_range, 2),
    )?;
    node_class.define_method(
        "named_descendant_for_point_range",
        method!(Node::named_descendant_for_point_range, 2),
    )?;

    node_class.define_method("to_sexp", method!(Node::to_sexp, 0))?;
    node_class.define_method("utf8_text", method!(Node::utf8_text, 1))?;
    node_class.define_method("walk", method!(Node::walk, 0))?;

    node_class.define_method("inspect", method!(Node::inspect, 0))?;
    node_class.define_method("to_s", method!(Node::to_s, 0))?;

    let point_class = namespace.define_class("Point", ruby.class_object())?;
    point_class.define_singleton_method("new", function!(data::Point::new, 2))?;
    point_class.define_method("hash", method!(<data::Point as typed_data::Hash>::hash, 0))?;
    point_class.define_method("==", method!(<data::Point as typed_data::IsEql>::is_eql, 1))?;
    point_class.define_method(
        "eql?",
        method!(<data::Point as typed_data::IsEql>::is_eql, 1),
    )?;
    point_class.define_method("row", method!(data::Point::get_row, 0))?;
    point_class.define_method("column", method!(data::Point::get_column, 0))?;
    point_class.define_method("inspect", method!(data::Point::inspect, 0))?;
    point_class.define_method("to_s", method!(data::Point::to_s, 0))?;

    let range_class = namespace.define_class("Range", ruby.class_object())?;
    range_class.define_singleton_method("new", function!(data::Range::new, 4))?;
    range_class.define_method("hash", method!(<data::Range as typed_data::Hash>::hash, 0))?;
    range_class.define_method("==", method!(<data::Range as typed_data::IsEql>::is_eql, 1))?;
    range_class.define_method(
        "eql?",
        method!(<data::Range as typed_data::IsEql>::is_eql, 1),
    )?;
    range_class.define_method("start_byte", method!(data::Range::get_start_byte, 0))?;
    range_class.define_method("end_byte", method!(data::Range::get_end_byte, 0))?;
    range_class.define_method("start_point", method!(data::Range::get_start_point, 0))?;
    range_class.define_method("end_point", method!(data::Range::get_end_point, 0))?;
    range_class.define_method("inspect", method!(data::Range::inspect, 0))?;
    range_class.define_method("to_s", method!(data::Range::to_s, 0))?;

    let language_class = namespace.define_class("LanguageRef", ruby.class_object())?;
    language_class.define_method("version", method!(LanguageRef::version, 0))?;
    language_class.define_method("node_kind_count", method!(LanguageRef::node_kind_count, 0))?;
    language_class.define_method(
        "parse_state_count",
        method!(LanguageRef::parse_state_count, 0),
    )?;
    language_class.define_method(
        "node_kind_for_id",
        method!(LanguageRef::node_kind_for_id, 1),
    )?;
    language_class.define_method(
        "id_for_node_kind",
        method!(LanguageRef::id_for_node_kind, 2),
    )?;
    language_class.define_method(
        "node_kind_is_named",
        method!(LanguageRef::node_kind_is_named, 1),
    )?;
    language_class.define_method(
        "node_kind_is_visible",
        method!(LanguageRef::node_kind_is_visible, 1),
    )?;
    language_class.define_method(
        "field_name_for_id",
        method!(LanguageRef::field_name_for_id, 1),
    )?;
    language_class.define_method(
        "field_id_for_name",
        method!(LanguageRef::field_id_for_name, 1),
    )?;
    language_class.define_method("next_state", method!(LanguageRef::next_state, 2))?;
    language_class.define_method(
        "lookahead_iterator",
        method!(LanguageRef::lookahead_iterator, 1),
    )?;

    let lookahead_iterator_class =
        namespace.define_class("LookaheadIterator", ruby.class_object())?;
    lookahead_iterator_class.define_method("next", method!(LookaheadIterator::next, 0))?;
    lookahead_iterator_class.define_method(
        "current_symbol_name",
        method!(LookaheadIterator::current_symbol_name, 0),
    )?;

    let query_class = namespace.define_class("Query", ruby.class_object())?;
    query_class.define_method(
        "start_byte_for_pattern",
        method!(Query::start_byte_for_pattern, 1),
    )?;
    query_class.define_method("pattern_count", method!(Query::pattern_count, 0))?;
    query_class.define_method("capture_names", method!(Query::capture_names, 0))?;
    query_class.define_method(
        "capture_quantifiers",
        method!(Query::capture_quantifiers, 1),
    )?;
    query_class.define_method(
        "capture_index_for_name",
        method!(Query::capture_index_for_name, 1),
    )?;
    query_class.define_method("disable_capture", method!(Query::disable_capture, 1))?;
    query_class.define_method("disable_pattern", method!(Query::disable_pattern, 1))?;
    query_class.define_method("is_pattern_rooted", method!(Query::is_pattern_rooted, 1))?;
    query_class.define_method(
        "is_pattern_guaranteed_at_step",
        method!(Query::is_pattern_guaranteed_at_step, 1),
    )?;

    Lazy::force(&QUERY_CAPTURE_CLASS, ruby);
    let struct_class = Lazy::try_get_inner(&QUERY_CAPTURE_CLASS).unwrap();
    namespace.const_set("QueryCapture", struct_class)?;

    let query_match_class = namespace.define_class("QueryMatch", ruby.class_object())?;
    query_match_class.define_method("pattern_index", method!(QueryMatch::pattern_index, 0))?;
    query_match_class.define_method("captures", method!(QueryMatch::captures, 0))?;

    let query_cursor_class = namespace.define_class("QueryCursor", ruby.class_object())?;
    query_cursor_class.define_singleton_method("new", function!(QueryCursor::new, 0))?;
    query_cursor_class.define_method("match_limit", method!(QueryCursor::match_limit, 0))?;
    query_cursor_class
        .define_method("set_match_limit", method!(QueryCursor::set_match_limit, 1))?;
    query_cursor_class.define_method(
        "did_exceed_match_limit",
        method!(QueryCursor::did_exceed_match_limit, 0),
    )?;
    query_cursor_class.define_method("matches", method!(QueryCursor::matches, 3))?;
    query_cursor_class.define_method("set_byte_range", method!(QueryCursor::set_byte_range, 1))?;
    query_cursor_class
        .define_method("set_point_range", method!(QueryCursor::set_point_range, 1))?;
    query_cursor_class.define_method(
        "set_max_start_depth",
        method!(QueryCursor::set_max_start_depth, 1),
    )?;

    Ok(())
}
