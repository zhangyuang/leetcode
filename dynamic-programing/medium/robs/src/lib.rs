/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */
struct Solution {}
// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        // 思路： 只需单独处理第一个和最后一个房屋，中间的房屋仍然和打家劫舍1一样处理
        return max(
            Self::rob1(nums[0..=nums.len() - 2].to_vec()), // 抢第一个，则不能抢最后一个
            Self::rob1(nums[1..=nums.len() - 1].to_vec()), // 不抢第一个，则可以抢最后一个
        );
    }
    fn rob1(nums: Vec<i32>) -> i32 {
        // 处理中间的房子，相当于直线排列的房子
        if nums.len() == 1 {
            return nums[0];
        }
        // dp 定义为最后一个房间可获得的最大金额
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = max(dp[i - 2] + nums[i], dp[i - 1])
        }
        return dp[nums.len() - 1];
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!("{:?}", Solution::rob(vec![1, 2]))
    }
}
