struct Solution {}

impl Solution {
  pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word_vec: Vec<&str> = word.split("").collect(); // string转vector
    let start_char = word_vec[1]; // 记录开头的字母
    let mut path: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()]; // 记录当前路径有没有被访问
    for i in 0..board.len() {
      let board_item = board[i].clone();
      for j in 0..board_item.len() {
        if board[i][j].to_string() == start_char {
          if check(&word_vec, &board, i as isize, j as isize, 1, &mut path) {
            // 找到第一个字母开头的坐标，可能有多个第一个字母出现
            return true;
          }
          path = vec![vec![false; board[0].len()]; board.len()]; // 清空之前的path记录
        }
      }
    }
    return false;
  }
}
fn check(
  word_vec: &Vec<&str>,
  board: &Vec<Vec<char>>,
  start_row: isize,
  start_column: isize,
  index: usize, // 当前正在找第几个字母
  path: &mut Vec<Vec<bool>>,
) -> bool {
  // ["",A,B,C,C,E,D,""]
  if word_vec[index] == "" {
    // 已经匹配完所有的字母了
    return true;
  }
  if start_row < 0
    || start_column < 0
    || start_row as usize == board.len()
    || start_column as usize == board[0].len()
    || path[start_row as usize][start_column as usize]
  {
    // 越界或者当前路径已经访问的情况直接返回false
    return false;
  }
  if word_vec[index] != board[start_row as usize][start_column as usize].to_string() {
    return false;
  }
  path[start_row as usize][start_column as usize] = true;

  if check(
    &word_vec,
    board,
    start_row + 1,
    start_column,
    index + 1,
    &mut path.clone(),
  ) {
    // 往下面找
    if word_vec[index + 1] == "" {
      // 已经匹配完所有的字母了
      return true;
    }
    path[start_row as usize + 1][start_column as usize] = true;
    return true;
  }
  if check(
    // 往左边找
    &word_vec,
    board,
    start_row,
    start_column - 1,
    index + 1,
    &mut path.clone(),
  ) {
    if word_vec[index + 1] == "" {
      // 已经匹配完所有的字母了
      return true;
    }
    path[start_row as usize][start_column as usize - 1] = true;
    return true;
  }

  if check(
    // 往右边找
    &word_vec,
    board,
    start_row,
    start_column + 1,
    index + 1,
    &mut path.clone(),
  ) {
    if word_vec[index + 1] == "" {
      // 已经匹配完所有的字母了
      return true;
    }
    path[start_row as usize][start_column as usize + 1] = true;
    return true;
  }

  if check(
    // 往上找
    &word_vec,
    board,
    start_row - 1,
    start_column,
    index + 1,
    &mut path.clone(),
  ) {
    if word_vec[index + 1] == "" {
      // 已经匹配完所有的字母了
      return true;
    }
    // 往上面找
    path[start_row as usize - 1][start_column as usize] = true;
    return true;
  }
  return false;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    println!(
      "{:?}",
      Solution::exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'E', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "ABCESEEE".to_string(),
      )
    )
  }
}
