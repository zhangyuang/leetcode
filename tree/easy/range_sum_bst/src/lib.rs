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
// 输入：root = [10,5,15,3,7,null,18], L = 7, R = 15
// 输出：32

use std::cell::RefCell;
use std::rc::Rc;

pub fn range_sum_bst(mut root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
  let mut left = 0 as usize;
  let mut right = 0 as usize;
  if root.is_none() {
    return 0;
  }
  let mut v = vec![];
  let mut stack = vec![];
  while stack.len() != 0 || !root.is_none() {
    while !root.is_none() {
      // 一直添加左子树直到为空
      let node = root.unwrap();
      root = node.borrow_mut().left.take();
      stack.push(node);
    }
    // 从栈中弹出，取节点的右子树
    root = stack.pop();
    if root.as_mut().unwrap().borrow().val == l {
      left = v.len() as usize;
    }
    if root.as_mut().unwrap().borrow().val == r {
      right = v.len() as usize;
    }
    v.push(root.as_mut().unwrap().borrow().val);
    root = root.unwrap().borrow_mut().right.take();
  }
  let v = &v[left..(right + 1)];
  v.iter().sum()
}
