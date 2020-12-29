/*
 * @lc app=leetcode.cn id=222 lang=rust
 *
 * [222] 完全二叉树的节点个数
 */
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
    // 解法1，计算高度，加上叶子节点的个数
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let height = Self::get_height(&root).unwrap();
        if height == 1 || height == 0 {
            return height;
        }
        let leaf = Self::get_leaf(&root, 1, height).unwrap();
        let two: i32 = 2;
        return two.pow((height - 1) as u32) - 1 + leaf;
    }
    fn get_leaf(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut current_height: i32,
        height: i32,
    ) -> Option<i32> {
        if root.is_none() {
            return Some(0);
        }
        let root_borrow = root.as_ref()?.borrow();
        let left = &root_borrow.left;
        let right = &root_borrow.right;
        if left.is_none() && right.is_none() && current_height == height {
            return Some(1);
        }
        current_height = current_height + 1;
        return Some(
            Self::get_leaf(left, current_height, height)?
                + Self::get_leaf(right, current_height, height)?,
        );
    }
    fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if root.is_none() {
            return Some(0);
        }
        let root_borrow = root.as_ref()?.borrow();
        let left = &root_borrow.left;
        let right = &root_borrow.right;
        return Some(max(Self::get_height(left)?, Self::get_height(right)?) + 1);
    }
}
// @lc code=end
