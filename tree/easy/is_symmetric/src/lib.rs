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
use std::rc::Rc;

pub fn is_symmetric(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if root.is_none() {
    return true;
  }
  let left = root.as_mut().unwrap().borrow_mut().left.take();
  let right = root.as_mut().unwrap().borrow_mut().right.take();
  return check(left, right);
}

fn check(
  mut node1: Option<Rc<RefCell<TreeNode>>>,
  mut node2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
  if node1.is_none() && node2.is_none() {
    return true;
  }
  if node1.is_none() || node2.is_none() {
    return false;
  }
  let node1_left = node1.as_mut().unwrap().borrow_mut().left.take();
  let node1_right = node1.as_mut().unwrap().borrow_mut().right.take();
  let node2_left = node2.as_mut().unwrap().borrow_mut().left.take();
  let node2_right = node2.as_mut().unwrap().borrow_mut().right.take();
  return node1.as_ref().unwrap().borrow().val == node2.as_ref().unwrap().borrow().val
    && check(node1_left, node2_right)
    && check(node2_left, node1_right);
}
