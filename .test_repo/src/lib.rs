/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */
pub struct Solution {}

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 使用滑动窗口
        if s == "" {
            return 0;
        }
        let mut start = 0;
        let mut end = 1;
        let mut res = 1; // 获取当前窗口长度
        while end != s.len() {
            // println!("{}  {}", &s[start..end], &s[end..end + 1]);
            if !&s[start..end].contains(&s[end..end + 1]) {
                end += 1; // 窗口向右扩大
                res = max(res, s[start..end].len())
            } else {
                // 窗口从左边缩小
                start += 1;
                if start >= end {
                    end = start + 1;
                }
            }
        }
        println!("{}", res);
        res as i32
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("aaa".to_string()), 1);

        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(
            Solution::length_of_longest_substring("anviaj".to_string()),
            5
        );
    }
}
