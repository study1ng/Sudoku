use std::ops::{Index, IndexMut};

use crate::cell::Cell;

use super::{Puzzle, PuzzleIndex};
#[derive(Clone, Copy, Debug)]
pub(super) enum SliceType {
    Row(usize),
    Col(usize),
    Block(usize),
}

pub struct PuzzleSlice<'a> {
    puzzle: &'a Puzzle,
    pub(super) type_: SliceType,
}

impl<'a> PuzzleSlice<'a> {
    pub(super) fn new(puzzle: &'a Puzzle, type_: SliceType) -> Self {
        Self { puzzle, type_ }
    }

    pub fn iter(&self) -> PuzzleIter<'_> {
        PuzzleIter::new(self, 0)
    }
}

impl<'a> Index<usize> for PuzzleSlice<'a> {
    type Output = Cell;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= 9 {
            panic!("PuzzleSlice only accept index < 9");
        }
        match self.type_ {
            SliceType::Row(n) => &self.puzzle[PuzzleIndex::new(index, n)],
            SliceType::Col(n) => &self.puzzle[PuzzleIndex::new(n, index)],
            SliceType::Block(n) => {
                let row = n % 3 * 3;
                let col = n / 3 * 3;
                let row = row + (index % 3);
                let col = col + (index / 3);
                &self.puzzle[PuzzleIndex::new(col, row)]
            }
        }
    }
}

impl<'a> From<PuzzleSliceMut<'a>> for PuzzleSlice<'a> {
    fn from(value: PuzzleSliceMut<'a>) -> Self {
        PuzzleSlice {
            puzzle: value.puzzle,
            type_: value.type_,
        }
    }
}

impl<'a> From<&'a PuzzleSliceMut<'a>> for &'a PuzzleSlice<'a> {
    fn from(value: &'a PuzzleSliceMut<'a>) -> Self {
        unsafe { &*(value as *const PuzzleSliceMut<'a> as *const PuzzleSlice<'a>) }
    }
}

pub struct PuzzleSliceMut<'a> {
    puzzle: &'a mut Puzzle,
    pub(super) type_: SliceType,
}

impl<'a> PuzzleSliceMut<'a> {
    pub(super) fn new(puzzle: &'a mut Puzzle, type_: SliceType) -> Self {
        Self { puzzle, type_ }
    }

    pub fn iter(&self) -> PuzzleIter<'_> {
        PuzzleIter::new(self, 0)
    }

    pub fn chunk_bit_sum(&self, chunk: usize) -> u16 {
        self[chunk * 3].bit() | self[chunk * 3 + 1].bit() | self[chunk * 3 + 2].bit()
    }

    pub fn stride_bit_sum(&self, stride: usize) -> u16 {
        self[stride].bit() | self[stride + 3].bit() | self[stride + 6].bit()
    }

    pub fn chunk_bit_product(&self, chunk: usize) -> u16 {
        self[chunk].bit() & self[chunk + 1].bit() & self[chunk + 2].bit()
    }

    pub fn stride_bit_product(&self, stride: usize) -> u16 {
        self[stride].bit() & self[stride + 3].bit() & self[stride + 6].bit()
    }

    pub fn determine(&mut self, index: usize) {
        let pos = self[index].pos();
        self.puzzle.determine(pos);
    }
}

impl<'a> IntoIterator for &'a PuzzleSliceMut<'a> {
    type Item = &'a Cell;

    type IntoIter = PuzzleIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Index<usize> for PuzzleSliceMut<'a> {
    type Output = Cell;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= 9 {
            panic!("PuzzleSlice only accept index < 9");
        }
        match self.type_ {
            SliceType::Row(row) => &self.puzzle[PuzzleIndex::new(index, row)],
            SliceType::Col(col) => &self.puzzle[PuzzleIndex::new(col, index)],
            SliceType::Block(n) => {
                let row = n % 3 * 3;
                let col = n / 3 * 3;
                let row = row + (index % 3);
                let col = col + (index / 3);
                &self.puzzle[PuzzleIndex::new(col, row)]
            }
        }
    }
}

impl<'a> IndexMut<usize> for PuzzleSliceMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= 9 {
            panic!("PuzzleSlice only accept index < 9");
        }
        match self.type_ {
            SliceType::Row(row) => &mut self.puzzle[PuzzleIndex::new(index, row)],
            SliceType::Col(col) => &mut self.puzzle[PuzzleIndex::new(col, index)],
            SliceType::Block(n) => {
                let row = n % 3 * 3;
                let col = n / 3 * 3;
                let row = row + (index % 3);
                let col = col + (index / 3);
                &mut self.puzzle[PuzzleIndex::new(col, row)]
            }
        }
    }
}

pub struct PuzzleIter<'a> {
    slice: &'a PuzzleSlice<'a>,
    idx: usize,
}

impl<'a> PuzzleIter<'a> {
    fn new<T>(slice: &'a T, idx: usize) -> Self
    where
        &'a T: Into<&'a PuzzleSlice<'a>>,
    {
        Self {
            slice: slice.into(),
            idx,
        }
    }
}

impl<'a> Iterator for PuzzleIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 9 {
            None
        } else {
            let ret = Some(&self.slice[self.idx]);
            self.idx += 1;
            ret
        }
    }
}
