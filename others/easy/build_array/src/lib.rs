/*
 * @lc app=leetcode.cn id=1441 lang=rust
 *
 * [1441] 用栈操作构建数组
 */
struct Solution {}
// @lc code=start
impl Solution {
  // pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
  //     let mut v = vec![];
  //     if n == 0 {
  //         return v;
  //     }
  //     for i in 1..=n {
  //         if target[target.len() - 1] == target.len() as i32 && target[target.len() - 1] < i {
  //             break;
  //         }
  //         let index = target.iter().position(|&r| r == i);
  //         if index.is_none() {
  //             v.push("Push".to_string());
  //             v.push("Pop".to_string())
  //         } else {
  //             v.push("Push".to_string());
  //         }
  //     }
  //     v
  // }
  pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut v = vec![];
    for i in 1..=target[target.len() - 1] {
      let index = target.iter().position(|&r| r == i);
      if index.is_none() {
        v.push("Push".to_string());
        v.push("Pop".to_string())
      } else {
        v.push("Push".to_string());
      }
    }
    v
  }
}
// @lc code=end
