/*
 * @lc app=leetcode.cn id=373 lang=rust
 *
 * [373] 查找和最小的K对数字
 */
struct Solution {}
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut heap = BinaryHeap::new();
        for i in 0..nums1.len() {
            heap.push((Reverse(nums1[i] + nums2[0]), i, 0))
        }
        while let Some((_, i, j)) = heap.pop() {
            if k <= 0 {
                break;
            }
            res.push(vec![nums1[i], nums2[j]]);
            if j + 1 < nums2.len() {
                heap.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
            }
            k -= 1;
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 8);
    }
}
