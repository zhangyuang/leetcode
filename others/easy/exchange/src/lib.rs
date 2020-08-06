struct Solution {}

impl Solution {
  pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
      return nums;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left != right {
      if nums[left] % 2 != 0 {
        left += 1;
        continue;
      }
      if nums[right] % 2 == 0 {
        right -= 1;
        continue;
      }
      let temp = nums[right];
      nums[right] = nums[left];
      nums[left] = temp;
    }
    println!("{:?}", nums);
    nums
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::exchange(vec![1, 2, 3, 4]);
  }
}
