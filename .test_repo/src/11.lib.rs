/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut dp = vec![0; n]; // dp 表示以 n 为左半轴盛水的最大体积
        let mut res = 0;
        height.iter().enumerate().for_each(|(i, _)| {
            for j in i + 1..n {
                dp[i] = dp[i].max(height[j].min(height[i]) * (j - i) as i32);
                res = res.max(dp[i]);
            }
        });
        return res;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        Solution::max_area(vec![1, 1]);
    }
}
