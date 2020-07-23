struct Solution {}

impl Solution {
  pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    // O(n)的解法一般都需要用到原地标记
    let mut i = 0;
    let mut res = vec![];
    while i < nums.len() {
      let key = (nums[i].abs() - 1) as usize;
      if nums[key] > 0 {
        nums[key] = nums[key] * -1;
      }
      i += 1;
    }
    let mut i = 0;
    while i < nums.len() {
      if nums[i] > 0 {
        res.push((i + 1) as i32);
      }
      i += 1;
    }
    println!("{:?}", res);
    res
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]);
  }
}
