/*
 * @lc app=leetcode.cn id=106 lang=rust
 *
 * [106] 从中序与后序遍历序列构造二叉树
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 后序遍历最后一个元素是根节点
        // 建议在纸上画个图便于确定具体计算逻辑
        // 中序遍历: 左子树|根接点|右子树
        // 后序遍历: 左子树|右子树|根结点
        //          postL       postR
        let mut hash_map = HashMap::new();
        for i in 0..inorder.len() {
            hash_map.insert(inorder[i], i);
        }
        Self::create_tree(&postorder, 0, (postorder.len() - 1) as i32, &hash_map, 0)
    }
    fn create_tree(
        postorder: &Vec<i32>,
        postL: i32, // 后序遍历的左边起始下标
        postR: i32, // 后序遍历的右边起始下标
        hash_map: &HashMap<i32, usize>,
        inL: i32, // 中序遍历的左边起始下标
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if postL > postR {
            return None;
        }
        let mut root = Self::create_node(postorder[postR as usize]);
        let left_index = *hash_map.get(&postorder[postR as usize])?;
        let left_tree_size = left_index - inL as usize;
        root.as_mut()?.borrow_mut().left = Self::create_tree(
            postorder,
            postL,
            postL + left_tree_size as i32 - 1,
            &hash_map,
            inL,
        );
        root.as_mut()?.borrow_mut().right = Self::create_tree(
            postorder,
            postL + left_tree_size as i32,
            postR - 1 as i32,
            &hash_map,
            left_index as i32 + 1,
        );
        root
    }
    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}
// @lc code=end
