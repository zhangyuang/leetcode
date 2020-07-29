struct Solution {}

impl Solution {
  pub fn word_break(mut s: String, mut word_dict: Vec<String>) -> bool {
    // 参考题解 https://leetcode-cn.com/problems/word-break/solution/dan-ci-chai-fen-ju-jue-zhuang-xcong-jian-dan-de-xi/
    // 动态规划算法，dp[i]表示s前i个字符能否拆分
    // 转移方程：dp[j] = dp[i] && check(s[i+1, j]);
    let mut dp = vec![true; s.len() + 1];
    for i in 1..s.len() + 1 {
      for j in (0..i).rev() {
        dp[i] = dp[j] && word_dict.contains(&s[j..i].to_string());
        if dp[i] {
          break;
        }
      }
    }
    println!("{:?}", dp);
    dp[dp.len() - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::word_break(
      "leetcode".to_string(),
      vec!["leet".to_string(), "code".to_string()],
    );
  }
}
