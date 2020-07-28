struct Solution {}
use std::cmp::max;

// 输入: [1,3,5,4,7]
// 输出: 3
// 解释: 最长连续递增序列是 [1,3,5], 长度为3。
// 尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为5和7在原数组里被4隔开。

impl Solution {
  pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
      return nums.len() as i32;
    }
    let mut res = 1;
    let mut path = vec![1; nums.len()];
    for i in 1..nums.len() {
      if nums[i] > nums[i - 1] {
        path[i] = path[i - 1] + 1;
        res = max(res, path[i]);
      }
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]);
  }
}
