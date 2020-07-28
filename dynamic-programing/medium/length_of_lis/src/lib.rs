struct Solution {}
use std::cmp::max;

// 只要 nums[i] 严格大于在它位置之前的某个数，那么 nums[i] 就可以接在这个数后面形成一个更长的上升子序列；
// 因此，dp[i] 就等于下标 i 之前严格小于 nums[i] 的状态值的最大者 +1。
// 语言描述：在下标 i 之前严格小于 nums[i] 的所有状态值中的最大者 +1。

// 符号描述：

// dp[i] = \max_{0 \le j < i, nums[j] < nums[i]} {dp[j] + 1}
// dp[i]=
// 0≤j<i,nums[j]<nums[i]
// max
// ​

impl Solution {
  pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
      return nums.len() as i32;
    }
    let mut dp = vec![1; nums.len()];
    // dp[i]代表到i为止最长的递增子序列长度
    for i in 1..nums.len() {
      for j in 0..i {
        if nums[i] > nums[j] {
          dp[i] = max(dp[j] + 1, dp[i]);
        }
      }
    }
    let mut res = dp[0];
    for i in dp {
      res = max(res, i);
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]);
  }
}
