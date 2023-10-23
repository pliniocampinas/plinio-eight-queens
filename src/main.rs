#[derive(Copy, Clone)]
struct MovesStack {
    columns_positions: [isize; 8],
    moves_count: usize,
}

impl MovesStack {
    fn new() -> MovesStack {
        MovesStack {
            columns_positions: [0;8],
            moves_count: 0
        }
    }

    fn push(&mut self, value: isize) {
        self.columns_positions[self.moves_count] = value;
        self.moves_count+=1;
    }

    fn pop(&mut self) {
        self.columns_positions[self.moves_count - 1] = 0;
        self.moves_count-=1;
    }
}

fn main() {
    println!("Eight Queens");

    let mut table: [u8; 64] = [0; 64];

    solve_and_fill_table(&mut table);

    print_table(&table);
}

fn solve_and_fill_table(table: & mut [u8; 64]) {
    let mut moves_stack: MovesStack = MovesStack::new();
    let mut results = [MovesStack::new(); 1];
    let mut result_count = 0;
    solve_queens(0, &mut moves_stack, &mut results, &mut result_count);

    println!("result_count {}", result_count);

    let first_solution = results[0].columns_positions;
    let mut line_index = 0;
    for col_index in first_solution {
        set_table_value(table, line_index + 1, (col_index+1) as usize, 1);
        line_index+=1;
    }
}

fn solve_queens(row: isize, moves_stack: &mut MovesStack, results: & mut [MovesStack; 1], solutions_count: &mut usize) {
    if row == 8 {
        results[0] = *moves_stack;
        *solutions_count+=1;
        return;
    }
    let mut col = 0;
    while col < 8 {
        moves_stack.push(col);
        if is_valid_move(moves_stack) {
            solve_queens(row+1, moves_stack, results, solutions_count);
        }
        moves_stack.pop();

        col+=1;
    }
}

fn is_valid_move(moves_stack: &MovesStack) -> bool {
    let row = moves_stack.moves_count - 1;

    let mut i: usize = 0;
    while i < row {
        let diff = (moves_stack.columns_positions[i] - moves_stack.columns_positions[row]).abs();
        if diff == 0 || diff == (row - i) as isize {
            return false;
        }
        i+=1;
    }

    return true;
}

fn set_table_value(table: & mut [u8; 64], line: usize, col: usize, value: u8) {
    let index = ((line - 1) * 8) + col - 1;
    table[index] = value;
}

fn print_table(&table: &[u8; 64]) {
    println!("-------------- Table -------------- ");
    print!("x ");
    for i in 1..9 {
        print!("| {} ", i);
    }
    println!("|");
    print!("-------------- ----- -------------- ");

    let mut index = 0;
    for elem in table {
        if index % 8 == 0 {
            println!();
            print!("{} |", (index/8 + 1));
        }
        index+=1;
        print!(" {} ", if elem > 0 { "Q" } else { "." });
        print!("|");
    }
    println!();
    println!("----------- ---------- ------------ ");
}

#[cfg(test)]
#[test]
fn is_valid_move_works() {
    let mut moves_stack: MovesStack = MovesStack::new();
    moves_stack.push(1);
    let is_valid = is_valid_move(&moves_stack);
    assert!(is_valid == true);
}

#[test]
fn is_valid_move_with_same_column_should_be_invalid() {
    let mut moves_stack: MovesStack = MovesStack::new();
    moves_stack.push(1);
    moves_stack.push(1);

    let is_valid = is_valid_move(&moves_stack);
    assert!(is_valid == false);
}

#[test]
fn is_valid_move_with_same_diagonal_should_be_invalid() {
    let mut moves_stack: MovesStack = MovesStack::new();
    moves_stack.push(2);
    moves_stack.push(3);

    let is_valid = is_valid_move(&moves_stack);
    assert!(is_valid == false);
}