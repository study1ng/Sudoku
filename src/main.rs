use std::{fs::File, io::Write};

use newsudoku::{puzzle::PuzzleIndex, Puzzle};

fn main() {
    let puzzle_seed =
        "*5**8*91***2****6****5*13*****9********7***8*7*5**6**4***********9**51763*4**2***";
    let mut puzzle = Puzzle::new();
    puzzle_seed
        .chars()
        .take(81)
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .for_each(|(i, c)| {
            let col = i / 9;
            let row = i % 9;
            puzzle.fill(PuzzleIndex::new(col, row), c.to_digit(10).unwrap() as u8);
        });
    while {
        let hash = puzzle.hash();
        puzzle.solve();
        hash != puzzle.hash()
    } {}
    puzzle.validate();
    let mut string = String::new();
    let mut file = File::create(".ans").unwrap();
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..9 {
                if (1 << k) & puzzle[PuzzleIndex::new(i, j)].bit() != 0 {
                    string += (k + 1).to_string().as_str();
                }
            }
            string += "/";
        }
    }
    string.pop();
    string.push('\n');
    file.write_all(string.as_bytes()).unwrap();
}
