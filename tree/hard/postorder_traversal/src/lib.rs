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

// 输出: [3,2,1]
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn postorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  // 这里使用了队列，但是返回结果需要是vector，最后一个遍历返回venctor
  let mut queue = VecDeque::new();
  let mut v = vec![];
  if root.is_none() {
    return v;
  }
  let mut stack = vec![root];
  while stack.len() != 0 {
    let mut node = stack.pop().unwrap().unwrap(); // stack.pop返回option所以这里需要执行两次unwrap
    queue.push_front(node.borrow().val); // 队列入队，相当于数组的unshift方法从左边插入
    if !node.borrow().left.is_none() {
      stack.push(node.borrow_mut().left.take())
    }
    if !node.borrow().right.is_none() {
      stack.push(node.borrow_mut().right.take())
    }
  }
  while queue.len() != 0 {
    v.push(queue.pop_front().unwrap())
  }
  v
}
