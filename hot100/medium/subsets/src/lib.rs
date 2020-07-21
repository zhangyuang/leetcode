struct Solution {}

impl Solution {
  pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 本质上类似于子集的全排列
    let mut res: Vec<Vec<i32>> = vec![];
    Self::get_subsets(&nums, &mut res, &mut vec![], 0);
    println!("{:?}", res);
    res
  }
  fn get_subsets(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, start: usize) {
    res.push(path.clone());
    for i in start..nums.len() {
      path.push(nums[i]);
      let start = i + 1;
      Self::get_subsets(nums, res, path, start);
      path.pop();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::subsets(vec![1, 2, 3]);
  }
}
