struct Solution {}
use std::cmp::max;

impl Solution {
  // 完全背包问题的变种
  pub fn coin_change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![0; amount + 1];
    dp[0] = 0;
    for i in 1..coins.len() + 1 {
      let weight = coins[i - 1];
      let val = coins[i - 1] as usize;
      for j in val..amount + 1 {
        dp[j] = max(dp[j], dp[j - weight as usize]);
      }
    }
    println!("{:?}", dp);
    1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let coins: Vec<i32> = vec![1, 2, 5];
    Solution::coin_change(5, coins);
  }
}
