use magnus::{Error, RTypedData, Ruby, Value};

use std::cell::RefCell;
use std::ops::Range;

#[magnus::wrap(class = "TreeHouse::Tree")]
pub struct Tree {
    pub raw_tree: tree_sitter::Tree,
}

impl Tree {
    pub fn root_node(&self) -> Node<'_> {
        Node {
            raw_node: self.raw_tree.root_node(),
        }
    }

    pub fn walk(&self) -> TreeCursor<'_> {
        TreeCursor {
            raw_cursor: RefCell::new(self.raw_tree.walk()),
        }
    }
}

#[magnus::wrap(class = "TreeHouse::TreeCursor", unsafe_generics)]
pub struct TreeCursor<'cursor> {
    raw_cursor: RefCell<tree_sitter::TreeCursor<'cursor>>,
}

impl<'cursor> TreeCursor<'cursor> {
    pub fn node(&self) -> Node {
        Node {
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

#[magnus::wrap(class = "TreeHouse::Node", unsafe_generics)]
pub struct Node<'tree> {
    raw_node: tree_sitter::Node<'tree>,
}

impl<'tree> Node<'tree> {
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

    pub fn grammer_name(&self) -> &'static str {
        self.raw_node.grammar_name()
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

    pub fn start_byte(&self) -> usize {
        self.raw_node.start_byte()
    }

    pub fn end_byte(&self) -> usize {
        self.raw_node.end_byte()
    }

    pub fn byte_range(&self) -> Range<usize> {
        self.raw_node.byte_range()
    }

    pub fn child(&self, index: usize) -> Option<Node> {
        self.raw_node
            .child(index)
            .map(|node| Node { raw_node: node })
    }

    pub fn children_with_cursor<'cursor>(
        ruby: &Ruby,
        _rb: Value,
        cursor: &'cursor TreeCursor<'tree>,
    ) -> Result<(), Error> {
        let rb_self = ruby.current_receiver::<Value>().unwrap();
        let typed_data = RTypedData::from_value(rb_self).expect("Expected typed data");
        let node = typed_data.get::<Node>().expect("Expected Node");
        let mut borrowed = cursor.raw_cursor.borrow_mut();
        let iter = node.raw_node.children(&mut borrowed);
        iter.for_each(|node| {
            let node = Node { raw_node: node };
            let _: Value = ruby.yield_value(node).unwrap();
        });
        Ok(())
    }

    pub fn parent(&self) -> Option<Node> {
        self.raw_node.parent().map(|node| Node { raw_node: node })
    }

    pub fn to_sexp(&self) -> String {
        self.raw_node.to_sexp()
    }

    pub fn walk(&self) -> TreeCursor {
        TreeCursor {
            raw_cursor: RefCell::new(self.raw_node.walk()),
        }
    }
}
