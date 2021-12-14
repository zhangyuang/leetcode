/*
 * @lc app=leetcode.cn id=807 lang=rust
 *
 * [807] 保持城市天际线
 */
struct Solution {}

// @lc code=start
impl Solution {
  pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
      return 0;
    };
    let mut res = 0;
    let row_max = grid
      .iter()
      .map(|val| *val.iter().max().unwrap())
      .collect::<Vec<i32>>();
    let mut col_max: Vec<i32> = vec![-1; n];
    grid.iter().enumerate().for_each(|(i, row)| {
      row.iter().enumerate().for_each(|(j, val)| {
        let max = val.max(&col_max[j]);
        col_max[j] = *max;
      })
    });
    grid.iter().enumerate().for_each(|(i, row)| {
      row
        .iter()
        .enumerate()
        .for_each(|(j, _)| res += row_max[i].min(col_max[j]) - grid[i][j])
    });
    return res;
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let grid = vec![
      vec![3, 0, 8, 4],
      vec![2, 4, 5, 7],
      vec![9, 2, 6, 3],
      vec![0, 3, 1, 0],
    ];
    Solution::max_increase_keeping_skyline(grid);
  }
}
