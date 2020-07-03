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
use std::cmp::max;
use std::rc::Rc;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if root.is_none() {
    return true;
  }

  check(root)
}

fn check(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if root.is_none() {
    return true;
  }
  let left = root.as_mut().unwrap().borrow_mut().left.take();
  let right = root.as_mut().unwrap().borrow_mut().right.take();
  return (get_height(&left) - get_height(&right)).abs() <= 1 && check(left) && check(right);
}
fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
  // 获取改节点的最大高度
  if root.is_none() {
    return 0;
  }
  let left = &root.as_ref().unwrap().borrow().left;
  let right = &root.as_ref().unwrap().borrow().right;
  return max(get_height(&left), get_height(&right)) + 1;
}
