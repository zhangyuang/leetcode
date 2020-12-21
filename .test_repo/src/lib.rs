/*
 * @lc app=leetcode.cn id=572 lang=rust
 *
 * [572] 另一个树的子树
 */

// #[derive(Debug, PartialEq, Eq)]
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
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        mut s: Option<Rc<RefCell<TreeNode>>>,
        mut t: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if s.is_none() && t.is_none() {
            return true;
        }
        if s.is_none() && t.is_some() {
            return false;
        }
        if s.is_some() && t.is_none() {
            return false;
        }
        let mut s_borrow = s.as_ref().unwrap().borrow();
        let s_val = s_borrow.val;
        let s_left = s_borrow.left.clone();
        let s_right = s_borrow.right.clone();
        let mut t_borrow = t.as_ref().unwrap().borrow();
        let t_val = t_borrow.val;
        let t_left = t_borrow.left.clone();
        let t_right = t_borrow.right.clone();
        return (s_val == t_val
            && Self::is_subtree(s_left.clone(), t_left)
            && Self::is_subtree(s_right.clone(), t_right))
            || Self::is_subtree(s_left, t.clone())
            || Self::is_subtree(s_right, t.clone());
    }
}
// @lc code=end
