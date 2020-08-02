/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */

struct Solution {}
// @lc code=start
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![1; grid[0].len()]; m]; // dp代表到m*n坐标时候的最小路径和
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = grid[i][0] + dp[i - 1][0]
        }
        for j in 1..n {
            dp[0][j] = grid[0][j] + dp[0][j - 1]
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j]
            }
        }
        println!("{:?}", dp);
        dp[m - 1][n - 1]
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::min_path_sum(vec![vec![1, 2, 5], vec![3, 2, 1]]);
    }
}
