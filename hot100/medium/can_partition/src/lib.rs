struct Solution {}
use std::cmp::max;

impl Solution {
  // 01背包变种
  pub fn can_partition(nums: Vec<i32>) -> bool {
    let mut len = nums.len();
    let mut res = false;
    while len >= 2 {
      let left_arr = nums[0..len - 1].to_vec();
      let right_arr = nums[len - 1..nums.len()].to_vec();
      let sum = get_sum(&right_arr);
      if zero_one_pack(&left_arr, sum) {
        res = true;
        break;
      }
      len -= 1;
    }
    res
  }
}

fn get_sum(nums: &Vec<i32>) -> usize {
  let mut sum = 0;
  for i in nums {
    sum += i;
  }
  sum as usize
}
fn zero_one_pack(nums: &Vec<i32>, sum: usize) -> bool {
  // 01背包，物品个数为nums.len, 每个物品的重量为1,背包容量为nums.len
  let mut dp = vec![0; nums.len() + 1];

  let len = nums.len();
  for i in 0..len + 1 {
    for j in (1..nums.len() + 1).rev() {
      println!("{:?}", dp);
      dp[j] = max(dp[j], dp[j - 1] + nums[i] as usize)
    }
  }
  if dp[len - 1] == sum {
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
    let coins: Vec<i32> = vec![1, 5, 11, 5];
    Solution::can_partition(coins);
  }
}
