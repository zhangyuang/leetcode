// 一维数组
/*
 * @lc app=leetcode.cn id=413 lang=rust
 *
 * [413] 等差数列划分
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![false; n];
        let mut res = 0;
        dp[0] = true;
        for i in 0..n {
            for j in (i + 1)..n {
                if j - i <= 1 {
                    dp[j] = true;
                    continue;
                }
                dp[j] = if dp[j - 1] && nums[j] - nums[j - 1] == nums[j - 1] - nums[j - 2] {
                    true
                } else {
                    false
                };
                if dp[j] && j >= 2 {
                    res = res + 1;
                }
            }
        }
        res
    }
}
// @lc code=end

// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!(
            "{:?}",
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4])
        );
        println!("{:?}", Solution::number_of_arithmetic_slices(vec![1]))
    }
}
