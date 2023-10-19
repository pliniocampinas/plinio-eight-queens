#[derive(Copy, Clone)]
struct Move {
    line: usize,
    col: usize,
}

fn main() {
    println!("Eight Queens");

    let mut table: [u8; 64] = [0; 64];

    solve_table(&mut table);

    print_table(&table);
}

fn solve_table(table: & mut [u8; 64]) {
    let mut moves_stack: [Move; 8] = [Move { line: 0, col: 0 }; 8];
    let mut stack_index: usize = 0;

    let mut interation: u64 = 0;
    let max_iterations: u64 = 1_000_000;

    let mut last_invalid_path = Move {
        line: 0,
        col: 0
    };

    loop {
        let previous_stack_index = stack_index;
        for line in 1..9 {
            for col in 1..9 {
                if last_invalid_path.line == line && last_invalid_path.col == col {
                    continue;
                }

                if is_valid_move(&moves_stack, line, col) {
                    moves_stack[stack_index] = Move {
                        line: line,
                        col: col
                    };
                    stack_index+=1;
                }
            }
        }

        if stack_index >= 8 {
            break;
        }

        if previous_stack_index == stack_index {
            last_invalid_path.line = moves_stack[stack_index].line;
            last_invalid_path.col = moves_stack[stack_index].col;
            moves_stack[stack_index].line = 0;
            moves_stack[stack_index].col = 0;
            stack_index-=1;
        }

        interation+=1;
        if interation >= max_iterations {
            println!("Max iterations - breaking with stack size: {}", stack_index);
            break;
        }
    }


    for elem in moves_stack {
        if elem.line == 0 || elem.col == 0 {
            continue;
        }
        println!("setting move line: {} col: {}", elem.line, elem.col);
        set_table_value(table, elem.line, elem.col, 1);
    }
}

fn is_valid_move(moves_stack: &[Move; 8], line: usize, col: usize) -> bool {
    if line < 1 || line > 8 {
        panic!("Invalid line value");
    }
    if col < 1 || col > 8 {
        panic!("Invalid column value");
    }

    // An integer that represents diagonal lines
    let diagonal_to_right: i32 = line as i32 - col as i32;
    let diagonal_to_left: i32 = col as i32 + line as i32;

    for elem in moves_stack {
        if elem.line == 0 || elem.col == 0 {
            continue;
        }

        // Validate lines.
        if elem.line == line {
            return false;
        }
        if elem.line == col {
            return false;
        }
        // Validate columns.
        if elem.col == col {
            return false;
        }
        if elem.col == line {
            return false;
        }

        let previous_move_diagonal_to_right: i32 = elem.line as i32 - elem.col as i32;
        let previous_move_diagonal_to_left: i32 = elem.col as i32 + elem.line as i32;

        if previous_move_diagonal_to_right == diagonal_to_right {
            return false;
        }
        if previous_move_diagonal_to_left == diagonal_to_left {
            return false;
        }
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
        print!(" {} ", elem);
        print!("|");
    }
    println!();
    println!("----------- ---------- ------------ ");
}

#[cfg(test)]
#[test]
fn is_valid_move_works() {
    let [ line, col ] = [ 4 , 6 ];
    let moves_stack: [Move; 8] = [Move { line: 0, col: 0 }; 8];
    let is_valid = is_valid_move(&moves_stack, line, col);
    assert!(is_valid == true);
}

#[test]
#[should_panic]
fn is_valid_move_should_panic() {
    let [ line, col ] = [ 0 , 6 ];
    let moves_stack: [Move; 8] = [Move { line: 0, col: 0 }; 8];
    is_valid_move(&moves_stack, line, col);
}