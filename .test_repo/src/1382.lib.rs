/*
 * @lc app=leetcode.cn id=1382 lang=rust
 *
 * [1382] 将二叉搜索树变平衡
 */
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
    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
    pub fn balance_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut new_root = None;
        let mut stack = vec![];
        while stack.len() != 0 || !root.is_none() {
            // 先序遍历原树，构建新的 avl 树
            while !root.is_none() {
                // 一直添加左子树直到为空
                let node = root.unwrap();
                new_root = Self::insert(new_root, node.borrow().val);
                root = node.borrow_mut().left.take();
                stack.push(node);
            }
            // 从栈中弹出，取节点的右子树
            root = stack.pop();
            root = root.unwrap().borrow_mut().right.take();
        }
        new_root
    }
    fn insert(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Self::create_node(val);
        }
        let mut root_borrow_mut = root.as_mut()?.borrow_mut();
        let left = root_borrow_mut.left.take();
        let right = root_borrow_mut.right.take();
        let root_val = root_borrow_mut.val;
        if root_val > val {
            root_borrow_mut.left = Self::insert(left, val)
        } else {
            root_borrow_mut.right = Self::insert(right, val)
        }
        root
    }
    fn get_balance_factor(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 获取节点平衡因子
        if root.is_none() {
            return 0;
        }
        let root_borrow = root.as_ref().unwrap().borrow();
        let left = &root_borrow.left;
        let right = &root_borrow.right;
        return (Self::get_height(left) - Self::get_height(right)).abs();
    }
    fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 获取节点的高度
        if root.is_none() {
            return 0;
        }
        let root_borrow = root.as_ref().unwrap().borrow();
        let left = &root_borrow.left;
        let right = &root_borrow.right;
        return max(Self::get_height(left), Self::get_height(right)) + 1;
    }
}
// @lc code=end
