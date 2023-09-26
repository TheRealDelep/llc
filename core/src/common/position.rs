use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct FilePosition {
    pub row: usize,
    pub col: usize
}

#[derive(Debug, Clone, Copy)]
pub struct FileSpan {
    pub begin: FilePosition,
    pub end: FilePosition
}

impl FilePosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self {row, col}
    }
}

impl FileSpan {
    pub fn new(begin: FilePosition, end: FilePosition) -> Self {
        Self {begin, end}
    }

    pub fn combine(first: &FileSpan, last: &FileSpan) -> Self {
        Self {
            begin: first.begin,
            end: last.end
        }
    }
}

impl Display for FilePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {0}, col {1}", self.row, self.col)
    }
}

impl Display for FileSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from {0}, to {1}", self.begin, self.end)
    }   
}