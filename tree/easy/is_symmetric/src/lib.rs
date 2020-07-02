//Definition for a binary tree node.
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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn is_symmetric(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if root.is_none() {
    return true;
  }
  let mut res = true;
  let mut node = root.as_mut().unwrap();
  if !node.borrow_mut().left.is_none() || !node.borrow_mut().right.is_none() {
    let left = node.borrow_mut().left;
    let right = node.borrow_mut().right;
    if left.val != right.val {
      res = false
    }
  }
  res
}
