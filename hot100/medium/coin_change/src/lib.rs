struct Solution {}
use std::cmp::max;
use std::cmp::min;

impl Solution {
  // 完全背包问题，个人认为相比于一维数组简化版这样写更容易理解
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut coins_u: Vec<usize> = vec![];
    for i in coins {
      // convert i32 -> usize
      coins_u.push(i as usize);
    }
    let amount = amount as usize;
    let mut dp = vec![vec![0; amount + 1]; coins_u.len() + 1];
    let mut G = vec![vec![0; amount + 1]; coins_u.len() + 1];

    // dp[i][j] 表示i个硬币装进容量为j的背包的最大价值
    for i in 0..coins_u.len() {
      // 初始化数据，把i个硬币装进容量为0的容器价值都是0
      if dp.get(i).is_none() {
        dp[i] = vec![0; amount + 1];
        dp[i][0] = 0;
      }
      if G.get(i).is_none() {
        G[i] = vec![0; amount + 1];
        G[i][0] = 0;
      }
    }
    let mut res = 9999;
    for i in 1..coins_u.len() + 1 {
      for j in 1..amount + 1 {
        if (coins_u[i - 1]) <= j {
          // 两种可能，是否选择第i个硬币
          // 当前硬币的价格必须小于j才能组成集合
          let v = coins_u[i - 1]; // 当前硬币的价值
          if dp[i - 1][j] < (dp[i][j - v] + v) {
            // 说明往里面增加了新硬币
            G[i][j] = 1
          }
          dp[i][j] = max(dp[i - 1][j], dp[i][j - v] + v);
        } else {
          dp[i][j] = dp[i - 1][j];
        }
        if dp[i][j] == amount {
          for i in 1..coins_u.len() + 1 {
            let mut foo = 0;
            for j in 1..amount + 1 {
              foo += G[i][j];
            }
            res = min(res, foo);
            println!("{}", res);
          }
        }
      }
    }
    -1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let coins: Vec<i32> = vec![1, 2, 5, 7, 8];
    Solution::coin_change(coins, 102);
  }
}
