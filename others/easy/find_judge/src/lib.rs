/*
 * @lc app=leetcode.cn id=997 lang=rust
 *
 * [997] 找到小镇的法官
 */
struct Solution {}

// @lc code=start
impl Solution {
  pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut in_degree: Vec<i32> = vec![0; n as usize];
    let mut out_degree: Vec<i32> = vec![0; n as usize];
    trust.iter().for_each(|v| {
      in_degree[(v[1] - 1) as usize] = in_degree[(v[1] - 1) as usize] + 1;
      out_degree[(v[0] - 1) as usize] = out_degree[(v[0] - 1) as usize] + 1;
    });
    let mut res: i32 = -1;

    in_degree.iter().enumerate().for_each(|(i, val)| {
      if *val == n - 1 && out_degree[i] == 0 {
        res = (i + 1) as i32;
      }
    });

    return res;
  }
}
// @lc code=end

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn tests() {
    println!(
      "{}",
      Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]])
    )
  }
}
