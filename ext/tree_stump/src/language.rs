use std::cell::RefCell;

#[magnus::wrap(class = "TreeStump::LanguageRef", free_immediately, unsafe_generics)]
pub struct LanguageRef<'a> {
    pub raw_language_ref: tree_sitter::LanguageRef<'a>,
}
unsafe impl Send for LanguageRef<'_> {}

impl<'a> LanguageRef<'a> {
    pub fn version(&self) -> usize {
        self.raw_language_ref.version()
    }

    pub fn node_kind_count(&self) -> usize {
        self.raw_language_ref.node_kind_count()
    }

    pub fn parse_state_count(&self) -> usize {
        self.raw_language_ref.parse_state_count()
    }

    pub fn node_kind_for_id(&self, id: u16) -> Option<&'static str> {
        self.raw_language_ref.node_kind_for_id(id)
    }

    pub fn id_for_node_kind(&self, kind: String, named: bool) -> u16 {
        self.raw_language_ref.id_for_node_kind(kind.as_str(), named)
    }

    pub fn node_kind_is_named(&self, id: u16) -> bool {
        self.raw_language_ref.node_kind_is_named(id)
    }

    pub fn node_kind_is_visible(&self, id: u16) -> bool {
        self.raw_language_ref.node_kind_is_visible(id)
    }

    pub fn field_name_for_id(&self, id: u16) -> Option<&'static str> {
        self.raw_language_ref.field_name_for_id(id)
    }

    pub fn field_id_for_name(&self, name: String) -> Option<u16> {
        self.raw_language_ref
            .field_id_for_name(name.as_str())
            .map(|id| id.into())
    }

    pub fn next_state(&self, state: u16, id: u16) -> u16 {
        self.raw_language_ref.next_state(state, id)
    }

    pub fn lookahead_iterator(&self, state: u16) -> Option<LookaheadIterator> {
        self.raw_language_ref
            .lookahead_iterator(state)
            .map(|raw_iterator| LookaheadIterator {
                raw_iterator: RefCell::new(raw_iterator),
            })
    }
}

#[magnus::wrap(class = "TreeStump::LookaheadIterator")]
pub struct LookaheadIterator {
    raw_iterator: RefCell<tree_sitter::LookaheadIterator>,
}

impl LookaheadIterator {
    pub fn next(&self) -> Option<u16> {
        self.raw_iterator.borrow_mut().next()
    }

    pub fn current_symbol_name(&self) -> &'static str {
        self.raw_iterator.borrow().current_symbol_name()
    }
}
