/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */
struct Solution {}
// @lc code=start

impl Solution {
  pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    let mut dp = vec![];
    let mut start = 0;
    let mut end = nums.len() - 1;
    while start <= end {
      let mut val;
      if nums[start].abs() >= nums[end].abs() {
        val = nums[start];
        start += 1;
      } else {
        val = nums[end];
        end -= 1;
      };
      dp.push(val * val)
    }
    dp.reverse();
    dp
  }
}
// @lc code=end
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    println!("{:?}", Solution::sorted_squares(vec![1]))
  }
}
