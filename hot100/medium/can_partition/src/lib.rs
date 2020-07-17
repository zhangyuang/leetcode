struct Solution {}
use std::cmp::max;

impl Solution {
  // 01背包变种
  pub fn can_partition(nums: Vec<i32>) -> bool {
    // 由于两个集合相等，即每一个集合的sum为总和/2
    let sum = get_sum(&nums);
    if sum % 2 != 0 {
      // 和为奇数直接跳出
      return false;
    }
    let sum_half = sum / 2;
    zero_one_pack(&nums, sum_half)
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
  // 01背包，物品个数为nums.len,背包容量为sum
  let mut dp = vec![0; sum + 1];
  let mut res = false;
  for i in 1..nums.len() + 1 {
    let weight = nums[i - 1];
    let val = nums[i - 1];
    for j in (1..sum + 1).rev() {
      if j >= weight as usize {
        dp[j] = max(dp[j], dp[j - weight as usize] + val as usize);
      }
    }
  }
  println!("{:?}", dp);
  println!("{:?}", sum);
  if dp.pop().unwrap() == sum {
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
    let coins: Vec<i32> = vec![1, 2, 3, 6];
    Solution::can_partition(coins);
  }
}
