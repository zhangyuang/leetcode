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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    // 解法1，计算高度，加上叶子节点的个数
    // pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let height = Self::get_height(&root).unwrap();
    //     if height == 1 || height == 0 {
    //         return height;
    //     }
    //     let leaf = Self::get_leaf(&root, 1, height).unwrap();
    //     let two: i32 = 2;
    //     return two.pow((height - 1) as u32) - 1 + leaf;
    // }
    // fn get_leaf(
    //     root: &Option<Rc<RefCell<TreeNode>>>,
    //     mut current_height: i32,
    //     height: i32,
    // ) -> Option<i32> {
    //     if root.is_none() {
    //         return Some(0);
    //     }
    //     let root_borrow = root.as_ref()?.borrow();
    //     let left = &root_borrow.left;
    //     let right = &root_borrow.right;
    //     if left.is_none() && right.is_none() && current_height == height {
    //         return Some(1);
    //     }
    //     current_height = current_height + 1;
    //     return Some(
    //         Self::get_leaf(left, current_height, height)?
    //             + Self::get_leaf(right, current_height, height)?,
    //     );
    // }
    // fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    //     if root.is_none() {
    //         return Some(0);
    //     }
    //     let root_borrow = root.as_ref()?.borrow();
    //     let left = &root_borrow.left;
    //     let right = &root_borrow.right;
    //     return Some(max(Self::get_height(left)?, Self::get_height(right)?) + 1);
    // }
    // 解法 2 bfs
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        queue.push_back(root);
        let mut res = 0;
        while queue.len() != 0 {
            let mut node_vec = vec![];
            while queue.len() != 0 {
                let node = queue.pop_front().unwrap();
                if node.is_some() {
                    node_vec.push(Self::create_rc(&node)); // 暂存当前行所有元素
                    res += 1; // 保存结果
                }
            }
            for i in &node_vec {
                // 把下一行所有元素入队
                let node_borrow = i.as_ref().unwrap().borrow();
                let left = Self::create_rc(&node_borrow.left);
                let right = Self::create_rc(&node_borrow.right);
                if left.is_some() {
                    queue.push_back(left)
                }
                if right.is_some() {
                    queue.push_back(right)
                }
            }
        }
        res
    }
    fn create_rc(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        return Some(Rc::clone(root.as_ref().unwrap()));
    }
}
// @lc code=end
