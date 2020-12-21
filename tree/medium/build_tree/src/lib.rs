/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    // 解法1，构建hashmap
    // 每次找到 中序序列中第一个在前序序列中出现的元素即为中序序列的根节点
    // pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     if preorder.len() == 0 {
    //         return None;
    //     }
    //     let mut hash_map = HashMap::new();
    //     for i in 0..preorder.len() {
    //         hash_map.insert(preorder[i], i);
    //     }
    //     let mut root = Self::create_node(preorder[0]); // 前序遍历第一个元素是根结点
    //     let index = inorder.iter().position(|&x| x == preorder[0])?;
    //     let left = &inorder[0..index];
    //     let right = &inorder[index + 1..inorder.len()];
    //     root.as_mut()?.borrow_mut().left = Self::create_tree(left, &preorder, &hash_map);
    //     root.as_mut()?.borrow_mut().right = Self::create_tree(right, &preorder, &hash_map);
    //     root
    // }
    // fn create_tree(
    //     nums: &[i32],
    //     preorder: &Vec<i32>,
    //     hash_map: &HashMap<i32, usize>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     if nums.len() == 0 {
    //         return None;
    //     }
    //     let mut index = 9999; // 找根节点
    //     for i in nums {
    //         if hash_map.get(i)? < &(index as usize) {
    //             index = *hash_map.get(i)?
    //         }
    //     }
    //     let mut root_index = nums.iter().position(|&x| x == preorder[index] as i32)?;
    //     let mut root = Self::create_node(nums[root_index]);
    //     root.as_mut().unwrap().borrow_mut().right =
    //         Self::create_tree(&nums[root_index + 1..nums.len()], &preorder, &hash_map);
    //     root.as_mut().unwrap().borrow_mut().left =
    //         Self::create_tree(&nums[0..root_index], &preorder, &hash_map);
    //     root
    // }
    // fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    //     Some(Rc::new(RefCell::new(TreeNode::new(val))))
    // }

    // 解法2，构建hashmap
    // 通过划分数组找到根节点
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let mut hash_map = HashMap::new();
        for i in 0..preorder.len() {
            hash_map.insert(inorder[i], i);
        }

        Self::create_tree(&preorder, 0, preorder.len() - 1, &hash_map, 0)
    }
    fn create_tree(
        preorder: &Vec<i32>,
        preL: usize,
        preR: usize,
        hash_map: &HashMap<i32, usize>,
        inoL: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preL > preR {
            return None;
        }
        let mut root = Self::create_node(preorder[preL]);
        let leftTreeLen = hash_map.get(&preorder[preL])? - inoL as usize;
        root.as_mut().unwrap().borrow_mut().left =
            Self::create_tree(preorder, preL + 1, preL + leftTreeLen, &hash_map, inoL);
        root.as_mut().unwrap().borrow_mut().right = Self::create_tree(
            preorder,
            preL + leftTreeLen + 1,
            preR,
            &hash_map,
            inoL + leftTreeLen + 1,
        );
        root
    }
    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::build_tree(vec![1, 2, 3], vec![3, 2, 1]);
    }
}
