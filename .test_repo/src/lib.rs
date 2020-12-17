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
use std::rc::Rc;
impl Solution {
    // 解法1，中序遍历后构建 avl 树
    // pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     let in_order_traversal = Self::in_order_traversal(&root)?;
    //     Self::build_balance_bst_from_arr(in_order_traversal)
    // }
    // fn build_balance_bst_from_arr(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     if nums.len() == 0 {
    //         return None;
    //     }
    //     let mut root_index = 0;
    //     let mut root = if nums.len() % 2 != 0 {
    //         root_index = nums.len() / 2;
    //         Self::create_node(nums[root_index])
    //     } else {
    //         root_index = nums.len() / 2 - 1;
    //         Self::create_node(nums[root_index])
    //     };
    //     root.as_mut()?.borrow_mut().right =
    //         Self::build_balance_bst_from_arr(nums[root_index + 1..nums.len()].to_vec());
    //     root.as_mut()?.borrow_mut().left =
    //         Self::build_balance_bst_from_arr(nums[0..root_index].to_vec());
    //     root
    // }
    // fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    //     Some(Rc::new(RefCell::new(TreeNode::new(val))))
    // }
    // fn in_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Vec<i32>> {
    //     let mut v = vec![];
    //     let mut stack = vec![];
    //     let mut node = root.clone();
    //     while stack.len() != 0 || node.is_some() {
    //         while node.is_some() {
    //             let res = node?;
    //             node = res.borrow_mut().left.take();
    //             stack.push(res);
    //         }
    //         node = stack.pop();
    //         v.push(node.as_ref()?.borrow_mut().val);
    //         node = node?.borrow_mut().right.take();
    //     }
    //     Some(v)
    // }
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root
    }
}
// @lc code=end
