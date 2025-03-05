use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

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
        self.hidden_pair();
        self.hidden_triple();
        self.x_wing();
        self.xy_wing();
    }

    fn x_wing(&mut self) {
        // 各数字について, ある二つの行/列が存在して, その行/列において数字が同じ二つの位置にのみ存在していた場合, その二つの位置の属する列/行からその数字を取り除く.
        for i in 0..9 {
            for j in i + 1..9 {
                let tar = self.col(i).to_number_appearance();
                let tar2 = self.col(j).to_number_appearance();
                for k in 0..9 {
                    if tar[k].count_ones() != 2 {
                        continue;
                    }
                    if tar2[k] != tar[k] {
                        continue;
                    }
                    for t in 0..9 {
                        if (1 << t) & tar[k] == 0 {
                            continue;
                        }
                        // t列の他のセルからkを取り除く.
                        let mut row = self.row_mut(t);
                        for l in 0..9 {
                            if l == i || l == j {
                                continue;
                            }
                            row[t] -= 1u16 << k;
                        }
                    }
                }
                let tar = self.row(i).to_number_appearance();
                let tar2 = self.row(j).to_number_appearance();
                for k in 0..9 {
                    if tar[k].count_ones() != 2 {
                        continue;
                    }
                    if tar2[k] != tar[k] {
                        continue;
                    }
                    for t in 0..9 {
                        if (1 << t) & tar[k] == 0 {
                            continue;
                        }
                        // t行の他のセルからkを取り除く.
                        let mut col = self.col_mut(t);
                        for l in 0..9 {
                            if l == i || l == j {
                                continue;
                            }
                            col[t] -= 1u16 << k;
                        }
                    }
                }
            }
        }
    }

    fn xy_wing(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                let idx = PuzzleIndex::new(i, j);
                if self[idx].bit().count_ones() != 2 {
                    continue;
                }
                let mut candidates = HashSet::new();
                let tar = self.row(idx.row());
                for i in 0..9 {
                    if tar[i].is_filled()
                        || tar[i].bit().count_ones() != 2
                        || (tar[i].bit() & self[idx].bit()).count_ones() != 1
                    {
                        continue;
                    }
                    candidates.insert(tar[i].pos());
                }
                let tar = self.col(idx.col());
                for i in 0..9 {
                    if tar[i].is_filled()
                        || tar[i].bit().count_ones() != 2
                        || (tar[i].bit() & self[idx].bit()).count_ones() != 1
                    {
                        continue;
                    }
                    candidates.insert(tar[i].pos());
                }
                let tar = self.block(idx.block_idx());
                for i in 0..9 {
                    if tar[i].is_filled()
                        || tar[i].bit().count_ones() != 2
                        || (tar[i].bit() & self[idx].bit()).count_ones() != 1
                    {
                        continue;
                    }
                    candidates.insert(tar[i].pos());
                }
                fn is_same_group(i: &Cell, j: &Cell, k: &Cell) -> bool {
                    (i.row() == j.row() && i.row() == k.row())
                        || (i.col() == j.col() && i.col() == k.col())
                        || (i.block_idx() == j.block_idx() && i.block_idx() == k.block_idx())
                }
                // 立っているビットの数が2かつself[idx]と一つだけ立っているビットが共通しているセルの集合
                let candidates = candidates.into_iter().collect::<Vec<_>>();
                for i in 0..candidates.len() {
                    for j in i + 1..candidates.len() {
                        let i = candidates[i];
                        let j = candidates[j];
                        // 三つのセルの論理和の立っているビットが3つで, self[idx]は二つのセルと異なる共通セルを持つ.
                        if !is_same_group(&self[idx], &self[i], &self[j])
                            && (self[i].bit()
                                | self[j].bit()
                                | self[idx].bit())
                            .count_ones()
                                == 3
                            && (self[i].bit() & self[idx].bit()) != (self[j].bit() & self[idx].bit())
                        {
                            let common = self[i].bit()
                                & self[j].bit();
                            // candidates[i]とcandidates[j]の共通の影響範囲からcommonを取り除く
                            // お互いの行/列の交差点
                            self[PuzzleIndex::new(i.col(), j.row())] -= common;
                            self[PuzzleIndex::new(j.col(), i.row())] -= common;
                            // iのブロックかつjの行/列
                            let mut block = self.block_mut(i.block_idx());
                            for k in 0..9 {
                                if block[k].pos() == i || block[k].pos() == j {
                                    continue;
                                }
                                if block[k].col() == j.col() {
                                    block[k] -= common;
                                }
                                if block[k].row() == j.row() {
                                    block[k] -= common;
                                }
                            }
                            // jのブロックかつiの行/列
                            let mut block = self.block_mut(j.block_idx());
                            for k in 0..9 {
                                if block[k].pos() == i || block[k].pos() == j {
                                    continue;
                                }
                                if block[k].col() == i.col() {
                                    block[k] -= common;
                                }
                                if block[k].row() == i.row() {
                                    block[k] -= common;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn naked_triple(&mut self) {
        for i in 0..9 {
            self.block_mut(i).naked_triple();
            self.col_mut(i).naked_triple();
            self.row_mut(i).naked_triple();
        }
    }

    fn naked_pair(&mut self) {
        // 各行/列/ブロックにおいて, ある二つのセルのビット和のcount_onesが2に等しいならば, 他のセルからそのビット和を取り除く.
        for i in 0..9 {
            self.block_mut(i).naked_pair();
            self.col_mut(i).naked_pair();
            self.row_mut(i).naked_pair();
        }
    }

    fn hidden_single(&mut self) {
        // 各行/列/ブロックにおいて, あるビットが他のセルに含まれていないならば, そのセルにそのビットを入れる.
        for i in 0..9 {
            self.block_mut(i).hidden_single();
            self.col_mut(i).hidden_single();
            self.row_mut(i).hidden_single();
        }
    }

    fn hidden_pair(&mut self) {
        for i in 0..9 {
            self.block_mut(i).hidden_pair();
            self.col_mut(i).hidden_pair();
            self.row_mut(i).hidden_pair();
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

    fn hidden_triple(&mut self) {
        for i in 0..9 {
            self.block_mut(i).hidden_triple();
            self.col_mut(i).hidden_triple();
            self.row_mut(i).hidden_triple();
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
