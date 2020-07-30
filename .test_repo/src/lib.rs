/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root_index = 0;
        let mut root = if nums.len() % 2 != 0 {
            root_index = nums.len() / 2;
            create_node(nums[root_index])
        } else {
            root_index = nums.len() / 2 - 1;
            create_node(nums[root_index])
        };
        let mut node = root;
        let mut node_clone = node.clone();
        while root_index + 2 < nums.len() + 2 {
            // 代表右边有两个数字
            node.as_mut().unwrap().borrow_mut().right = create_node(nums[root_index + 2]);
            node.as_mut()
                .unwrap()
                .borrow_mut()
                .right
                .as_mut()
                .unwrap()
                .borrow_mut()
                .left = create_node(nums[root_index + 1]);
            root_index = root_index + 2;
            node = node.as_mut().unwrap().borrow_mut().right.clone();
        }
        println!("{:?}", node);
        // if root_index == nums.len() - 2 {
        //     node.as_mut().unwrap().borrow_mut().right = create_node(nums[root_index + 1])
        // }
        root
    }
}
fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}
// @lc code=end
