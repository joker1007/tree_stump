use magnus::block::Yield;
use magnus::value::ReprValue;
use magnus::{typed_data, Error, RFile, Ruby, Value};

use std::cell::RefCell;
use std::hash::Hash;
use std::num::NonZero;
use std::ops::Range;
use std::sync::Arc;

use crate::data;
use crate::data::Point;
use crate::language::LanguageRef;
use crate::util::build_error;

#[magnus::wrap(class = "TreeStump::Tree", free_immediately)]
pub struct Tree {
    raw_tree: Arc<tree_sitter::Tree>,
}

impl Tree {
    pub fn from(raw_tree: Arc<tree_sitter::Tree>) -> Self {
        Self { raw_tree }
    }

    pub fn root_node(&self) -> Node<'_> {
        Node {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: self.raw_tree.root_node(),
        }
    }

    pub fn language(&self) -> LanguageRef<'_> {
        let raw_lang_ref = self.raw_tree.language();
        LanguageRef {
            raw_language_ref: raw_lang_ref,
        }
    }

    pub fn walk(&self) -> TreeCursor<'_> {
        TreeCursor {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_cursor: RefCell::new(self.raw_tree.walk()),
        }
    }

    pub fn print_dot_graph(&self, io: RFile) {
        self.raw_tree.print_dot_graph(&io);
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self.raw_tree)
    }
}

#[magnus::wrap(class = "TreeStump::TreeCursor", free_immediately, unsafe_generics)]
pub struct TreeCursor<'cursor> {
    raw_tree: Arc<tree_sitter::Tree>,
    raw_cursor: RefCell<tree_sitter::TreeCursor<'cursor>>,
}

impl<'cursor> TreeCursor<'cursor> {
    pub fn node(&self) -> Node {
        Node {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: self.raw_cursor.borrow().node(),
        }
    }

    pub fn field_id(&self) -> Option<u16> {
        self.raw_cursor.borrow().field_id().map(|id| id.get())
    }

    pub fn goto_first_child(&self) -> bool {
        self.raw_cursor.borrow_mut().goto_first_child()
    }

    pub fn goto_last_child(&self) -> bool {
        self.raw_cursor.borrow_mut().goto_last_child()
    }

    pub fn goto_parent(&self) -> bool {
        self.raw_cursor.borrow_mut().goto_parent()
    }

    pub fn goto_next_sibling(&self) -> bool {
        self.raw_cursor.borrow_mut().goto_next_sibling()
    }

    pub fn goto_descendant(&self, descendant_index: usize) {
        self.raw_cursor
            .borrow_mut()
            .goto_descendant(descendant_index)
    }

    pub fn goto_previous_sibling(&self) -> bool {
        self.raw_cursor.borrow_mut().goto_previous_sibling()
    }

    pub fn goto_first_child_for_byte(&self, index: usize) -> Option<usize> {
        self.raw_cursor
            .borrow_mut()
            .goto_first_child_for_byte(index)
    }

    pub fn reset(&self, node: &Node<'cursor>) -> bool {
        self.raw_cursor.borrow_mut().reset(node.raw_node);
        true
    }

    pub fn reset_to(&self, cursor: &Self) {
        self.raw_cursor
            .borrow_mut()
            .reset_to(&cursor.raw_cursor.borrow())
    }
}

#[magnus::wrap(class = "TreeStump::Node", free_immediately, unsafe_generics)]
#[derive(Debug, Clone)]
pub struct Node<'tree> {
    pub raw_tree: Arc<tree_sitter::Tree>,
    pub raw_node: tree_sitter::Node<'tree>,
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_node == other.raw_node
    }
}

impl Eq for Node<'_> {}

impl Hash for Node<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_node.hash(state)
    }
}

impl<'tree> Node<'tree> {
    pub fn new(raw_tree: Arc<tree_sitter::Tree>, raw_node: tree_sitter::Node<'tree>) -> Self {
        Self { raw_tree, raw_node }
    }

