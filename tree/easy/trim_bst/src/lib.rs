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

// 给定二叉搜索树（BST）的根节点和一个值。 你需要在BST中找到节点值等于给定值的节点。 返回以该节点为根的子树。 如果节点不存在，则返回 NULL。

// 例如，

// 给定二叉搜索树:

//         4
//        / \
//       2   7
//      / \
//     1   3

// 和值: 2
// 你应该返回如下子树:

//       2
//      / \
//     1   3

use std::cell::RefCell;
use std::rc::Rc;

pub fn trim_bst(
  mut root: Option<Rc<RefCell<TreeNode>>>,
  l: i32,
  r: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
  cut_branch(&mut root, l, r);
  root
}

fn cut_branch(mut root: &mut Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) {
  let val = root.as_mut().unwrap().borrow_mut().val;
  let mut left = root.as_mut().unwrap().borrow_mut().left.take();
  let mut right = root.as_mut().unwrap().borrow_mut().right.take();
  if val < l || val > r {
    // if left.is_none() && right.is_none() {
    //   root = None;
    // }
    if val < l {
      root = &mut right;
    }
    if val > r {
      root = &mut left;
    }
  }
  cut_branch(&mut left, l, r);
  cut_branch(&mut right, l, r);
}
