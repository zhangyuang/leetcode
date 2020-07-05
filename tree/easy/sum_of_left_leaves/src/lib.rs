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
// 计算给定二叉树的所有左叶子之和。

// 3
// / \
// 9  20
//  /  \
// 15   7

// 在这个二叉树中，有两个左叶子，分别是 9 和 15，所以返回 24

use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  let mut v = vec![];
  find_leaf(&root, &mut v, "root".to_string());
  let mut sum = 0;
  for i in &v {
    sum += i;
  }

  sum
}

fn find_leaf(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>, node_type: String) {
  // 递归找到叶子节点记录路径即可
  if root.is_none() {
    return;
  }
  let val = &root.as_ref().unwrap().borrow().val;
  let left = &root.as_ref().unwrap().borrow().left;
  let right = &root.as_ref().unwrap().borrow().right;

  if left.is_none() && right.is_none() {
    // 说明当前为左叶子节点
    if node_type == "left" {
      v.push(*val);
    }
    return;
  }
  find_leaf(left, v, "left".to_string());
  find_leaf(right, v, "right".to_string());
}
