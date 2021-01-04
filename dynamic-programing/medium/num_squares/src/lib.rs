/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */
struct Solution {}
// @lc code=start
use std::cmp::min;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let nusize = n as usize;
        let mut dp = vec![0; nusize + 1];
        for i in 1..=nusize {
            dp[i] = i;
            let mut j = 1;
            while i >= j * j {
                dp[i] = min(dp[i], dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[nusize] as i32
    }
}
// @lc code=end
