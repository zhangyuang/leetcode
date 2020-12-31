/*
 * @lc app=leetcode.cn id=303 lang=rust
 *
 * [303] 区域和检索 - 数组不可变
 */

// @lc code=start
struct NumArray {
  nums: Vec<i32>,
  subscript: Vec<i32>, // 代表 以 i 为 右侧索引的范围和
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl NumArray {
  // fn new(nums: Vec<i32>) -> Self {
  //     NumArray {
  //         subscript: vec![0; nums.len()],
  //         nums,
  //     }
  // }

  // fn sum_range(&mut self, i: i32, j: i32) -> i32 {
  //     let start = i as usize;
  //     let end = j as usize;
  //     if start != 0 && self.subscript[start - 1] == 0 {
  //         let mut sum = 0;
  //         for i in 0..=start - 1 {
  //             sum += self.nums[i]
  //         }
  //         self.subscript[start - 1] = sum
  //     }
  //     if self.subscript[end] == 0 {
  //         let mut sum = 0;
  //         for i in 0..=end {
  //             sum += self.nums[i]
  //         }
  //         self.subscript[end] = sum
  //     }
  //     if start == 0 {
  //         self.subscript[end]
  //     } else {
  //         self.subscript[end] - self.subscript[start - 1]
  //     }
  // }
  fn new(nums: Vec<i32>) -> Self {
    let mut subscript = vec![0; nums.len()];
    let mut sum = 0;
    for i in 0..nums.len() {
      sum += nums[i];
      subscript[i] = sum;
    }
    NumArray { subscript, nums }
  }

  fn sum_range(&mut self, i: i32, j: i32) -> i32 {
    if i == 0 {
      self.subscript[j as usize]
    } else {
      self.subscript[j as usize] - self.subscript[i as usize - 1]
    }
  }
}

// /**
//  * Your NumArray object will be instantiated and called as such:
//  * let obj = NumArray::new(nums);
//  * let ret_1: i32 = obj.sum_range(i, j);
//  */
// @lc code=end
