/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
use std::i64::MAX;
use std::i64::MIN;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        check(root, MAX, MIN)
    }
}
fn check(mut root: Option<Rc<RefCell<TreeNode>>>, upper: i64, lower: i64) -> bool {
    if root.is_none() {
        return true;
    }
    let val = root.as_ref().unwrap().borrow().val;
    let right = root.as_ref().unwrap().borrow_mut().right.take();
    let left = root.as_ref().unwrap().borrow_mut().left.take();
    if val as i64 <= lower || val as i64 >= upper {
        return false;
    }
    return check(left, val as i64, lower) && check(right, upper, val as i64);
}
// @lc code=end
