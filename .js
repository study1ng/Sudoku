
let sudoku_table = document.getElementById("Sudoku").children[0]
FillSudoku(`
4/5/6/2/8/3/9/1/7/1/3/2/4/9/7/5/6/8/9/7/8/5/6/1/3/4/2/6/4/1/9/2/8/7/3/5/2/9/3/7/5/4/6/8/1/7/8/5/1/3/6/2/9/4/5/6/7/8/1/9/4/2/3/8/2/9/3/4/5/1/7/6/3/1/4/6/7/2/8/5/9

`)

function FillSudoku(text) {
    let i = 0;
    text = text.trim();
    text = text.split("/");
    text.forEach(t => {
        let c = i % 9;
        let r = i / 9 | 0;
        t = t.trim();
        if (t.length == 1) {
            sudoku_table.children[r].children[c].textContent = t;
            sudoku_table.children[r].children[c].classList.add("Sudoku" + t)
        } else {
            let table = document.createElement("table");
            let tr = document.createElement("tr");
            let td = document.createElement("td");
            for (let index = 0; index < 3; index++) {
                tr.appendChild(td.cloneNode(true));
            }
            for (let index = 0; index < 3; index++) {
                table.appendChild(tr.cloneNode(true));
            }
            for (let i = 0; i < 9; ++i) {
                let c = i % 3;
                let r = i / 3 | 0;
                table.children[r].children[c].pos = (i + 1);
            }
            t.split("").forEach(c => {
                let pc = parseInt(c) - 1;
                let row = pc / 3 | 0;
                let col = pc % 3;
                table.children[row].children[col].textContent = c;
                table.children[row].children[col].classList.add("Sudoku" + c);
            })
            sudoku_table.children[r].children[c].appendChild(table)
        }
        ++i;
    })
}

function toggleMouse(number) {
    return function (e) {
        if (!e.target.classList.contains("Sudoku" + e.target.pos)) {
            return;
        }
        document.querySelectorAll(`.Sudoku${number}`).forEach(e => { if (e.textContent != "") { e.classList.toggle("same-number") } });
    }
}

for (i = 1; i <= 9; ++i) {
    document.querySelectorAll(`.Sudoku${i}`).forEach(e => e.addEventListener("mouseenter", toggleMouse(i)));
    document.querySelectorAll(`.Sudoku${i}`).forEach(e => e.addEventListener("mouseleave", toggleMouse(i)));
}

function onMouseClick(e) {
    if (e.target.textContent == "") {
        e.target.classList.add("Sudoku" + e.target.pos);
        e.target.textContent = e.target.pos;
    } else {
        e.target.classList = [];
        e.target.textContent = "";
    }
}

for (i = 1; i <= 9; ++i) {
    document.querySelectorAll(`table td table td`).forEach(e => e.addEventListener("click", onMouseClick));
}
