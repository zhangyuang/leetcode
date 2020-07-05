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
// 给定一个有相同值的二叉搜索树（BST），找出 BST 中的所有众数（出现频率最高的元素）。

// 假定 BST 有如下定义：

// 结点左子树中所含结点的值小于等于当前结点的值
// 结点右子树中所含结点的值大于等于当前结点的值
// 左子树和右子树都是二叉搜索树
// 例如：
// 给定 BST [1,null,2,2],

//    1
//     \
//      2
//     /
//    2
// 返回[2].

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn find_mode(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  if root.is_none() {
    return vec![];
  }
  let mut max = 1; // 记录出现最多的次数
  let mut pre_val = 99999;
  let mut hash_map = HashMap::new();
  // 先中序遍历得到递增数组
  let mut v = vec![];
  let mut res: Vec<i32> = vec![];
  if root.is_none() {
    return v;
  }
  let mut stack = vec![];
  let mut temp_max = 1;
  while stack.len() != 0 || !root.is_none() {
    while !root.is_none() {
      // 一直添加左子树直到为空
      let node = root.unwrap();
      root = node.borrow_mut().left.take();
      stack.push(node);
    }
    // 从栈中弹出，取节点的右子树
    root = stack.pop();
    if root.as_mut().unwrap().borrow().val == pre_val {
      temp_max += 1;
      if temp_max > max {
        max = temp_max
      }
    } else {
      temp_max = 1
    }
    v.push(root.as_mut().unwrap().borrow().val);
    pre_val = root.as_mut().unwrap().borrow().val;
    root = root.unwrap().borrow_mut().right.take();
  }

  // 遍历一遍递归数组，记录出现的次数等于max为众数(不是最优解但易于理解)
  for i in &v {
    if hash_map.get(i).is_none() {
      hash_map.insert(i, 1);
    } else {
      hash_map.insert(i, hash_map.get(i).unwrap() + 1);
    }
    if *hash_map.get(i).unwrap() == max {
      res.push(*i)
    }
  }
  res
}
