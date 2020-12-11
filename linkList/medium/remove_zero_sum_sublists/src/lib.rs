/*
 * @lc app=leetcode.cn id=1171 lang=rust
 *
 * [1171] 从链表中删去总和值为零的连续节点
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}
struct Solution {}
// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::HashMap;

impl Solution {
  pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
      return head;
    }
    let phead = Some(Box::new(ListNode { val: 0, next: head }));
    let mut hash_map = HashMap::new();
    let mut head_clone = phead.clone();
    let mut head_refer = &phead;
    let mut sum = 0;
    while head_refer.is_some() {
      sum = sum + &head_refer.as_ref()?.val;
      hash_map.insert(sum, head_refer.clone());
      head_refer = &head_refer.as_ref()?.next;
    }
    let mut head_refer = &mut head_clone;
    sum = 0;
    while head_refer.is_some() {
      sum += &head_refer.as_mut()?.val;
      let node = hash_map.get_mut(&sum)?;
      head_refer.as_mut()?.next = node.as_mut()?.next.take();
      head_refer = &mut head_refer.as_mut()?.next;
    }
    head_clone?.next
  }
  // pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  //     // 本题转换成找一个正数数列的和为链表的和
  //     // 写完这种思路才发现题目要求的是连续结点-。-那还降低难度了，本解法可用于不连续节点的情况
  //     if head.is_none() {
  //         return head;
  //     }
  //     let mut sum = 0;
  //     let mut head_refer = &head;
  //     let mut v = vec![];
  //     let mut stack = vec![];
  //     while head_refer.is_some() {
  //         sum += head_refer.as_ref()?.val;
  //         if head_refer.as_ref()?.val > 0 {
  //             v.push(head_refer.as_ref()?.val);
  //         }
  //         head_refer = &head_refer.as_ref()?.next;
  //     }
  //     let n = v.len() - 1;
  //     let mut res = Self::get_sum(&v, sum, n, &mut stack);
  //     res.reverse(); // 得到结果后将vec反转得到的顺序就是最终的结果在链表中的顺序
  //     head_refer = &head;
  //     let mut n = 0;
  //     let mut head_clone = head.clone();
  //     while head_refer.is_some() {
  //         // 将不在最终链表中的节点删除
  //         if &head_refer.as_ref()?.val != &res[n] {
  //             head_clone = Self::delete_node(head_clone, head_refer);
  //         } else {
  //             n += 1;
  //         }
  //         head_refer = &head_refer.as_ref()?.next;
  //     }
  //     head_clone
  // }
  // fn get_sum(v: &Vec<i32>, sum: i32, n: usize, mut stack: &mut Vec<i32>) -> Vec<i32> {
  //     // println!("{:?}", sum);
  //     if n <= 0 {
  //         return vec![];
  //     }
  //     if sum == 0 {
  //         return stack.clone();
  //     }
  //     stack.push(v[n]); // 选择第n个数
  //     let res1 = Self::get_sum(v, sum - v[n], n - 1, &mut stack);
  //     if res1.len() != 0 {
  //         return res1; // 则从剩下的n-1个数中继续选择
  //     }
  //     stack.pop(); // 不选择第n个数
  //     let res2 = Self::get_sum(v, sum, n - 1, &mut stack);
  //     if res2.len() != 0 {
  //         return res2; // 则从剩下的n-1个数中继续选择
  //     }
  //     return vec![];
  // }
  // fn delete_node(
  //     head: Option<Box<ListNode>>,
  //     target: &Option<Box<ListNode>>,
  // ) -> Option<Box<ListNode>> {
  //     let mut phead = Some(Box::new(ListNode { val: 1, next: head }));
  //     let mut root = &mut phead;
  //     while root.as_mut()?.next.is_some() {
  //         if &root.as_mut()?.next == target {
  //             let target_next = &target.as_ref()?.next;
  //             root.as_mut()?.next = target_next.clone();
  //             break;
  //         }
  //         root = &mut root.as_mut()?.next;
  //     }
  //     phead?.next
  // }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let mut res = vec![];
    // println!("{:?}", Solution::get_sum(&vec![1, 2, 3, 1], 4, 3, &mut res));
  }
}
