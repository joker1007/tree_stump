#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[magnus::wrap(class = "TreeHouse::Point", free_immediately)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl Point {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn inspect(&self) -> String {
        format!("#<Point({}, {})>", self.row, self.column)
    }

    pub fn to_s(&self) -> String {
        format!("({}, {})", self.row, self.column)
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn into_raw(self) -> tree_sitter::Point {
        tree_sitter::Point {
            row: self.row,
            column: self.column,
        }
    }
}

impl From<tree_sitter::Point> for Point {
    fn from(point: tree_sitter::Point) -> Self {
        Self {
            row: point.row,
            column: point.column,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[magnus::wrap(class = "TreeHouse::Range", free_immediately)]
pub struct Range {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: Point,
    pub end_point: Point,
}

impl Range {
    pub fn new(start_byte: usize, end_byte: usize, start_point: &Point, end_point: &Point) -> Self {
        let start_point = start_point.to_owned();
        let end_point = end_point.to_owned();
        Self {
            start_byte,
            end_byte,
            start_point,
            end_point,
        }
    }

    pub fn get_start_byte(&self) -> usize {
        self.start_byte
    }

    pub fn get_end_byte(&self) -> usize {
        self.end_byte
    }

    pub fn get_start_point(&self) -> Point {
        self.start_point
    }

    pub fn get_end_point(&self) -> Point {
        self.end_point
    }

    pub fn inspect(&self) -> String {
        format!(
            "#<Range({}, {}, {:?}, {:?})>",
            self.start_byte, self.end_byte, self.start_point, self.end_point
        )
    }
    pub fn to_s(&self) -> String {
        format!("({}..{})", self.start_point.to_s(), self.end_point.to_s())
    }
}

impl From<tree_sitter::Range> for Range {
    fn from(range: tree_sitter::Range) -> Self {
        Self {
            start_byte: range.start_byte,
            end_byte: range.end_byte,
            start_point: range.start_point.into(),
            end_point: range.end_point.into(),
        }
    }
}
