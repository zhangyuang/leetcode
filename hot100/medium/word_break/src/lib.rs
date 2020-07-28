struct Solution {}

impl Solution {
  pub fn word_break(mut s: String, mut word_dict: Vec<String>) -> bool {
    word_dict.sort_by(|a, b| a.len().cmp(&b.len()));
    // println!("{:?}", word_dict);

    let mut count = 0;
    while count < 50 {
      for i in &word_dict {
        if s.contains(i) {
          s = s.replacen(i, "", 1);
        }

        if s == "" {
          break;
        }
      }
      count += 1;
    }
    println!("{}", s == "");
    s == ""
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::word_break(
      "aaaaaaaa".to_string(),
      vec!["aaaa".to_string(), "aaa".to_string(), "aa".to_string()],
    );
  }
}
