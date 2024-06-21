pub struct Sudoku {
    pub board: Vec<u32>,
    pub row_memo: Vec<u32>,
    pub col_memo: Vec<u32>,
    pub square_memo: Vec<u32>,
}

pub fn initialize(input: &mut Sudoku) {
    for idx in 0..=8 {
        input.col_memo.push(col_taken(&input.board, idx));
        input.row_memo.push(row_taken(&input.board, idx));
        input
            .square_memo
            .push(square_taken(&input.board, idx / 3 * 3, idx % 3 * 3));
    }
}

fn get_empty_square(board: &[u32]) -> Option<(u32, u32)> {
    let square = board.iter().enumerate().find(|(_, item)| **item == 0)?;
    let index = first.0 as u32;
    Some((index / 9, index % 9))
}

pub fn solve(input: &mut Sudoku) -> bool {
    match find_first_empty(&input.board) {
        None => true,
        Some((row, col)) => {
            for option in 1..=9 {
                if 1 << option - 1 & options_for(&input, row, col) == 0 {
                    let idx = (row * 9 + col) as usize;
                    input.board[idx] = option;
                    remove_option(input, row, col, option);
                    let sol = solve_sudoku(input);
                    if sol {
                        return sol;
                    }
                    input.board[idx] = 0;
                    add_option(input, row, col, option);
                }
            }
            false
        }
    }
}

fn get_possible_vals(board: &Sudoku, row: u32, col: u32) -> u32 {
    board.square_memo[row_col_to_square(row, col) as usize] | board.row_memo[row as usize] | board.col_memo[col as usize]
}

fn row_col_to_square(row: u32, col: u32) -> u32 {
    row / 3 * 3 + col / 3
}

fn remove_option(board: &mut Sudoku, row: u32, col: u32, value: u32) {
    let mask = 1 << value - 1;
    board.row_memo[row as usize] |= mask;
    board.col_memo[col as usize] |= mask;
    board.square_memo[row_col_to_square(row, col) as usize] |= mask;
}

fn add_option(board: &mut Sudoku, row: u32, col: u32, value: u32) {
    let mask = !(1 << value - 1);
    board.row_memo[row as usize] &= mask;
    board.col_memo[col as usize] &= mask;
    board.square_memo[row_col_to_square(row, col) as usize] &= mask;
}