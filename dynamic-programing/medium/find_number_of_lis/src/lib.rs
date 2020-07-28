struct Solution {}
use std::cmp::max;

// 给定一个未排序的整数数组，找到最长递增子序列的个数。

// 示例 1:

// 输入: [1,3,5,4,7]
// 输出: 2
// 解释: 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
// dp=[1,2,3,3,4]

impl Solution {
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
      max_len = max(max_len, dp[i]);
    }
    let mut item = vec![dp[0]];
    if max_len == 1 {
      return dp.len() as i32;
    }
    println!("{:?}", dp);

    Solution::find_path(&dp, &mut path, max_len as usize, &mut item);
    println!("{:?}", path);
    path.len() as i32
  }
  fn find_path(dp: &Vec<i32>, path: &mut Vec<Vec<i32>>, max_len: usize, item: &mut Vec<i32>) {
    if item.len() == max_len {
      // 找4个数字的递增全排列
      path.push(item.clone());
    }
    for i in 0..dp.len() {
      if dp[i] <= item[item.len() - 1] {
        continue;
      }
      // 如果dp数组当前的值大于item当前的最大值，则添加进去
      item.push(dp[i]);
      Solution::find_path(dp, path, max_len, item);
      item.pop();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]);
  }
}
