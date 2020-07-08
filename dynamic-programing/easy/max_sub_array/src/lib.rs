pub struct Solution {}

use std::cmp::max;

impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut sum = 0;
    for num in nums {
      if sum <= 0 {
        sum = num;
      } else {
        // 如果当前的sum大于0就一直加下去。如果小于0就从当前值重新开始
        sum += num;
      }
      res = max(res, sum);
    }
    res
  }
}
