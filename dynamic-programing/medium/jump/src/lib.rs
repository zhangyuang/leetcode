/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![i32::MAX; n]; // 跳到 i 的最小步数
        dp[0] = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] >= (i - j) as i32 {
                    // dp[i] = true;
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }
        dp[n - 1]
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!("{:?}", Solution::jump(vec![1, 2, 3, 4]));
    }
}
