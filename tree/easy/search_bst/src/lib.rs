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

impl Solution {
  pub fn search_bst(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
      return root;
    }
    let node_val = root.as_ref().unwrap().borrow().val;
    if node_val == val {
      return root;
    }
    let left = root.as_ref().unwrap().borrow_mut().left.take();
    let right = root.as_ref().unwrap().borrow_mut().right.take();
    if node_val > val {
      return Self::search_bst(left, val);
    } else {
      return Self::search_bst(right, val);
    }
  }
}
