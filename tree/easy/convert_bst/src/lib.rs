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
// 给定一个二叉搜索树（Binary Search Tree），把它转换成为累加树（Greater Tree)，使得每个节点的值是原来的节点值加上所有大于它的节点值之和。

//

// 例如：

// 输入: 原始二叉搜索树:
//               5
//             /   \
//            2     13

// 输出: 转换为累加树:
//              18
//             /   \
//           20     13

use std::cell::RefCell;
use std::rc::Rc;

pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  check(&mut root, None);
  root
}

fn check(root: &mut Option<Rc<RefCell<TreeNode>>>, parent_val: Option<i32>) {
  if root.is_none() {
    return;
  }
  let root = root.as_mut().unwrap();
  let root_val = root.borrow().val;
  let mut val = root_val;
  let mut left = &mut root.borrow_mut().left;
  let mut right = &mut root.borrow_mut().right;
  if !parent_val.is_none() {
    // 加上父节点的val
    let parent_val = parent_val.as_ref().unwrap();
    if *parent_val > val {
      val += parent_val
    }
  }
  if !right.is_none() {
    // 加上右节点的val
    val += &right.as_ref().unwrap().borrow().val
  }
  root.borrow_mut().val = val;
  check(&mut left, Some(root_val));
  check(&mut right, Some(root_val));
}
