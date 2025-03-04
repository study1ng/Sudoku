use std::ops::{BitAndAssign, SubAssign};

#[derive(Debug, Clone)]
pub(super) enum _Cell {
    Filled(u8),
    Unfilled(u16), // フラグで管理
}

impl _Cell {
    pub fn is_filled(&self) -> bool {
        matches!(self, Self::Filled(..))
    }
    pub fn bit(&self) -> u16 {
        return self.into();
    }
    pub fn to_determined_number(&self) -> Option<u8> {
        if self.bit().count_ones() == 1 {
            Some((self.bit().trailing_zeros() + 1) as u8)
        } else {
            None
        }
    }
    pub fn determine(&mut self) -> bool {
        if self.is_filled() {
            return false;
        }
        self.to_determined_number().and_then(|i| {*self = Self::Filled(i); Some(())}).is_some()
    }

    pub fn insert(&mut self, value: u8) -> bool {
        if self.is_filled() {
            false
        } else {
            *self = Self::Filled(value);
            true
        }
    }
}

impl Into<u16> for _Cell {
    fn into(self) -> u16 {
        match self {
            Self::Filled(i) => 1 << (i - 1),
            Self::Unfilled(b) => b,
        }
    }
}

impl Into<u16> for &_Cell {
    fn into(self) -> u16 {
        match self {
            &_Cell::Filled(i) => 1 << (i - 1),
            &_Cell::Unfilled(b) => b,
        }
    }
}

impl Into<u16> for &mut _Cell {
    fn into(self) -> u16 {
        match self {
            &mut _Cell::Filled(i) => 1 << (i - 1),
            &mut _Cell::Unfilled(b) => b,
        }
    }
}

impl<T> BitAndAssign<T> for _Cell
where
    T: Into<u16>,
{
    fn bitand_assign(&mut self, rhs: T) {
        match self {
            Self::Filled(..) => return,
            Self::Unfilled(b) => *self = Self::Unfilled(*b & rhs.into()),
        }
    }
}

impl<T> SubAssign<T> for _Cell where T: Into<u16>, {
    fn sub_assign(&mut self, rhs: T) {
        // shorthand for self &= ~rhs
        *self &= !rhs.into()
    }
}