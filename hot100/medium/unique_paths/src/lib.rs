/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

struct Solution {}
// @lc code=start

impl Solution {
  pub fn unique_paths(m: i32, n: i32) -> i32 {
    // let m = m as usize;
    // let n = n as usize;
    // let mut dp = vec![vec![1; n]; m]; // dp[i][j]的含义为i*j的网格有多少条路径
    // for i in 1..m {
    //     for j in 1..n {
    //         dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
    //     }
    // }
    // dp[m - 1][n - 1]
    // 2压缩数组后
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![1; n]; // dp[i][j]的含义为i*j的网格有多少条路径
    for i in 1..m {
      for j in 1..n {
        dp[j] = dp[j] + dp[j - 1]
      }
    }
    dp[n - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    println!("{:?}", Solution::unique_paths(3, 2))
  }
}

// @lc code=end
