/*
 * @lc app=leetcode.cn id=872 lang=rust
 *
 * [872] 叶子相似的树
 */
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}
struct Solution {}
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
  ) -> bool {
    let mut leaf_1 = vec![];
    Self::get_leaf(&root1, &mut leaf_1);
    let mut leaf_2 = vec![];
    Self::get_leaf(&root2, &mut leaf_2);
    if leaf_1.len() != leaf_2.len() {
      return false;
    }
    let mut res = true;
    for i in 0..leaf_1.len() {
      if leaf_1[i] != leaf_2[i] {
        res = false;
        break;
      }
    }
    res
  }
  fn get_leaf(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    if root.is_none() {
      return;
    }
    let root_borrow = root.as_ref().unwrap().borrow();
    let left = &root_borrow.left;
    let right = &root_borrow.right;
    if left.is_none() && right.is_none() {
      v.push(root_borrow.val);
      return;
    }
    Self::get_leaf(left, v);
    Self::get_leaf(right, v);
  }
}
// @lc code=end
