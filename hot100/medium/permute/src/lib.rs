struct Solution {}

impl Solution {
  pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![]; // 记录最终结果
    let mut path: Vec<i32> = vec![]; // 记录当前的path
    back_trace(&mut res, &mut path, &nums);
    res
  }
}

fn back_trace(res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, nums: &Vec<i32>) {
  if path.len() == nums.len() {
    res.push(path.clone()); // 所有的数字都使用了，递归结束
    return;
  }
  for i in 0..nums.len() {
    if path.contains(&nums[i]) {
      // 说明当前的path vector已经有该数字了，跳出该循环
      continue;
    }
    path.push(nums[i]); // 选择数字
    back_trace(res, path, nums);
    path.pop(); // 撤销选择
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::permute(vec![1, 2, 3]);
  }
}
