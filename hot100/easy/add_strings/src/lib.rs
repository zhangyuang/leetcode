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
        let min_len = min(num1.len(), num2.len());
        let mut res = String::from("");
        let mut sign = false;
        for i in 1..=min_len {
            // 以短的字符串为标准。从后向前进行一位数字的加法
            let num1_curr = &num1[num1.len() - i..num1.len() - i + 1];
            let num2_curr = &num2[num2.len() - i..num2.len() - i + 1];
            let mut foo = num1_curr.parse::<i32>().unwrap() + num2_curr.parse::<i32>().unwrap();
            if sign == true {
                // 说明上次相加产生了进位
                foo += 1;
            }
            if foo >= 10 {
                // 说明相加之后的结果为两位数只需取后一位即可
                sign = true;
                res.push_str(&foo.to_string()[1..2])
            } else {
                sign = false;
                res.push_str(&foo.to_string())
            }
        }
        let mut foo = String::from("");
        res = res.chars().rev().collect::<String>(); // 反转之前得到的结果
        if num1.len() != num2.len() {
            // 当两个字符串数字不想等时，将长的那位剩下的字符串拼接在最前面即可
            if num1.len() > num2.len() {
                foo = num1[0..num1.len() - min_len].to_string();
            } else {
                foo = num2[0..num2.len() - min_len].to_string();
            }
            if sign == true {
                // 说明之前的最后一次相加产生了进位。需要把剩下的字符串与1进行想家
                let mut bar = Self::add_strings(foo, "1".to_string());
                bar.push_str(res.as_str());
                return bar;
            } else {
                foo.push_str(res.as_str());
                return foo;
            }
        } else {
            // 如果两个字符串位数想等且之前的最后一次运算产生了进位，则需要在最前面补上1
            if sign == true {
                let mut foo = String::from("1");
                foo.push_str(res.as_str());
                return foo;
            } else {
                return res;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::add_strings(
            "965851889636410748708524976419405193491".to_string(),
            "76217403373357744506668".to_string(),
        );
    }
}
