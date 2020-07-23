struct Solution {}

impl Solution {
  pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    // 时间复杂度O(n)
    if nums.len() == 1 {
      return 0;
    }
    let mut c_max = nums[0]; // 记录当前选取的数组范围内的最大值
    let mut c_min = nums[0]; // 记录当前选取的数组范围内的最小值
    let mut start = 0;
    let mut end = 0;
    let mut sign = false;
    for i in 0..nums.len() {
      if sign == false && i + 1 < nums.len() && nums[i + 1] < nums[i] {
        start = i; // 6 记录数组开始的下标
        c_max = nums[i]; // 6
        c_min = nums[i + 1];
        end = i + 1; // 4 记录数组结束时的下标
        sign = true; // 只需要首次找到的结果即可
      }
      if nums[i] > c_max {
        c_max = nums[i];
      }
      if nums[i] < c_min {
        c_min = nums[i];
      }
      if nums[i] < c_max {
        // 如果当前值比之前选取的数组中的最大值小，则说明当前值应该被包含进去
        end = i;
      }
    }
    if end == start && end == 0 {
      return 0;
    }

    for i in (0..start).rev() {
      if nums[i] > c_min {
        // 如果start前面的数字小于当前数组中的最小值，则start往前移动
        start -= 1;
      } else {
        break;
      }
    }

    println!("{:?}", format!("{}{}", end, start));

    (end - start + 1) as i32
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]);
  }
}
