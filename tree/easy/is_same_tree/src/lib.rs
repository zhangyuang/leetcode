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

struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn is_same_tree(
    mut p: Option<Rc<RefCell<TreeNode>>>,
    mut q: Option<Rc<RefCell<TreeNode>>>,
  ) -> bool {
    check(p, q)
  }
}
fn check(mut p: Option<Rc<RefCell<TreeNode>>>, mut q: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if p.is_none() && q.is_none() {
    return true;
  }
  if p.is_none() || q.is_none() {
    return false;
  }

  if let Some(p_node) = &p {
    let p_left = p_node.borrow_mut().left.take();
    let p_right = p_node.borrow_mut().right.take();
    if let Some(q_node) = &q {
      let q_left = q_node.borrow_mut().left.take();
      let q_right = q_node.borrow_mut().right.take();
      return p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val
        && check(p_left, q_left)
        && check(q_right, p_right);
    }
  }
  false
}
