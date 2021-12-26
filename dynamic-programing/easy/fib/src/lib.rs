/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 * [509] 斐波那契数
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0, 1, 1];
        if n <= 2 {
            return dp[n];
        }

        for i in 2..=n {
            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = dp[0] + dp[1];
        }
        dp[1]
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!("{:?}", Solution::fib(6))
    }
}
