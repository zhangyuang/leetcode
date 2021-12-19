/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut v = vec![vec![1]; num_rows as usize];
        for i in 2..=num_rows {
            let mut current_val = vec![1; i as usize];
            let pre_val = &v[(i - 2) as usize];
            for j in 1..=(current_val.len() - 2) {
                current_val[j as usize] = pre_val[j] + pre_val[j - 1]
            }
            current_val[(i-1) as usize] =1;
            v[(i-1) as usize]=current_val;
        }
        v
    }
}
// @lc code=end

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn tests() {
        println!("{:?}", Solution::generate(5));
    }
}
