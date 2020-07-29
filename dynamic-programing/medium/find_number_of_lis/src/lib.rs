struct Solution {}
use std::cmp::max;

// 给定一个未排序的整数数组，找到最长递增子序列的个数。

// 示例 1:

// 输入: [1,3,5,4,7]
// 输出: 2
// 解释: 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
// dp=[1,2,3,3,4]

impl Solution {
  // 动态规划+回溯倒推路径
  pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
      return nums.len() as i32;
    }
    let mut dp = vec![1; nums.len()];
    let mut path = vec![];
    let mut max_len = 0;
    // dp[i]代表到i为止最长的递增子序列长度
    for i in 1..nums.len() {
      for j in 0..i {
        if nums[i] > nums[j] {
          dp[i] = max(dp[i], dp[j] + 1)
        }
      }

      max_len = max(dp[i], max_len);
    }
    if max_len == 1 {
      return dp.len() as i32;
    }
    for i in 0..dp.len() {
      // 兼容有多个元素都可以作为递增子序列结尾的情况
      if dp[i] == max_len {
        let max_len_index = i;
        let mut item = vec![nums[max_len_index]]; // vec第一个元素为递增子序列的结尾元素
        println!("{:?}", nums);
        println!("{:?}", dp);
        find_path(
          &nums,
          &dp,
          &mut path,
          max_len as usize,
          max_len_index as usize,
          &mut item,
        );
        println!("{:?}", path);
      }
    }
    path.len() as i32
  }
}
fn find_path(
  nums: &Vec<i32>,
  dp: &Vec<i32>,
  path: &mut Vec<Vec<i32>>,
  max_len: usize,
  max_len_index: usize,
  item: &mut Vec<i32>,
) {
  if item.len() == max_len {
    path.push(item.clone());
  }
  // println!("{:?}", format!("{:?}{}", item, max_len_index));

  for i in (0..max_len_index).rev() {
    if dp[i] != dp[max_len_index] as i32 - 1 {
      // 当max_len_index为5时，需要找到值4的元素
      continue;
    }
    if nums[i] >= item[item.len() - 1] {
      continue;
    }
    item.push(nums[i]);
    find_path(nums, dp, path, max_len, i, item);
    item.pop();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::find_number_of_lis(vec![1, 3, 2]);
  }
}
