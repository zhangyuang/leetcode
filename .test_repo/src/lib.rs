/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }
        let left = 0;
        let right = nums.len() - 1;
        return Self::binary_search(nums, target, left, right);
    }
    fn binary_search(nums: Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if right < left {
            return -1;
        }
        let pivot_index = (right + left) / 2;
        if nums[pivot_index] == target {
            return pivot_index as i32;
        }
        if nums[pivot_index] < target {
            return Self::binary_search(nums, target, pivot_index + 1, right);
        } else {
            if right == 0 || pivot_index == 0 {
                // 防止 usize 溢出
                return -1;
            }
            return Self::binary_search(nums, target, left, pivot_index - 1);
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let nums: Vec<i32> = vec![2, 5];
        println!("{:?}", Solution::search(nums, 0));
    }
}
