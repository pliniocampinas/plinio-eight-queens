fn main() {
    println!("Eight Queens");

    let mut table: [u8; 64] = [0; 64];

    let [ line, col ] = [ 4 , 6 ];
    set_table_value(&mut table, line, col, 1);
    let cell_value = get_table_value(&table, line, col);

    println!("Table line {}, col: {}", line, col);
    print_table(&table);
    assert!(cell_value == 1);
}

fn get_table_value(&table: &[u8; 64], line: usize, col: usize) -> u8 {
    let index = (line * col) - 1;
    return *table.get(index).unwrap();
}

fn set_table_value(table: & mut [u8; 64], line: usize, col: usize, value: u8) {
    let index = (line * col) - 1;
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