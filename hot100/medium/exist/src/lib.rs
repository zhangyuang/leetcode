struct Solution {}

impl Solution {
  pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word_vec: Vec<&str> = word.split("").collect(); // string转vector
    let start_char = word_vec[1]; // 记录开头的字母
    let mut start_row = 0;
    let mut start_column = 0;
    'outer: for i in 0..board.len() {
      let board_item = board[i].clone();
      for j in 0..board_item.len() {
        if board[i][j].to_string() == start_char {
          start_row = i;
          start_column = j;
          break 'outer; // 跳出外层循环
        }
      }
    }

    println!(
      "{:?}",
      Solution::check(&word_vec, &board, start_row, start_column, 1)
    );
    true
    // Solution::check(&word_vec, &board, start_row, start_column, 1)
  }
  fn check(
    word_vec: &Vec<&str>,
    board: &Vec<Vec<char>>,
    mut start_row: usize,
    mut start_column: usize,
    index: usize,
  ) -> bool {
    if index == word_vec.len() {
      return false;
    }
    if start_column < 0 || start_row < 0 || start_row == board.len() {
      return false;
    }
    println!(
      "{:?}",
      format!("{}{}{}", word_vec[index], start_row, start_column)
    );
    return word_vec[index] == board[start_row][start_column].to_string()
      && (Solution::check(&word_vec, board, start_row + 1, start_column, index + 1)
        || Solution::check(&word_vec, board, start_row, start_column + 1, index + 1)
        || Solution::check(&word_vec, board, start_row - 1, start_column, index + 1));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::exist(
      vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
      ],
      "ABCCED".to_string(),
    );
  }
}
