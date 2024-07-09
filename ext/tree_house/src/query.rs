use std::cell::RefCell;

use magnus::{
    symbol::IntoSymbol,
    value::{InnerRef, Opaque, ReprValue},
    Class, Error, RArray, RStruct, RTypedData, Ruby, Value,
};

use crate::{data::Point, tree::Node, util::build_error, QUERY_CAPTURE_CLASS};

#[magnus::wrap(class = "TreeHouse::Query", free_immediately)]
pub struct Query {
    pub raw_query: RefCell<tree_sitter::Query>,
}

impl Query {
    pub fn new(language: &tree_sitter::Language, source: String) -> Result<Self, magnus::Error> {
        let raw_query = tree_sitter::Query::new(language, source.as_str());
        let raw_query = raw_query.map_err(|e| build_error(e.to_string()));
        raw_query.map(|q| Self {
            raw_query: RefCell::new(q),
        })
    }

    pub fn start_byte_for_pattern(&self, pattern_index: usize) -> usize {
        self.raw_query
            .borrow()
            .start_byte_for_pattern(pattern_index)
    }

    pub fn pattern_count(&self) -> usize {
        self.raw_query.borrow().pattern_count()
    }

    pub fn capture_names(&self) -> Vec<String> {
        self.raw_query
            .borrow()
            .capture_names()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn capture_quantifiers(&self, index: usize) -> Result<RArray, Error> {
        let raw_query = self.raw_query.borrow();
        let quantifiers = raw_query
            .capture_quantifiers(index)
            .iter()
            .map(|q| format!("{:?}", q).into_symbol());
        let ruby = Ruby::get().expect("Ruby is not initialized");
        let array = ruby.ary_from_iter(quantifiers);
        Ok(array)
    }

    pub fn capture_index_for_name(&self, name: String) -> Option<u32> {
        self.raw_query
            .borrow()
            .capture_index_for_name(name.as_str())
    }

    pub fn disable_capture(&self, name: String) {
        self.raw_query.borrow_mut().disable_capture(&name);
    }

    pub fn disable_pattern(&self, index: usize) {
        self.raw_query.borrow_mut().disable_pattern(index);
    }

    pub fn is_pattern_rooted(&self, index: usize) -> bool {
        self.raw_query.borrow().is_pattern_rooted(index)
    }

    pub fn is_pattern_guaranteed_at_step(&self, index: usize) -> bool {
        self.raw_query.borrow().is_pattern_guaranteed_at_step(index)
    }
}

#[magnus::wrap(class = "TreeHouse::QueryMatch", free_immediately, unsafe_generics)]
pub struct QueryMatch {
    pattern_index: usize,
    captures: Opaque<RArray>,
}

impl QueryMatch {
    pub fn pattern_index(&self) -> usize {
        self.pattern_index
    }

    pub fn captures(ruby: &Ruby, rb_self: Value) -> Result<RArray, Error> {
        let typed_data = RTypedData::from_value(rb_self).expect("Expected typed data");
        let query_match = typed_data.get::<Self>()?;
        Ok(ruby.get_inner(query_match.captures))
    }
}

#[magnus::wrap(class = "TreeHouse::QueryCursor", free_immediately)]
pub struct QueryCursor {
    raw_cursor: RefCell<tree_sitter::QueryCursor>,
}

impl QueryCursor {
    pub fn new() -> Self {
        Self {
            raw_cursor: RefCell::new(tree_sitter::QueryCursor::new()),
        }
    }

    pub fn match_limit(&self) -> u32 {
        self.raw_cursor.borrow().match_limit()
    }

    pub fn set_match_limit(&self, limit: u32) {
        self.raw_cursor.borrow_mut().set_match_limit(limit);
    }

    pub fn did_exceed_match_limit(&self) -> bool {
        self.raw_cursor.borrow().did_exceed_match_limit()
    }

    pub fn matches<'query, 'tree>(
        ruby: &Ruby,
        rb_self: Value,
        query: &'query Query,
        node: &Node<'tree>,
        source: String,
    ) -> Result<Value, Error> {
        let typed_data = RTypedData::from_value(rb_self).expect("Expected typed data");
        let query_cursor = typed_data.get::<Self>()?;
        let mut cursor = query_cursor.raw_cursor.borrow_mut();
        let query = query.raw_query.borrow();
        let matches = cursor.matches(&query, node.get_raw_node(), source.as_bytes());
        let struct_class = QUERY_CAPTURE_CLASS.get_inner_ref_with(ruby);

        for m in matches {
            let r_array = ruby.ary_new();
            for c in m.captures {
                let r_struct =
                    RStruct::from_value(struct_class.new_instance((Node::new(c.node), c.index))?)
                        .unwrap();
                r_array.push(r_struct)?;
            }
            let match_obj = QueryMatch {
                pattern_index: m.pattern_index,
                captures: Opaque::from(r_array),
            };
            let _: Value = ruby.yield_value(match_obj)?;
        }

        Ok(rb_self)
    }

    pub fn set_byte_range(
        _ruby: &Ruby,
        rb_self: Value,
        range: magnus::Range,
    ) -> Result<Value, Error> {
        let typed_data = RTypedData::from_value(rb_self).expect("Expected typed data");
        let query_cursor = typed_data.get::<Self>()?;
        let mut cursor = query_cursor.raw_cursor.borrow_mut();
        let len = range.funcall("size", ())?;
        let std_range = range.to_range_with_len(len)?;
        cursor.set_byte_range(std_range);
        Ok(rb_self)
    }

    pub fn set_point_range(
        _ruby: &Ruby,
        rb_self: Value,
        range: magnus::Range,
    ) -> Result<Value, Error> {
        let excl = range.excl();

        if excl {
            return Err(build_error("Point range must be inclusive".to_string()));
        }

        let start: Value = range.beg()?;
        let end: Value = range.end()?;

        let start_typed_data = RTypedData::from_value(start).expect("Expected typed data");
        let start = start_typed_data.get::<Point>()?;

        let end_typed_data = RTypedData::from_value(end).expect("Expected typed data");
        let end = end_typed_data.get::<Point>()?;

        let point_range = start.into_raw()..end.into_raw();

        let typed_data = RTypedData::from_value(rb_self).expect("Expected typed data");
        let query_cursor = typed_data.get::<Self>()?;
        let mut cursor = query_cursor.raw_cursor.borrow_mut();
        cursor.set_point_range(point_range);
        Ok(rb_self)
    }

    pub fn set_max_start_depth(
        _ruby: &Ruby,
        rb_self: Value,
        depth: Option<u32>,
    ) -> Result<Value, Error> {
        let typed_data = RTypedData::from_value(rb_self).expect("Expected typed data");
        let query_cursor = typed_data.get::<Self>()?;
        let mut cursor = query_cursor.raw_cursor.borrow_mut();
        cursor.set_max_start_depth(depth);
        Ok(rb_self)
    }
}
