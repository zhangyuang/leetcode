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

pub fn flatten(mut root: &mut Option<Rc<RefCell<TreeNode>>>) {
  if root.is_none() {
    return;
  }
  let mut v = vec![];
  let mut stack = vec![];
  let mut root_clone = root.clone();
  while stack.len() != 0 || !root_clone.is_none() {
    while !root_clone.is_none() {
      // 一直添加左子树直到为空
      let node = root_clone.unwrap();
      v.push(node.clone());
      root_clone = node.borrow_mut().left.take();
      stack.push(node);
    }
    // 从栈中弹出，取节点的右子树
    root_clone = stack.pop();
    root_clone = root_clone.unwrap().borrow_mut().right.take();
  }
  for i in 1..v.len() {
    let pre = &v[i - 1];
    let curr = &v[i];
    pre.as_ref().borrow_mut().left = None;
    pre.as_ref().borrow_mut().right = Some(curr.clone());
  }
}
