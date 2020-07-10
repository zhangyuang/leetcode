struct Solution {}

// 60(2) = 3*4*5
// 40(3) = 2*4*5
// 30(4) = 2 * 3 * 5
// 24(5) = 2*3*4
impl Solution {
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut dp = vec![0; nums.len()];
    dp[0] = 1;
    for i in 1..nums.len() {
      // 左侧数字的乘积
      dp[i] = nums[i - 1] * dp[i - 1]
    }
    let mut j = nums.len() - 1;
    let mut R = 1;
    while j >= 0 {
      // 乘以右侧数字的乘积
      dp[j] = dp[j] * R;
      R *= nums[j];
      if j == 0 {
        // 这里break方式防止usize为负数导致panic
        break;
      }
      j -= 1;
    }
    dp
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    Solution::product_except_self(nums);
  }
}
