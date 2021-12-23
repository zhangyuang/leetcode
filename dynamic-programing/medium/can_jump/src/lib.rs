/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![false; n];
        dp[0] = true;
        for i in 1..nums.len() {
            for j in 0..i {
                dp[i] = if dp[j] && nums[j] >= (i - j) as i32 {
                    true
                } else {
                    false
                };
                if dp[i] {
                    break;
                }
            }
        }
        return dp[n - 1];
    }
}
// @lc code=end

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn name() {
        println!("{:?}", Solution::can_jump(vec![2, 3, 1, 1, 4]));
        println!("{:?}", Solution::can_jump(vec![3, 2, 1, 0, 4]))
    }
}
