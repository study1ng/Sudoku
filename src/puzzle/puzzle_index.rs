#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PuzzleIndex {
    col: usize,
    row: usize,
}

impl PuzzleIndex {
    pub fn new(col: usize, row: usize) -> Self {
        Self {col, row}
    }

    pub fn row(&self) -> usize {
        self.row
    }
    
    pub fn col(&self) -> usize {
        self.col
    }

    pub fn block_idx(&self) -> usize {
        self.col / 3 * 3 + self.row / 3
    }
}
