/*
 * @lc app=leetcode.cn id=563 lang=rust
 *
 * [563] 二叉树的坡度
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
use std::rc::Rc;
impl Solution {
    // [4,2,9,3,5,null,7]
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if root.is_none() {
                return 0;
            }
            let root_val = root.as_ref().unwrap().borrow().val;
            let left_node = &root.as_ref().unwrap().borrow().left;
            let right_node = &root.as_ref().unwrap().borrow().right;
            let left_val = dfs(left_node, res);
            let right_val = dfs(right_node, res);
            *res = *res + (left_val - right_val).abs();
            return root_val + left_val + right_val;
        };
        dfs(&root, &mut res);
        res
    }
    // fn get_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if root.is_none() {
    //         return 0;
    //     };
    //     let root_val = root.as_ref().unwrap().borrow().val;
    //     let left_node = &root.as_ref().unwrap().borrow().left;
    //     let right_node = &root.as_ref().unwrap().borrow().right;

    //     return root_val + Self::get_sum(left_node) + Self::get_sum(right_node);
    // }
}
// @lc code=end
