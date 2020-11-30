/*
 * @lc app=leetcode.cn id=103 lang=rust
 *
 * [103] 二叉树的锯齿形层次遍历
 */
struct Solution {}
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
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut v = vec![];
        if root.is_none() {
            return v;
        }
        let mut queue = VecDeque::new();
        queue.push_front(root);
        let mut from_left = false;
        while queue.len() != 0 {
            let mut node_vec = vec![]; // 暂时保存当前行的所有元素
            let mut val_vec = vec![];
            while queue.len() != 0 {
                let node = queue.pop_front().unwrap().unwrap();
                val_vec.push(node.borrow().val); // 记录当前行的所有元素的val
                node_vec.push(node);
            }
            v.push(val_vec);
            while node_vec.len() != 0 {
                let node = node_vec.pop().unwrap();
                // 把当前行所有元素的下一行元素入队
                if from_left {
                    if !node.borrow().left.is_none() {
                        queue.push_back(node.borrow_mut().left.take());
                    }
                    if !node.borrow().right.is_none() {
                        queue.push_back(node.borrow_mut().right.take());
                    }
                } else {
                    if !node.borrow().right.is_none() {
                        queue.push_back(node.borrow_mut().right.take());
                    }
                    if !node.borrow().left.is_none() {
                        queue.push_back(node.borrow_mut().left.take());
                    }
                }
            }
            from_left = !from_left;
        }
        v
    }
}
// @lc code=end
