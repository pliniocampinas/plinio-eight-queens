fn main() {
    println!("Eight Queens");

    let table: [u8; 64] = [1; 64];

    let [ line, col ] = [ 4 , 6 ];
    let cell_value = get_table_value(&table, line, col);

    println!("Table line {}, col: {}", line, col);
    println!("Table {}", table[63]);
    assert!(cell_value == 1);
}

fn get_table_value(&table: &[u8; 64], line: usize, col: usize) -> u8 {
    let index = (line * col) - 1;
    return *table.get(index).unwrap();
}