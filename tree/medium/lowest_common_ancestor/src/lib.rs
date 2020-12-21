/*
 * @lc app=leetcode.cn id=235 lang=rust
 *
 * [235] 二叉搜索树的最近公共祖先
 */

// Definition for a binary tree node.
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
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // 记录路径
        if root.is_none() {
            return None;
        }
        if p.is_none() {
            return q;
        }
        if q.is_none() {
            return p;
        }
        let mut path_q = vec![];
        let mut path_p = vec![];
        Self::find_path(&root, &mut path_q, &q);
        Self::find_path(&root, &mut path_p, &p);
        for i in 0..path_q.len() {
            let index = path_p.iter().position(|x| *x == path_q[i]);
            if index.is_some() {
                return path_p[index.unwrap()].take();
            }
        }
        None
    }
    fn find_path(
        // 自底向上记录path
        root: &Option<Rc<RefCell<TreeNode>>>,
        v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        node: &Option<Rc<RefCell<TreeNode>>>,
    ) {
        if root.is_none() {
            return;
        }
        let root_val = root.as_ref().unwrap().borrow().val;
        let node_val = node.as_ref().unwrap().borrow().val;
        if root_val == node_val {
            v.push(Some(Rc::clone(root.as_ref().unwrap())));
            return;
        }
        let right = &root.as_ref().unwrap().borrow().right;
        let left = &root.as_ref().unwrap().borrow().left;

        if root_val < node_val {
            Self::find_path(right, v, node);
            v.push(Some(Rc::clone(root.as_ref().unwrap())))
        } else {
            Self::find_path(left, v, node);
            v.push(Some(Rc::clone(root.as_ref().unwrap())))
        }
    }
}
// @lc code=end
