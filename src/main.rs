mod lib;

fn main() {
    let line = "050000000300420000006000984607100000000200070000080090400600030700000005030008410"
        .to_string();
    let mut s = sudoku_from_line(line);
    lib::solve(&mut s);
}

fn sudoku_from_line(line: String) -> lib::Sudoku {
    let mut res = lib::Sudoku {
        board: line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10))
            .flatten()
            .collect(),
        col_memo: Vec::new(),
        row_memo: Vec::new(),
        square_memo: Vec::new(),
    };
    lib::initialize(&mut res);
    res
}