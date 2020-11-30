/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */
struct Solution {}
// @lc code=start
use std::cmp::max;
use std::cmp::min;

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 || prices.len() == 1 {
      return 0;
    }
    let mut min_sign = prices[0]; // 记录当天之前的最低价格
    let mut dp = vec![0; prices.len()]; // dp 代表当天卖出能获得的最大收益
    let mut res = 0;
    dp[0] = 0;
    for i in 1..prices.len() {
      dp[i] = prices[i] - min_sign;
      min_sign = min(min_sign, prices[i]);
      res = max(res, dp[i])
    }
    res
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    println!("{:?}", Solution::max_profit(vec![1]))
  }
}
