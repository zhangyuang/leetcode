/*
 * @lc app=leetcode.cn id=687 lang=rust
 *
 * [687] 最长同值路径
 */
// Definition for a binary tree node.
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
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn longest_univalue_path(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let res = &mut 0;
        Self::arrow_length(&root, res);
        *res
    }
    fn arrow_length(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root_borrow_mut = root.as_ref().unwrap().borrow();
        let left = &root_borrow_mut.left;
        let right = &root_borrow_mut.right;
        let root_val = root_borrow_mut.val;
        let arrow_length_left = Self::arrow_length(left, res);
        let arrow_length_right = Self::arrow_length(right, res);
        let mut left_length = 0;
        let mut right_length = 0;

        if left.is_some() {
            let left_val = left.as_ref().unwrap().borrow().val;
            if left_val == root_val {
                left_length = arrow_length_left + 1;
            }
        }
        if right.is_some() {
            let right_val = right.as_ref().unwrap().borrow().val;
            if right_val == root_val {
                right_length = arrow_length_right + 1;
            }
        }
        *res = max(*res, right_length + left_length);
        return max(right_length, left_length);
    }
}
// @lc code=end
