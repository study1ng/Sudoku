use std::ops::{Index, IndexMut};

use crate::cell::Cell;

use super::{
    puzzle_slice::{PuzzleSlice, SliceType},
    PuzzleIndex, PuzzleSliceMut,
};

#[derive(Clone)]
pub struct Puzzle {
    inner: Vec<Vec<Cell>>,
}

impl Puzzle {
    pub fn new() -> Self {
        let mut inner = vec![vec![]; 9];
        for i in 0..9 {
            for j in 0..9 {
                inner[i].push(Cell::unfilled(PuzzleIndex::new(i, j)));
            }
        }
        Puzzle { inner }
    }

    pub fn fill(&mut self, pos: PuzzleIndex, v: u8) {
        if !self[pos].insert(v) {
            return;
        };
        self.propagate(pos);
    }

    pub(super) fn determine(&mut self, pos: PuzzleIndex) {
        if self[pos].determine() {
            self.propagate(pos);
        }
    }

    fn propagate(&mut self, pos: PuzzleIndex) -> bool {
        if !self[pos].is_filled() {
            return false;
        }
        let mut determined = vec![];
        let bit = self[pos].bit();
        let mut col = self.col_mut(pos.col());
        for i in 0..9 {
            col[i] -= bit;
            if col[i].determine() {
                determined.push(col[i].pos());
            }
        }
        let mut row = self.row_mut(pos.row());
        for i in 0..9 {
            row[i] -= bit;
            if row[i].determine() {
                determined.push(row[i].pos());
            }
        }
        let mut block = self.block_mut(pos.block_idx());
        for i in 0..9 {
            block[i] -= bit;
            if block[i].determine() {
                determined.push(block[i].pos());
            }
        }
        determined.into_iter().for_each(|c| {
            self.propagate(c);
        });
        true
    }

    fn col(&self, idx: usize) -> PuzzleSlice {
        PuzzleSlice::new(self, SliceType::Col(idx))
    }

    fn row(&self, idx: usize) -> PuzzleSlice {
        PuzzleSlice::new(self, SliceType::Row(idx))
    }

    fn block(&self, idx: usize) -> PuzzleSlice {
        PuzzleSlice::new(self, SliceType::Block(idx))
    }

    fn col_mut(&mut self, idx: usize) -> PuzzleSliceMut {
        PuzzleSliceMut::new(self, SliceType::Col(idx))
    }

    fn row_mut(&mut self, idx: usize) -> PuzzleSliceMut {
        PuzzleSliceMut::new(self, SliceType::Row(idx))
    }

    fn block_mut(&mut self, idx: usize) -> PuzzleSliceMut {
        PuzzleSliceMut::new(self, SliceType::Block(idx))
    }

    pub fn validate(&self) -> bool {
        fn check(s: PuzzleSlice) -> bool {
            // 同じ行/列/ブロックにおいてfilledが同じ値を持つことがない.
            if s.iter().filter(|c| c.is_filled()).count()
                != s.iter()
                    .filter(|c| c.is_filled())
                    .map(|c| c.bit() as usize)
                    .sum::<usize>()
                    .count_ones() as usize
            {
                println!(
                    "{:?} filledが同じ値を持っている {:b}",
                    s.type_,
                    s.iter()
                        .filter(|c| c.is_filled())
                        .map(|c| c.bit() as usize)
                        .sum::<usize>()
                );
                return false;
            }
            // 同じ行/列/ブロックにおいて, すべてのビット和をとると0b111111111となる.
            if s.iter().map(|c| c.bit()).fold(0, |i, b| i | b) != 0b111111111 {
                println!(
                    "{:?} ビット和が0b111111111とならない {:b}",
                    s.type_,
                    s.iter().map(|c| c.bit()).fold(0, |i, b| i | b)
                );
                return false;
            }
            // 同じ行/列/ブロックにおいて, filledとunfilledのビット和のビット和は0
            if s.iter()
                .filter(|c| c.is_filled())
                .map(|c| c.bit())
                .fold(0, |i, b| i | b)
                & s.iter()
                    .filter(|c| !c.is_filled())
                    .map(|c| c.bit())
                    .fold(0, |i, b| i | b)
                != 0
            {
                println!(
                    "{:?} filledとunfilledが重なっている {}",
                    s.type_,
                    s.iter()
                        .filter(|c| c.is_filled())
                        .map(|c| c.bit())
                        .fold(0, |i, b| i | b)
                        & s.iter()
                            .filter(|c| !c.is_filled())
                            .map(|c| c.bit())
                            .fold(0, |i, b| i | b)
                );
                return false;
            }
            true
        }
        (0..9)
            .map(|i| self.block(i))
            .chain((0..9).map(|i| self.col(i)))
            .chain((0..9).map(|i| self.row(i)))
            .map(|rcb| check(rcb))
            .all(|b| b)
    }

