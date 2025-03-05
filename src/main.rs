use std::{fs::{read, File}, io::Write};

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
    let mut string = read(".html").unwrap();
    let mut file = File::create("index.html").unwrap();
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..9 {
                if (1 << k) & puzzle[PuzzleIndex::new(i, j)].bit() != 0 {
                    string.push(k as u8 + b'1');
                }
            }
            string.push(b'/');
        }
    }
    string.pop();
    string.append(b"`</script><script src=\".js\"></script></body></html>".to_vec().as_mut());
    file.write_all(&string[..]).unwrap();
    // open ./index.html in browser
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("index.html")
        .spawn()
        .unwrap();
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg("index.html")
        .spawn()
        .unwrap();
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg("index.html")
        .spawn()
        .unwrap();
}
