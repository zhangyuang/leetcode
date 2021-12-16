/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */
struct Solution {}
// @lc code=start
impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right, mut res) = (0, height.len() - 1, 0);
    while left < right {
      res = res.max(height[left].min(height[right]) * (right - left) as i32);
      if height[left] < height[right] {
        left = left + 1;
      } else {
        right = right - 1;
      }
    }
    return res;
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    Solution::max_area(vec![1, 1]);
  }
}
