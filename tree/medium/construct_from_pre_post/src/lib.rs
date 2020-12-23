/*
 * @lc app=leetcode.cn id=889 lang=rust
 *
 * [889] 根据前序和后序遍历构造二叉树
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut hash_map = HashMap::new();
        for i in 0..pre.len() {
            hash_map.insert(pre[i], i);
        }

        Self::build_tree(&post, 0, post.len() as i32 - 1, &hash_map, 0)
    }
    fn build_tree(
        post: &Vec<i32>,
        postL: i32, // 后序遍历左边坐标
        postR: i32, // 后序遍历右边坐标
        hash_map: &HashMap<i32, usize>,
        preL: i32, // 前序遍历左边坐标
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if postL > postR {
            return None;
        }
        let mut root = Self::create_node(post[postR as usize]);
        if postR == postL {
            return root;
        }
        let right_post_r = post[postR as usize - 1]; // 右子树的后续遍历的右边值 == 右子树前序遍历的左边值
        let right_post_r_index = (*hash_map.get(&right_post_r)?) as i32;
        let left_size = right_post_r_index - 1 - preL;
        root.as_mut()?.borrow_mut().left =
            Self::build_tree(post, postL, postL + left_size - 1, hash_map, preL + 1);
        root.as_mut()?.borrow_mut().right = Self::build_tree(
            post,
            left_size + postL,
            postR - 1,
            hash_map,
            left_size + preL + 1,
        );
        root
    }
    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}
// @lc code=end
