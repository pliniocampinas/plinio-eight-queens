mod queens_solver;
use queens_solver::{
    MovesStack,
    fill_table,
    solve_queens,
};

fn main() {
    println!("Eight Queens");

    let mut table: [u8; 64] = [0; 64];

    solve_and_fill_table(&mut table);

    print_table(&table);
}

fn solve_and_fill_table(table: & mut [u8; 64]) {
    let mut moves_stack: queens_solver::MovesStack = queens_solver::MovesStack::new();
    let mut results: Vec<MovesStack> = Vec::new();
    solve_queens(0, &mut moves_stack, &mut results);
    fill_table(table, &results[0]);

    println!("result_count {}", results.len());
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