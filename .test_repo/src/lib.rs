/*
 * @lc app=leetcode.cn id=687 lang=rust
 *
 * [687] 最长同值路径
 */
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
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let res = find(&root, 0);
        res
    }
}
fn find(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
    if root.is_none() {
        return val;
    }
    if let Some(root_unwrap) = root {
        let left = &root_unwrap.borrow().left;
        let right = &root_unwrap.borrow().right;
        let mut left_val = 0;
        let mut right_val = 0;
        if left.is_some()
            && left.as_ref().unwrap().borrow().val == root_unwrap.borrow().val
            && right.is_some()
            && right.as_ref().unwrap().borrow().val == root_unwrap.borrow().val
        {
            left_val = find(left, right_val + 1);
            right_val = find(right, left_val + 1);
            return max(val, max(left_val, right_val));
        }
        if left.is_some() && left.as_ref().unwrap().borrow().val == root_unwrap.borrow().val {
            left_val = find(left, val + 1);
        } else {
            left_val = find(left, 0)
        }
        if right.is_some() && right.as_ref().unwrap().borrow().val == root_unwrap.borrow().val {
            right_val = find(right, val + 1);
        } else {
            right_val = find(right, 0)
        }
        return max(val, max(left_val, right_val));
    }
    val
}
// @lc code=end
