struct Solution {}

use std::cmp::max;

impl Solution {
  pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    if nums.len() == 1 {
      return res;
    }
    let mut i = 1;
    while i < nums.len() {
      // 动态规划，nums[i]代表的含义为到当前第i个元素的最大子序列和
      nums[i] = max(nums[i], nums[i] + nums[i - 1]);
      res = max(res, nums[i]);
      i += 1;
    }
    res
  }
}
