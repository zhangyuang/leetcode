struct Solution {}

impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![]; // 存放结果
    let mut path = String::from(""); // 存放当前路径, 第一个路径是固定的
    back_trace(&mut res, &mut path, 0, 0, n);
    println!("{:?}", res);
    res
  }
}

fn back_trace(res: &mut Vec<String>, path: &mut String, left: i32, right: i32, n: i32) {
  if path.len() == 2 * n as usize {
    // 递归结束
    res.push(path.clone());
    return;
  }
  if left < n {
    let mut path = path.clone();
    path.push_str("(");
    back_trace(res, &mut path, left + 1, right, n)
  }
  if left > 0 && left > right && right < n {
    // 剪枝（左括号可以使用的个数严格大于右括号可以使用的个数，才剪枝
    let mut path = path.clone();
    path.push_str(")");
    back_trace(res, &mut path, left, right + 1, n)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::generate_parenthesis(3);
  }
}
