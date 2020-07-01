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
// 输入: [1,null,2,3]
//    1
//     \
//      2
//     /
//    3

// 输出: [1,2,3]
use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut v = vec![];
  if root.is_none() {
    return v;
  }
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
  v
}
