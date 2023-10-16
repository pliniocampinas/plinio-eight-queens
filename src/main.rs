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

    while stack_index < 8 {
        for line in 1..9 {
            for col in 1..9 {
                if is_valid_move(&moves_stack, line, col) {
                    moves_stack[stack_index] = Move {
                        line: line,
                        col: col
                    };
                    stack_index+=1;
                }
            }
        }
    }


    for elem in moves_stack {
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

    for elem in moves_stack {
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
        // TODO: Validate diagonal.
    }

    return true;
}

fn get_table_value(&table: &[u8; 64], line: usize, col: usize) -> u8 {
    let index = (line * col) - 1;
    return *table.get(index).unwrap();
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
fn set_and_get_table_works() {
    let mut table: [u8; 64] = [0; 64];
    let [ line, col ] = [ 4 , 6 ];
    set_table_value(&mut table, line, col, 1);
    let cell_value = get_table_value(&table, line, col);
    assert!(cell_value == 1);
}

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