/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */
struct Solution {}
// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 1;
        }
        if nums.len() == 1 {
            if nums[0] != 1 {
                return 1;
            } else if nums[0] == 1 {
                return 2;
            };
        }
        let mut v: Vec<i32> = vec![-1; nums.len()];
        let mut MAX = 0;
        for i in nums {
            if i > 0 && i < v.len() as i32 {
                v[i as usize] = 1;
            }
            MAX = max(MAX, i)
        }
        for i in 1..v.len() {
            if v[i] < 0 {
                return i as i32;
            }
        }
        if v.len() >= MAX as usize {
            return MAX + 1;
        } else {
            return v.len() as i32;
        }
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let nums: Vec<i32> = vec![2, 1];
        println!("{:?}", Solution::first_missing_positive(nums));
    }
}
