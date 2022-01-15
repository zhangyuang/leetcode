/*
 * @lc app=leetcode.cn id=334 lang=rust
 *
 * [334] 递增的三元子序列
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut left_min = vec![i32::MAX; nums.len()];
        let mut right_max = vec![i32::MIN; nums.len()];
        let mut foo = nums[0];
        for i in 1..nums.len() {
            left_min[i] = if foo < nums[i] {
                foo
            } else {
                foo = nums[i];
                nums[i]
            }
        }
        let mut foo = nums[nums.len() - 1];
        for i in (1..nums.len()).rev() {
            right_max[i] = if foo > nums[i] {
                foo
            } else {
                foo = nums[i];
                nums[i]
            }
        }
        for i in 1..nums.len() {
            if nums[i] > left_min[i] && nums[i] < right_max[i] {
                return true;
            }
        }
        false
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::increasing_triplet(vec![1, 3, 5, 4, 7]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false)
    }
}
