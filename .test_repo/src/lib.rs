/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![1; n]; m];
        // 特殊处理第一列的情况
        let mut hasObstacle = false;
        for i in 0..n {
            if obstacle_grid[0][i] == 1 {
                hasObstacle = true
            }
            if hasObstacle {
                dp[0][i] = 0
            }
        }
        hasObstacle = false;
        // 特殊处理第一行的情况
        for i in 0..m {
            if obstacle_grid[i][0] == 1 {
                hasObstacle = true
            }
            if hasObstacle {
                dp[i][0] = 0
            }
        }
        for i in 1..m {
            for j in 1..n {
                let top = if obstacle_grid[i - 1][j] == 1 {
                    0
                } else {
                    dp[i - 1][j]
                };
                let left = if obstacle_grid[i][j - 1] == 1 {
                    0
                } else {
                    dp[i][j - 1]
                };
                dp[i][j] = top + left;
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0
                }
            }
        }
        dp[m - 1][n - 1]
    }
}
// @lc code=end
