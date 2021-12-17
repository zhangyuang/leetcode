/*
 * @lc app=leetcode.cn id=821 lang=rust
 *
 * [821] 字符的最短距离
 */
struct Solution {}

// @lc code=start
use std::i32::MAX;
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut arr = s.split("").collect::<Vec<&str>>();
        arr = arr[1..arr.len() - 1].to_vec();
        let mut ans = vec![0; arr.len()];
        let mut pre: i32 = MAX;
        for (i, val) in arr.iter().enumerate() {
            if *val == c.to_string() {
                pre = i as i32
            }
            ans[i] = (i as i32 - pre).abs()
        }
        let mut pre: i32 = 99999;
        for (i, _) in arr.iter().enumerate().rev() {
            if arr[i] == c.to_string() {
                pre = i as i32
            }
            ans[i] = ans[i].min((i as i32 - pre).abs())
        }
        return ans;
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
            Solution::shortest_to_char(String::from("aaab"), 'b')
        );
        println!(
            "{:?}",
            Solution::shortest_to_char(String::from("loveleetcode"), 'e')
        );
    }
}
