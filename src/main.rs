fn main() {
    println!("Eight Queens");

    let table: [u8; 64] = [0; 64];

    let [ line, col ] = [ 4 , 6 ];
    let cell_value = get_table_value(&table, line, col);

    println!("Table line {}, col: {}", line, col);
    print_table(&table);
    assert!(cell_value == 0);
}

fn get_table_value(&table: &[u8; 64], line: usize, col: usize) -> u8 {
    let index = (line * col) - 1;
    return *table.get(index).unwrap();
}

fn print_table(&table: &[u8; 64]) {
    println!("-------------- Table ------------- ");
    let mut index = 0;
    for elem in table {
        if index % 8 == 0 {
            println!();
            print!("|");
        }
        index+=1;
        print!(" {} ", elem);
        print!("|");
    }
    println!();
    println!("----------- ---------- ---------- ");
}