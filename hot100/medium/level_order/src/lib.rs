struct Solution {}

impl Solution {
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut dp = vec![0; nums.len()];
    let mut start = 1;
    for i in 1..nums.len() {
      start *= nums[i]
    }
    dp[0] = start;
    for i in 1..nums.len() {
      if nums[i] == 0 {
        dp[i] = 1;
        for j in 0..i {
          dp[i] *= nums[j]
        }
      } else {
        dp[i] = start / nums[i];
      }
    }
    dp
  }
}
