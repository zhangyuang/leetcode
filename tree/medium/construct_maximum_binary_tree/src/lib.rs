/*
 * @lc app=leetcode.cn id=654 lang=rust
 *
 * [654] 最大二叉树
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        let index = Self::get_max(&nums);
        let mut root = Self::create_node(nums[index]);
        let nums_left = nums[0..index].to_vec();
        let nums_right = nums[index + 1..nums.len()].to_vec();
        root.as_mut()?.borrow_mut().left = Self::construct_maximum_binary_tree(nums_left);
        root.as_mut()?.borrow_mut().right = Self::construct_maximum_binary_tree(nums_right);
        root
    }
    fn get_max(v: &Vec<i32>) -> usize {
        let mut index = 0;
        let mut MAX = v[index];
        for i in 0..v.len() {
            if v[i] > MAX {
                MAX = v[i];
                index = i
            }
        }
        index
    }
    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}
// @lc code=end
