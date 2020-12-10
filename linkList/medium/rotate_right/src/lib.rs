/*
 * @lc app=leetcode.cn id=61 lang=rust
 *
 * [61] 旋转链表
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
impl Solution {
  pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // rust 链表无法成环，拆成两个链表再连接
    if head.is_none() {
      return head;
    }
    let mut len = 0;
    let mut head_refer = &head;
    while head_refer.as_ref().is_some() {
      // 记录链表长度
      len += 1;
      head_refer = &head_refer.as_ref()?.next;
    }
    let offset = k % len;
    if offset == 0 {
      return head;
    }
    let mut head_mut_refer = &mut head;
    while len - offset != 1 {
      len -= 1;
      head_mut_refer = &mut head_mut_refer.as_mut()?.next;
    }
    // right = 4->5  head = 1->2->3
    let mut right = head_mut_refer.as_mut()?.next.take();
    let mut right_mut_refer = &mut right;
    while right_mut_refer.as_ref()?.next.is_some() {
      // 找到 right 的尾部
      right_mut_refer = &mut right_mut_refer.as_mut()?.next;
    }
    right_mut_refer.as_mut()?.next = head;
    right
  }
}
// @lc code=end
