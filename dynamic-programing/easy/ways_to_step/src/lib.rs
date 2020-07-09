pub struct Solution {}
use std::cmp::max;

impl Solution {
  pub fn ways_to_step(n: i32) -> i32 {
    let len = max(n, 3);
    let mut dp = vec![0; len as usize];
    dp[0] = 1;
    dp[1] = 2;
    dp[2] = 4;
    let mut i: usize = 3;
    while i < n as usize {
      for j in 1..4 {
        dp[i] += (dp[i - j]) % 1000000007;
        dp[i] = dp[i] % 1000000007;
      }
      i += 1;
    }
    let res: i32 = dp[(n - 1) as usize];
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::ways_to_step(61);
  }
}
