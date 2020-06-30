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

impl Solution {
  pub fn preorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v = vec![];
    let mut stack = vec![];
    let mut root = &mut root;
    while let Some(mut node) = root {
      if node.borrow_mut().left != None {
        v.push(node.borrow_mut().val);
        stack.push(node.borrow_mut());
        root = &mut node.borrow_mut().left;
      } else {
        root = &mut stack.pop();
      }
    }
    v
  }
}
