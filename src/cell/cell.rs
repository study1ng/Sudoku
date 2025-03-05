use std::{fmt::Debug, ops::{BitAndAssign, SubAssign}};

use super::_cell::_Cell;
use crate::puzzle::PuzzleIndex;

#[derive(Clone)]
pub struct Cell {
    cell: _Cell,
    pos: PuzzleIndex,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_filled() {
            write!(f, "{}", self.cell.bit().trailing_zeros() + 1)
        } else {
            write!(f, "{:b}", self.cell.bit())
        }
    }
}

impl Cell {
    pub fn filled(value: u8, pos: PuzzleIndex) -> Self {
        Self {
            cell: _Cell::Filled(value),
            pos,
        }
    }

    pub fn pos(&self) -> PuzzleIndex {
        self.pos
    }

    pub fn unfilled(pos: PuzzleIndex) -> Self {
        Self {
            cell: _Cell::Unfilled(0b111111111),
            pos,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.cell.is_filled()
    }

    pub fn bit(&self) -> u16 {
        self.cell.bit()
    }

    pub fn row(&self) -> usize {
        self.pos.row()
    }

    pub fn col(&self) -> usize {
        self.pos.col()
    }

    pub fn block_idx(&self) -> usize {
        self.pos.block_idx()
    }

    pub fn determine(&mut self) -> bool {
        self.cell.determine()
    }

    pub fn insert(&mut self, value: u8) -> bool {
        self.cell.insert(value)
    }

    pub fn is_same_row(&self, other: &Self) -> bool {
        self.pos.row() == other.pos.row()
    }

    pub fn is_same_col(&self, other: &Self) -> bool {
        self.pos.col() == other.pos.col()
    }

    pub fn is_same_block(&self, other: &Self) -> bool {
        self.pos.block_idx() == other.pos.block_idx()
    }
}

impl<T> BitAndAssign<T> for Cell
where
    T: Into<u16>,
{
    fn bitand_assign(&mut self, rhs: T) {
        self.cell &= rhs;
    }
}

impl<T> SubAssign<T> for Cell
where
    T: Into<u16>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.cell -= rhs;
    }
}
impl<T> BitAndAssign<T> for &mut Cell
where
    T: Into<u16>,
{
    fn bitand_assign(&mut self, rhs: T) {
        self.cell &= rhs;
    }
}
impl<T> SubAssign<T> for &mut Cell
where
    T: Into<u16>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.cell -= rhs;
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Cell {}
