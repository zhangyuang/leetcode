/*
 * @lc app=leetcode.cn id=415 lang=rust
 *
 * [415] 字符串相加
 */
struct Solution {}
// @lc code=start
use std::cmp::min;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut max_str = if num1.len() > num2.len() {
            "num1"
        } else {
            "num2"
        };
        let min_len = min(num1.len(), num2.len());
        let mut res = String::from("");
        let mut sign = false;
        for i in (0..min_len).rev() {
            let num1_curr = &num1[i..i + 1];
            let num2_curr = &num2[i..i + 1];
            let mut foo = num1_curr.parse::<i32>().unwrap() + num2_curr.parse::<i32>().unwrap();
            if sign == true {
                foo += 1;
            }
            if foo >= 10 {
                sign = true;
            } else {
                sign = false;
            }
            res.push_str(if foo > 10 {
                return foo.to_string()[1..2].to_string();
            } else {
                return foo.to_string();
            });
        }
        println!("{}", res);
        res.to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::add_strings("15".to_string(), "26".to_string());
    }
}
