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

// 输入:

//    1
//  /   \
// 2     3
//  \
//   5

// 输出: ["1->2->5", "1->3"]

// 解释: 所有根节点到叶子节点的路径为: 1->2->5, 1->3

use std::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
  let mut v = vec![];
  find_leaf(&root, &mut v, None);
  v
}

fn find_leaf(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<String>, str: Option<&mut String>) {
  // 递归找到叶子节点记录路径即可
  if root.is_none() {
    return;
  }
  let val = &root.as_ref().unwrap().borrow().val.to_string();
  let left = &root.as_ref().unwrap().borrow().left;
  let right = &root.as_ref().unwrap().borrow().right;

  let mut res_str;
  match str {
    None => {
      res_str = String::from("");
    }
    Some(str) => {
      res_str = str.to_string();
    }
  }
  res_str.push_str(val);
  if left.is_none() && right.is_none() {
    // 说明当前为叶子节点
    v.push(res_str);
    return;
  }
  res_str.push_str("->");
  find_leaf(left, v, Some(&mut res_str));
  find_leaf(right, v, Some(&mut res_str));
}
