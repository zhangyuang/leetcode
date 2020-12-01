/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            } else {
                return vec![-1, -1];
            }
        }
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let left = Self::binary_search_left(&nums, target, 0, nums.len() - 1);
        let right = Self::binary_search_right(&nums, target, 0, nums.len() - 1);
        vec![
            if left >= 0 && left < nums.len() as i32 && nums[left as usize] == target {
                left
            } else {
                -1
            },
            if right >= 0 && left < nums.len() as i32 && nums[right as usize] == target {
                right
            } else {
                -1
            },
        ]
    }

    fn binary_search_left(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if right < left {
            return left as i32;
        }
        let pivot_index = (right + left) / 2;
        if nums[pivot_index] == target {
            if pivot_index == 0 {
                return pivot_index as i32;
            }
            return Self::binary_search_left(nums, target, left, pivot_index - 1);
        }
        if nums[pivot_index] < target {
            return Self::binary_search_left(nums, target, pivot_index + 1, right);
        } else {
            if pivot_index == 0 {
                // 防止 usize 溢出
                return -1;
            }
            return Self::binary_search_left(nums, target, left, pivot_index - 1);
        }
    }
    fn binary_search_right(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if right < left {
            return right as i32;
        }
        let pivot_index = (right + left) / 2;
        if nums[pivot_index] == target {
            return Self::binary_search_right(nums, target, pivot_index + 1, right);
        }
        if nums[pivot_index] < target {
            return Self::binary_search_right(nums, target, pivot_index + 1, right);
        } else {
            if pivot_index == 0 {
                if nums[pivot_index] as i32 == target {
                    return pivot_index as i32;
                } else {
                    return -1;
                }
            }
            return Self::binary_search_right(nums, target, left, pivot_index - 1);
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let nums: Vec<i32> = vec![2, 2];
        println!("{:?}", Solution::search_range(nums, 2));
    }
}