    pub fn get_raw_node(&self) -> tree_sitter::Node<'tree> {
        self.raw_node
    }

    pub fn id(&self) -> usize {
        self.raw_node.id()
    }

    pub fn kind(&self) -> &'static str {
        self.raw_node.kind()
    }

    pub fn kind_id(&self) -> u16 {
        self.raw_node.kind_id()
    }

    pub fn grammar_id(&self) -> u16 {
        self.raw_node.grammar_id()
    }

    pub fn grammar_name(&self) -> &'static str {
        self.raw_node.grammar_name()
    }

    pub fn language(&self) -> LanguageRef<'_> {
        let raw_lang_ref = self.raw_node.language();
        LanguageRef {
            raw_language_ref: raw_lang_ref,
        }
    }

    pub fn is_named(&self) -> bool {
        self.raw_node.is_named()
    }

    pub fn is_extra(&self) -> bool {
        self.raw_node.is_extra()
    }

    pub fn has_changes(&self) -> bool {
        self.raw_node.has_changes()
    }

    pub fn has_error(&self) -> bool {
        self.raw_node.has_error()
    }

    pub fn is_error(&self) -> bool {
        self.raw_node.is_error()
    }

    pub fn parse_state(&self) -> u16 {
        self.raw_node.parse_state()
    }

    pub fn next_parse_state(&self) -> u16 {
        self.raw_node.parse_state()
    }

    pub fn start_byte(&self) -> usize {
        self.raw_node.start_byte()
    }

    pub fn end_byte(&self) -> usize {
        self.raw_node.end_byte()
    }

    pub fn byte_range(&self) -> Range<usize> {
        self.raw_node.byte_range()
    }

    pub fn range(&self) -> data::Range {
        self.raw_node.range().into()
    }

    pub fn start_position(&self) -> Point {
        self.raw_node.start_position().into()
    }

    pub fn end_position(&self) -> Point {
        self.raw_node.end_position().into()
    }

    pub fn child(&self, index: usize) -> Option<Self> {
        self.raw_node.child(index).map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn child_count(&self) -> usize {
        self.raw_node.child_count()
    }

    pub fn named_child(&self, index: usize) -> Option<Self> {
        self.raw_node.named_child(index).map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn named_child_count(&self) -> usize {
        self.raw_node.named_child_count()
    }

    pub fn child_by_field_name(&self, field_name: String) -> Option<Self> {
        self.raw_node
            .child_by_field_name(field_name)
            .map(|node| Self {
                raw_tree: Arc::clone(&self.raw_tree),
                raw_node: node,
            })
    }

    pub fn child_by_field_id(&self, field_id: u16) -> Option<Self> {
        self.raw_node.child_by_field_id(field_id).map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn field_name_for_child(&self, child_index: u32) -> Option<&'static str> {
        self.raw_node.field_name_for_child(child_index)
    }

    pub fn children<'cursor>(
        ruby: &Ruby,
        rb_self: typed_data::Obj<Self>,
    ) -> Result<Yield<impl Iterator<Item = Value>>, Error> {
        let mut cursor = rb_self.raw_node.walk();
        let nodes = rb_self.raw_node.children(&mut cursor);
        let array = ruby.ary_new_capa(nodes.len());
        for n in nodes {
            let node = Self {
                raw_tree: Arc::clone(&rb_self.raw_tree),
                raw_node: n,
            };
            array.push(node)?
        }
        array.freeze();

        if ruby.block_given() {
            Ok(Yield::Iter(array.into_iter()))
        } else {
            Ok(Yield::Enumerator(rb_self.enumeratorize("children", ())))
        }
    }

    pub fn children_with_cursor<'cursor>(
        ruby: &Ruby,
        rb_self: typed_data::Obj<Self>,
        cursor: typed_data::Obj<TreeCursor<'tree>>,
    ) -> Result<Yield<impl Iterator<Item = Value>>, Error> {
        let mut borrowed = cursor.raw_cursor.borrow_mut();
        let nodes = rb_self.raw_node.children(&mut borrowed);
        let array = ruby.ary_new_capa(nodes.len());
        for n in nodes {
            let node = Self {
                raw_tree: Arc::clone(&rb_self.raw_tree),
                raw_node: n,
            };
            array.push(node)?
        }
        array.freeze();

        if ruby.block_given() {
            Ok(Yield::Iter(array.into_iter()))
        } else {
            Ok(Yield::Enumerator(
                rb_self.enumeratorize("children_with_cursor", ()),
            ))
        }
    }

    pub fn named_children_with_cursor<'cursor>(
        ruby: &Ruby,
        rb_self: typed_data::Obj<Self>,
        cursor: typed_data::Obj<TreeCursor<'tree>>,
    ) -> Result<Yield<impl Iterator<Item = Value>>, Error> {
        let mut borrowed = cursor.raw_cursor.borrow_mut();
        let nodes = rb_self.raw_node.named_children(&mut borrowed);
        let array = ruby.ary_new_capa(nodes.len());
        for n in nodes {
            let node = Self {
                raw_tree: Arc::clone(&rb_self.raw_tree),
                raw_node: n,
            };
            array.push(node)?
        }
        array.freeze();

        if ruby.block_given() {
            Ok(Yield::Iter(array.into_iter()))
        } else {
            Ok(Yield::Enumerator(
                rb_self.enumeratorize("named_children_with_cursor", [cursor]),
            ))
        }
    }

    pub fn children_by_field_name_with_cursor<'cursor>(
        ruby: &Ruby,
        rb_self: typed_data::Obj<Self>,
        field_name: String,
        cursor: typed_data::Obj<TreeCursor<'tree>>,
    ) -> Result<Yield<impl Iterator<Item = Value>>, Error> {
        let mut borrowed = cursor.raw_cursor.borrow_mut();
        let nodes = rb_self
            .raw_node
            .children_by_field_name(&field_name, &mut borrowed);
        let array = ruby.ary_new();
        for n in nodes {
            let node = Self {
                raw_tree: Arc::clone(&rb_self.raw_tree),
                raw_node: n,
            };
            array.push(node)?
        }
        array.freeze();

        if ruby.block_given() {
            Ok(Yield::Iter(array.into_iter()))
        } else {
            Ok(Yield::Enumerator(
                rb_self.enumeratorize("named_children_with_cursor", [cursor]),
            ))
        }
    }

    pub fn children_by_field_id_with_cursor<'cursor>(
        ruby: &Ruby,
        rb_self: typed_data::Obj<Self>,
        field_id: u16,
        cursor: typed_data::Obj<TreeCursor<'tree>>,
    ) -> Result<Yield<impl Iterator<Item = Value>>, Error> {
        let mut borrowed = cursor.raw_cursor.borrow_mut();
        let non_zero_field_id = match NonZero::new(field_id) {
            Some(id) => Ok(id),
            None => Err(build_error("field_id must be non-zero")),
        }?;
        let nodes = rb_self
            .raw_node
            .children_by_field_id(non_zero_field_id, &mut borrowed);
        let array = ruby.ary_new();
        for n in nodes {
            let node = Self {
                raw_tree: Arc::clone(&rb_self.raw_tree),
                raw_node: n,
            };
            array.push(node)?
        }
        array.freeze();

        if ruby.block_given() {
            Ok(Yield::Iter(array.into_iter()))
        } else {
            Ok(Yield::Enumerator(
                rb_self.enumeratorize("named_children_with_cursor", [cursor]),
            ))
        }
    }

    pub fn parent(&self) -> Option<Self> {
        self.raw_node.parent().map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn child_containing_descendant(
        &self,
        descendant: typed_data::Obj<Self>,
    ) -> Result<Option<Self>, Error> {
        Ok(self
            .raw_node
            .child_containing_descendant(descendant.raw_node)
            .map(|node| Self {
                raw_tree: Arc::clone(&self.raw_tree),
                raw_node: node,
            }))
    }

    pub fn next_sibling(&self) -> Option<Self> {
        self.raw_node.next_sibling().map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn prev_sibling(&self) -> Option<Self> {
        self.raw_node.prev_sibling().map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn next_named_sibling(&self) -> Option<Self> {
        self.raw_node.next_named_sibling().map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn prev_named_sibling(&self) -> Option<Self> {
        self.raw_node.prev_named_sibling().map(|node| Self {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_node: node,
        })
    }

    pub fn descendant_count(&self) -> usize {
        self.raw_node.descendant_count()
    }

    pub fn descendant_for_byte_range(&self, start: usize, end: usize) -> Option<Self> {
        self.raw_node
            .descendant_for_byte_range(start, end)
            .map(|node| Self {
                raw_tree: Arc::clone(&self.raw_tree),
                raw_node: node,
            })
    }

    pub fn named_descendant_for_byte_range(&self, start: usize, end: usize) -> Option<Self> {
        self.raw_node
            .named_descendant_for_byte_range(start, end)
            .map(|node| Self {
                raw_tree: Arc::clone(&self.raw_tree),
                raw_node: node,
            })
    }

    pub fn descendant_for_point_range(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<Self> {
        let start = tree_sitter::Point::new(start.0, start.1);
        let end = tree_sitter::Point::new(end.0, end.1);
        self.raw_node
            .descendant_for_point_range(start, end)
            .map(|node| Self {
                raw_tree: Arc::clone(&self.raw_tree),
                raw_node: node,
            })
    }

    pub fn named_descendant_for_point_range(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<Self> {
        let start = tree_sitter::Point::new(start.0, start.1);
        let end = tree_sitter::Point::new(end.0, end.1);
        self.raw_node
            .descendant_for_point_range(start, end)
            .map(|node| Self {
                raw_tree: Arc::clone(&self.raw_tree),
                raw_node: node,
            })
    }

    pub fn to_sexp(&self) -> String {
        self.raw_node.to_sexp()
    }

    pub fn utf8_text(&self, source: String) -> String {
        self.raw_node
            .utf8_text(source.as_bytes())
            .unwrap()
            .to_string()
    }

    pub fn walk(&self) -> TreeCursor {
        TreeCursor {
            raw_tree: Arc::clone(&self.raw_tree),
            raw_cursor: RefCell::new(self.raw_node.walk()),
        }
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self.raw_node)
    }

    pub fn to_s(&self) -> String {
        format!("{}", self.raw_node)
    }
}
