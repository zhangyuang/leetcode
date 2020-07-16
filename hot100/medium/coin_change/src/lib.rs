struct Solution {}
use std::cmp::min;

impl Solution {
  // 完全背包问题的变种
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // 本题可以看liweiwei的分析，写的特别清晰 https://leetcode-cn.com/problems/coin-change/solution/dong-tai-gui-hua-shi-yong-wan-quan-bei-bao-wen-ti-/
    // dp[amout]的含义为凑成amount金额所需要的最小硬币数量
    let amount = amount as usize;
    let mut dp = vec![amount + 1; amount + 1];
    dp[0] = 0;
    for i in 1..amount + 1 {
      for j in 0..coins.len() {
        if coins[j] as usize <= i {
          dp[i] = min(dp[i], dp[i - coins[j] as usize] + 1);
        }
      }
    }

    if dp[amount] == amount + 1 {
      -1
    } else {
      dp[amount] as i32
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let coins: Vec<i32> = vec![1, 2, 6];
    Solution::coin_change(coins, 11);
  }
}
