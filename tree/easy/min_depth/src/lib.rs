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

// 给定二叉树 [3,9,20,null,null,15,7],

//     3
//    / \
//   9  20
//     /  \
//    15   7
// 返回它的最小深度  2.

use std::cell::RefCell;
use std::rc::Rc;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if root.is_none() {
    return 0;
  }
  let mut v = vec![];
  find_leaf(&root, &mut v, Some(1));
  let mut min = v.get(0).unwrap();
  for i in &v {
    if i < min {
      min = i;
    }
  }
  *min
}

fn find_leaf(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>, depth: Option<i32>) {
  // 递归找到叶子节点记录路径即可
  if root.is_none() {
    return;
  }
  let left = &root.as_ref().unwrap().borrow().left;
  let right = &root.as_ref().unwrap().borrow().right;

  let mut depth = depth.unwrap();
  if left.is_none() && right.is_none() {
    // 说明当前为叶子节点
    v.push(depth);
    return;
  }
  depth += 1;

  find_leaf(left, v, Some(depth));
  find_leaf(right, v, Some(depth));
}
