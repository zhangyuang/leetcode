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

// 一开始理解错了题意，以为是加上相连的节点的val，没想到是所有节点。这题如果改成相连的节点用Rust编写的难度更大，解法如下
// pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//   root = get_sum(root, None);
//   root
// }

// fn get_sum(
//   mut root: Option<Rc<RefCell<TreeNode>>>,
//   parent_val: Option<i32>,
// ) -> Option<Rc<RefCell<TreeNode>>> {
//   if root.is_none() {
//     return root;
//   }
//   let root_unwrap = root.as_mut().unwrap();
//   let mut val = root_unwrap.borrow().val;
//   let left = root_unwrap.borrow_mut().left.take();
//   let right = root_unwrap.borrow_mut().right.take();
//   if !parent_val.is_none() {
//     // 加上父节点的val
//     let parent_val = parent_val.as_ref().unwrap();
//     if *parent_val > val {
//       val += parent_val
//     }
//   }
//   if !right.is_none() {
//     // 加上右节点的val
//     val += &right.as_ref().unwrap().borrow().val
//   }
//   root_unwrap.borrow_mut().val = val;
//   root_unwrap.borrow_mut().left = get_sum(left, Some(val));
//   root_unwrap.borrow_mut().right = get_sum(right, Some(val));
//   root
// }

// 符合题意的解法，反序中序遍历。依次访问值最大的节点
pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  let mut v = vec![];
  if root.is_none() {
    return root;
  }
  let mut stack = vec![];
  let mut root_clone = root.clone();
  while stack.len() != 0 || !root_clone.is_none() {
    while !root_clone.is_none() {
      // 一直添加右子树直到为空
      let node = root_clone.unwrap();
      root_clone = node.borrow_mut().right.clone();
      stack.push(node);
    }
    // 从栈中弹出，取节点的左子树
    root_clone = stack.pop();
    let mut root_val = root_clone.as_mut().unwrap().borrow().val;
    if v.len() != 0 {
      for i in &v {
        root_val += i;
      }
    }
    v.push(root_clone.as_mut().unwrap().borrow().val);
    root_clone.as_mut().unwrap().borrow_mut().val = root_val;
    root_clone = root_clone.unwrap().borrow_mut().left.clone();
  }
  root
}
