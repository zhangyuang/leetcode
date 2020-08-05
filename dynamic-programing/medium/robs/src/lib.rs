/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */

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
        if nums.len() == 2 {
            return max(nums[0], nums[1]);
        }
        let len = nums.len();
        let mut res1 = nums[0]; // 抢第一个的情况，最后一个无法抢，终止条件到n-1
        let mut dp1: Vec<Vec<i32>> = vec![vec![0, nums[0]]; len]; // 设定length定义dp二维数组
        dp1[1] = vec![nums[0], nums[0]];
        for i in 2..len - 1 {
            if dp1.get(i).is_none() {
                dp1[i] = vec![0, 0]
            }
            dp1[i][0] = max(dp1[i - 1][0], dp1[i - 1][1]); // dp1[i][0] 第i个不抢的最长时间
            dp1[i][1] = dp1[i - 1][0] + nums[i]; // dp1[i][1] 表示第i个抢的最长时间
            res1 = max(max(res1, dp1[i][0]), dp1[i][1])
        }
        let mut res2 = 0; // 不抢第一个，则可以抢最后一个
        let mut dp2: Vec<Vec<i32>> = vec![vec![0, 0]; len]; // 设定length定义dp二维数组
        dp2[1] = vec![0, nums[1]];
        for i in 2..len {
            if dp2.get(i).is_none() {
                dp2[i] = vec![0, 0]
            }
            dp2[i][0] = max(dp2[i - 1][0], dp2[i - 1][1]); // dp2[i][0] 第i个不抢的最长时间
            println!("{}{}", dp2[i - 1][0], dp2[i - 1][1]);

            dp2[i][1] = dp2[i - 1][0] + nums[i]; // dp2[i][1] 表示第i个抢的最长时间
            res2 = max(max(res2, dp2[i][0]), dp2[i][1])
        }
        max(res1, res2)
    }
}
// @lc code=end
