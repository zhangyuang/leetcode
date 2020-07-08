pub struct Solution {}

use std::cmp::max;

pub fn massage(nums: Vec<i32>) -> i32 {
  if nums.len() == 0 {
    return 0;
  }
  let mut res = nums[0];
  let len = nums.len();
  let mut dp: Vec<Vec<i32>> = vec![vec![0, nums[0]]; len]; // 设定length定义dp二维数组
  for i in 1..len {
    if dp.get(i).is_none() {
      dp[i] = vec![0, 0]
    }

    dp[i][0] = max(dp[i - 1][0], dp[i - 1][1]); // dp[i][0] 表示前i个预约第i个预约不接的最长时间
    dp[i][1] = dp[i - 1][0] + nums[i]; // dp[i][1] 表示前i个预约第i个预约接的最长时间
    res = max(max(res, dp[i][0]), dp[i][1])
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let nums: Vec<i32> = vec![2, 1, 4, 5, 3, 1, 1, 3];
    massage(nums);
  }
}
