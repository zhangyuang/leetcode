/*
 * @lc app=leetcode.cn id=475 lang=rust
 *
 * [475] 供暖器
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        heaters.sort_by(|a, b| a.partial_cmp(b).unwrap());
        houses.iter().for_each(|h| {
            let distance = (Solution::binary_search_left(&heaters, h) - h)
                .abs()
                .min((Solution::binary_search_right(&heaters, h) - h).abs());
            res = res.max(distance);
        });
        res
    }
    pub fn binary_search_left(nums: &Vec<i32>, target: &i32) -> i32 {
        // 找到距离 target 最近的左边距离
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if &nums[mid] >= target {
                right = mid
            } else {
                left = mid + 1
            }
        }
        nums[left]
    }
    pub fn binary_search_right(nums: &Vec<i32>, target: &i32) -> i32 {
        // 找到距离 target 最近的右边距离
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left + 1) / 2;
            if &nums[mid] <= target {
                left = mid
            } else {
                right = mid - 1
            }
        }
        nums[left]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name() {
        println!("{:?}", Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]))
        // println!("{:?}", Solution::binary_search(&vec![1, 2, 3], &2));
    }
}
