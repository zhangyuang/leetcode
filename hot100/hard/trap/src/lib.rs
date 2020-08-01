/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

struct Solution {}
// @lc code=start
use std::cmp::min;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        if height.len() == 1 {
            return 0;
        }
        let mut left_max = vec![0; height.len()]; // 记录左边的最大高度
        let mut right_max = vec![0; height.len()]; // 记录右边的最大高度
        let mut temp_left_max = 0;
        let mut temp_right_max = 0;
        for i in 0..height.len() {
            left_max[i] = temp_left_max;
            if height[i] > temp_left_max {
                temp_left_max = height[i]
            }
        }
        for i in (0..height.len()).rev() {
            right_max[i] = temp_right_max;
            if height[i] > temp_right_max {
                temp_right_max = height[i]
            }
        }
        println!("{:?}", left_max);
        println!("{:?}", right_max);
        let mut res = 0;
        for i in 1..height.len() - 1 {
            // 当前能够接住的雨水高度为左边的最大高度减去右边的最大高度的绝对值再减去当前的高度
            let min_height = min(right_max[i], left_max[i]);
            if min_height - height[i] > 0 {
                res += min_height - height[i];
            }
        }
        println!("{:?}", res);

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let nums: Vec<i32> = vec![0, 2, 0];
        Solution::trap(nums);
    }
}
