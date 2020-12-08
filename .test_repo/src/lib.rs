/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 * [392] 判断子序列
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut _t = t.clone();
        for i in s.chars() {
            let index = _t.find(i);
            if index.is_some() {
                let index_val = index.unwrap();
                let len = _t.chars().count();
                _t = _t[index_val + 1..len].to_string();
            } else {
                return false;
            }
        }
        true
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!(
            "{:?}",
            Solution::is_subsequence("aaaaa".to_string(), "baaa".to_string())
        )
    }
}
