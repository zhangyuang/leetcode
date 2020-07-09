pub struct Solution {}
use std::cmp::max;
use std::cmp::min;

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 {
      return 0;
    }
    let mut MIN = prices[0];
    let mut MAX = 0;
    for i in &prices {
      MIN = min(MIN, *i); // 找到当天之前的最小值
      println!("{:?}", MIN);
      MAX = max(MAX, i - MIN);
    }
    MAX
  }
}
