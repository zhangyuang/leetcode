/*
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * [96] 不同的二叉搜索树
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // 题解https://leetcode-cn.com/problems/unique-binary-search-trees/solution/bu-tong-de-er-cha-sou-suo-shu-by-leetcode-solution/
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                let i = i as usize;
                let j = j as usize;
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        println!("{:?}", dp);
        dp[n as usize]
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::num_trees(3);
    }
}
