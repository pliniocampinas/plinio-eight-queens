#[derive(Copy, Clone)]
pub struct MovesStack {
  pub columns_positions: [isize; 8],
  pub moves_count: usize,
}

impl MovesStack {
  pub fn new() -> MovesStack {
    MovesStack {
      columns_positions: [0; 8],
      moves_count: 0,
    }
  }

  pub fn push(&mut self, value: isize) {
    self.columns_positions[self.moves_count] = value;
    self.moves_count += 1;
  }

  pub fn pop(&mut self) {
    self.columns_positions[self.moves_count - 1] = 0;
    self.moves_count -= 1;
  }
}

pub fn fill_table(table: &mut [u8; 64], moves_stack: &MovesStack) {
  let first_solution = moves_stack.columns_positions;
  let mut line_index = 0;
  for col_index in first_solution {
    set_table_value(table, line_index + 1, (col_index + 1) as usize, 1);
    line_index += 1;
  }
}

pub fn solve_queens(row: isize, moves_stack: &mut MovesStack, results: &mut Vec<MovesStack>, max_results: usize) {
  if results.len() == max_results {
    return;
  }

  if row == 8 {
    results.push(*moves_stack);
    return;
  }

  let mut col = 0;
  while col < 8 {
    moves_stack.push(col);
    if is_valid_move(moves_stack) {
      solve_queens(row + 1, moves_stack, results, max_results);
    }
    moves_stack.pop();

    col += 1;
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
    i += 1;
  }

  return true;
}

fn set_table_value(table: &mut [u8; 64], line: usize, col: usize, value: u8) {
  let index = ((line - 1) * 8) + col - 1;
  table[index] = value;
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
