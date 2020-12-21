/*
 * @lc app=leetcode.cn id=572 lang=rust
 *
 * [572] 另一个树的子树
 */

// #[derive(Debug, PartialEq, Eq)]
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
  pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if t.is_none() {
      return true;
    }
    if s.is_none() {
      return false;
    }
    let s_borrow = s.as_ref().unwrap().borrow();
    let s_left = if s_borrow.left.is_some() {
      Some(Rc::clone(s_borrow.left.as_ref().unwrap()))
    } else {
      None
    };
    let s_right = if s_borrow.right.is_some() {
      Some(Rc::clone(s_borrow.right.as_ref().unwrap()))
    } else {
      None
    };
    return Self::is_subtree(s_left, Some(Rc::clone(t.as_ref().unwrap())))
      || Self::is_subtree(s_right.clone(), Some(Rc::clone(t.as_ref().unwrap())))
      || Self::dfs(&s, &t);
  }
  fn dfs(s: &Option<Rc<RefCell<TreeNode>>>, t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if s.is_none() && t.is_none() {
      return true;
    }
    if s.is_none() || t.is_none() {
      return false;
    }
    let s_borrow = s.as_ref().unwrap().borrow();
    let s_val = s_borrow.val;
    let s_left = &s_borrow.left;
    let s_right = &s_borrow.right;
    let t_borrow = t.as_ref().unwrap().borrow();
    let t_val = t_borrow.val;
    let t_left = &t_borrow.left;
    let t_right = &t_borrow.right;
    if s_val != t_val {
      return false;
    }
    return Self::dfs(s_left, t_left) && Self::dfs(s_right, t_right);
  }
}
// @lc code=end
