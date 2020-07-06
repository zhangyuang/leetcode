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
// 输入: root = [4,2,6,1,3,null,null]
// 输出: 1
// 解释:
// 注意，root是树节点对象(TreeNode object)，而不是数组。

// 给定的树 [4,2,6,1,3,null,null] 可表示为下图:

//           4
//         /   \
//       2      6
//      / \
//     1   3

// 最小的差值是 1, 它是节点1和节点2的差值, 也是节点3和节点2的差值。

use std::cell::RefCell;
use std::rc::Rc;

pub fn min_diff_in_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
  let mut min = 999;
  let mut i = 0;
  while i + 1 < v.len() {
    let res = v.get(i + 1).unwrap() - v.get(i).unwrap();
    if res < min {
      min = res
    }
    i += 1;
  }
  min
}
