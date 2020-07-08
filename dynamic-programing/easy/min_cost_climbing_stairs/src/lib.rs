pub struct Solution {}
use std::cmp::min;

impl Solution {
  pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
    cost.push(0); // 爬楼梯的最后一步当成是无消耗的，需要额外增加一个0元素
    let mut dp = vec![cost[0]; cost.len()];
    dp[1] = cost[1]; // 跳到第一个和第二个阶梯的答案就是cost对应的位置本身
    for i in 2..cost.len() {
      dp[i] = min(dp[i - 1] + cost[i], dp[i - 2] + cost[i]); // 第i层有两种可能分别是从前1个跳过来，或者前2个调过来
    }
    dp.pop().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let cost: Vec<i32> = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    Solution::min_cost_climbing_stairs(cost);
  }
}
