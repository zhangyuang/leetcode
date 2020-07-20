struct Solution {}

impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut v = vec![];
    let mut i = 0;
    while i < n {}
    vec!["1".to_string()]
  }
}

fn check(v: &str) -> bool {
  // 判断当前字符串是否符合要求
  let mut stack = vec![];
  let v = v.split("");
  for i in v {
    if i == "{" {
      stack.push(i);
    } else if i == "}" {
      stack.pop();
    }
  }
  if stack.len() == 0 {
    true
  } else {
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let res = check("{{}");
    println!("{}", res);
  }
}
