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

pub fn average_of_levels(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
  let mut v = vec![];
  if root.is_none() {
    return v;
  }
  let mut queue = VecDeque::new();
  queue.push_front(root);
  while queue.len() != 0 {
    let mut node_vec = vec![];
    let mut sum: f64 = 0.0;
    let mut len = queue.len();
    while queue.len() != 0 {
      let node = queue.pop_front().unwrap().unwrap();
      sum += node.borrow().val as f64;
      node_vec.push(node); // 将当前队列中的所有元素出队并保存，即当前行的所有元素
    }
    v.push((sum as f64) / (len as f64));
    for i in &node_vec {
      // 把当前行所有元素的下一行元素入队
      let mut node = i;
      if !node.borrow().left.is_none() {
        queue.push_back(node.borrow_mut().left.take());
      }
      if !node.borrow().right.is_none() {
        queue.push_back(node.borrow_mut().right.take());
      }
    }
  }
  v
}