    pub fn hash(&self) -> String {
        let mut ans = String::new();
        for i in 0..9 {
            for j in 0..9 {
                let idx = PuzzleIndex::new(i, j);
                ans += self[idx].bit().to_string().as_str();
                ans.push('/');
            }
        }
        ans.pop();
        ans
    }

    fn naked_single(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                self.determine(PuzzleIndex::new(i, j));
            }
        }
    }

    pub fn solve(&mut self) {
        self.naked_single();
        self.hidden_single();
        self.box_line_reduction();
        self.naked_pair();
        self.naked_triple();
    }

    fn naked_triple(&mut self) {
        for i in 0..9 {
            let mut tar = self.block_mut(i);
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in i + 1..9 {
                    if tar[j].is_filled() {
                        continue;
                    }
                    for k in j + 1..9 {
                        if tar[k].is_filled() {
                            continue;
                        }
                        let bit = tar[i].bit() | tar[j].bit() | tar[k].bit();
                        if bit.count_ones() != 3 {
                            continue;
                        }
                        for l in 0..9 {
                            if i == l || j == l || k == l {
                                continue;
                            }
                            tar[l] -= bit;
                            tar.determine(l);
                        }
                    }
                }
            }
            let mut tar = self.col_mut(i);
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in i + 1..9 {
                    if tar[j].is_filled() {
                        continue;
                    }
                    for k in j + 1..9 {
                        if tar[k].is_filled() {
                            continue;
                        }
                        let bit = tar[i].bit() | tar[j].bit() | tar[k].bit();
                        if bit.count_ones() != 3 {
                            continue;
                        }
                        for l in 0..9 {
                            if i == l || j == l || k == l {
                                continue;
                            }
                            tar[l] -= bit;
                            tar.determine(l);
                        }
                    }
                }
            }
            let mut tar = self.row_mut(i);
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in i + 1..9 {
                    if tar[j].is_filled() {
                        continue;
                    }
                    for k in j + 1..9 {
                        if tar[k].is_filled() {
                            continue;
                        }
                        let bit = tar[i].bit() | tar[j].bit() | tar[k].bit();
                        if bit.count_ones() != 3 {
                            continue;
                        }
                        for l in 0..9 {
                            if i == l || j == l || k == l {
                                continue;
                            }
                            tar[l] -= bit;
                            tar.determine(l);
                        }
                    }
                }
            }
        }
    }

    fn naked_pair(&mut self) {
        // 各行/列/ブロックにおいて, ある二つのセルのビット和のcount_onesが2に等しいならば, 他のセルからそのビット和を取り除く.
        for i in 0..9 {
            let mut tar = self.block_mut(i);
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in i + 1..9 {
                    if tar[j].is_filled() {
                        continue;
                    }
                    let bit = tar[i].bit() | tar[j].bit();
                    if bit.count_ones() != 2 {
                        continue;
                    }
                    for k in 0..9 {
                        if i == k || j == k {
                            continue;
                        }
                        tar[k] -= bit;
                        tar.determine(k);
                    }
                }
            }
            let mut tar = self.col_mut(i);
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in i + 1..9 {
                    if tar[j].is_filled() {
                        continue;
                    }
                    let bit = tar[i].bit() | tar[j].bit();
                    if bit.count_ones() != 2 {
                        continue;
                    }
                    for k in 0..9 {
                        if i == k || j == k {
                            continue;
                        }
                        tar[k] -= bit;
                        tar.determine(k);
                    }
                }
            }
            let mut tar = self.row_mut(i);
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in i + 1..9 {
                    if tar[j].is_filled() {
                        continue;
                    }
                    let bit = tar[i].bit() | tar[j].bit();
                    if bit.count_ones() != 2 {
                        continue;
                    }
                    for k in 0..9 {
                        if i == k || j == k {
                            continue;
                        }
                        tar[k] -= bit;
                        tar.determine(k);
                    }
                }
            }
        }
    }

    fn hidden_single(&mut self) {
        // 各行/列/ブロックにおいて, あるビットが他のセルに含まれていないならば, そのセルにそのビットを入れる.
        for i in 0..9 {
            let tar = self.block(i);
            let mut cnt = [0; 9];
            let mut pos = [PuzzleIndex::new(0, 0); 9];
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in 0..9 {
                    if tar[i].bit() & (1 << j) != 0 {
                        cnt[j] += 1;
                        pos[j] = tar[i].pos();
                    }
                }
            }
            for i in 0..9 {
                if cnt[i] == 1 {
                    self.fill(pos[i], i as u8 + 1);
                }
            }
            let tar = self.col(i);
            let mut cnt = [0; 9];
            let mut pos = [PuzzleIndex::new(0, 0); 9];
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in 0..9 {
                    if tar[i].bit() & (1 << j) != 0 {
                        cnt[j] += 1;
                        pos[j] = tar[i].pos();
                    }
                }
            }
            for i in 0..9 {
                if cnt[i] == 1 {
                    self.fill(pos[i], i as u8 + 1);
                }
            }
            let tar = self.row(i);
            let mut cnt = [0; 9];
            let mut pos = [PuzzleIndex::new(0, 0); 9];
            for i in 0..9 {
                if tar[i].is_filled() {
                    continue;
                }
                for j in 0..9 {
                    if tar[i].bit() & (1 << j) != 0 {
                        cnt[j] += 1;
                        pos[j] = tar[i].pos();
                    }
                }
            }
            for i in 0..9 {
                if cnt[i] == 1 {
                    self.fill(pos[i], i as u8 + 1);
                }
            }
        }
    }

    fn box_line_reduction(&mut self) {
        // 各列/行/ブロックにおいて, あるビットが一つの部分(行や列, ブロック)にのみ含まれていた場合, その部分の全体からそのビットを取り除く
        for i in 0..9 {
            let block = self.block_mut(i);
            let c0_idx = block[0].col();
            let c1_idx = block[3].col();
            let c2_idx = block[6].col();
            let r0_idx = block[0].row();
            let r1_idx = block[1].row();
            let r2_idx = block[2].row();
            let c0 = block.chunk_bit_sum(0);
            let c1 = block.chunk_bit_sum(1);
            let c2 = block.chunk_bit_sum(2);
            let r0 = block.stride_bit_sum(0);
            let r1 = block.stride_bit_sum(1);
            let r2 = block.stride_bit_sum(2);
            let only_c0 = c0 & (c0 ^ c1) & (c0 ^ c2);
            let only_c1 = c1 & (c1 ^ c0) & (c1 ^ c2);
            let only_c2 = c2 & (c2 ^ c0) & (c2 ^ c1);
            let only_r0 = r0 & (r0 ^ r1) & (r0 ^ r2);
            let only_r1 = r1 & (r1 ^ r0) & (r1 ^ r2);
            let only_r2 = r2 & (r2 ^ r0) & (r2 ^ r1);
            for j in 0..9 {
                let tar = &mut self.col_mut(c0_idx)[j];
                if tar.pos().block_idx() == i {
                    continue;
                }
                *tar -= only_c0;
                self.determine(PuzzleIndex::new(j, c0_idx));
            }
            for j in 0..9 {
                let tar = &mut self.col_mut(c1_idx)[j];
                if tar.pos().block_idx() == i {
                    continue;
                }
                *tar -= only_c1;
                self.determine(PuzzleIndex::new(j, c1_idx));
            }
            for j in 0..9 {
                let tar = &mut self.col_mut(c2_idx)[j];
                if tar.pos().block_idx() == i {
                    continue;
                }
                *tar -= only_c2;
                self.determine(PuzzleIndex::new(j, c2_idx));
            }
            for j in 0..9 {
                let tar = &mut self.row_mut(r0_idx)[j];
                if tar.pos().block_idx() == i {
                    continue;
                }
                *tar -= only_r0;
                self.determine(PuzzleIndex::new(j, r0_idx));
            }
            for j in 0..9 {
                let tar = &mut self.row_mut(r1_idx)[j];
                if tar.pos().block_idx() == i {
                    continue;
                }
                *tar -= only_r1;
                self.determine(PuzzleIndex::new(j, r1_idx));
            }
            for j in 0..9 {
                let tar = &mut self.row_mut(r2_idx)[j];
                if tar.pos().block_idx() == i {
                    continue;
                }
                *tar -= only_r2;
                self.determine(PuzzleIndex::new(j, r2_idx));
            }
            let row = self.row_mut(i);
            let b0_idx = row[0].block_idx();
            let b1_idx = row[3].block_idx();
            let b2_idx = row[6].block_idx();
            let b0 = row.chunk_bit_sum(0);
            let b1 = row.chunk_bit_sum(1);
            let b2 = row.chunk_bit_sum(2);
            let only_b0 = b0 & (b0 ^ b1) & (b0 ^ b2);
            let only_b1 = b1 & (b1 ^ b0) & (b1 ^ b2);
            let only_b2 = b2 & (b2 ^ b0) & (b2 ^ b1);
            for j in 0..9 {
                let tar = &mut self.block_mut(b0_idx)[j];
                if tar.pos().row() == i {
                    continue;
                }
                *tar -= only_b0;
                let idx = self.block(b0_idx)[j].pos();
                self.determine(idx);
            }
            for j in 0..9 {
                let tar = &mut self.block_mut(b1_idx)[j];
                if tar.pos().row() == i {
                    continue;
                }
                *tar -= only_b1;
                let idx = self.block(b1_idx)[j].pos();
                self.determine(idx);
            }
            for j in 0..9 {
                let tar = &mut self.block_mut(b2_idx)[j];
                if tar.pos().row() == i {
                    continue;
                }
                *tar -= only_b2;
                let idx = self.block(b2_idx)[j].pos();
                self.determine(idx);
            }
            let col = self.col_mut(i);
            let b0_idx = col[0].block_idx();
            let b1_idx = col[3].block_idx();
            let b2_idx = col[6].block_idx();
            let b0 = col.chunk_bit_sum(0);
            let b1 = col.chunk_bit_sum(1);
            let b2 = col.chunk_bit_sum(2);
            let only_b0 = b0 & (b0 ^ b1) & (b0 ^ b2);
            let only_b1 = b1 & (b1 ^ b0) & (b1 ^ b2);
            let only_b2 = b2 & (b2 ^ b0) & (b2 ^ b1);
            for j in 0..9 {
                let tar = &mut self.block_mut(b0_idx)[j];
                if tar.pos().col() == i {
                    continue;
                }
                *tar -= only_b0;
                let idx = self.block(b0_idx)[j].pos();
                self.determine(idx);
            }
            for j in 0..9 {
                let tar = &mut self.block_mut(b1_idx)[j];
                if tar.pos().col() == i {
                    continue;
                }
                *tar -= only_b1;
                let idx = self.block(b1_idx)[j].pos();
                self.determine(idx);
            }
            for j in 0..9 {
                let tar = &mut self.block_mut(b2_idx)[j];
                if tar.pos().col() == i {
                    continue;
                }
                *tar -= only_b2;
                let idx = self.block(b2_idx)[j].pos();
                self.determine(idx);
            }
        }
    }
}

impl Index<PuzzleIndex> for Puzzle {
    type Output = Cell;
    fn index(&self, index: PuzzleIndex) -> &Self::Output {
        &self.inner[index.col()][index.row()]
    }
}

impl IndexMut<PuzzleIndex> for Puzzle {
    fn index_mut(&mut self, index: PuzzleIndex) -> &mut Self::Output {
        &mut self.inner[index.col()][index.row()]
    }
}
