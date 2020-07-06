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
// 给定一棵二叉搜索树，请找出其中第k大的节点。

//

// 示例 1:

// 输入: root = [3,1,4,null,2], k = 1
//    3
//   / \
//  1   4
//   \
//    2
// 输出: 4

use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_largest(mut root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
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
    v.push(root.as_mut().unwrap().borrow().val);
    root = root.unwrap().borrow_mut().right.take();
  }
  return *v.get(v.len() - k as usize).unwrap();
}
