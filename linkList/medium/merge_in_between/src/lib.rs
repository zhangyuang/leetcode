/*
 * @lc app=leetcode.cn id=1669 lang=rust
 *
 * [1669] 合并两个链表
 */
//Definition for singly-linked list.
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
  pub fn merge_in_between(
    mut list1: Option<Box<ListNode>>,
    a: i32,
    b: i32,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut head_clone = list1.clone();
    let mut head = &mut list1;
    let mut left = 1;
    let mut right = 1;
    while right != b + 2 {
      head_clone = head_clone?.next;
      right += 1;
    }
    while left != a {
      head = &mut head.as_mut()?.next;
      left += 1;
    }
    head.as_mut()?.next = list2;
    while head.as_mut()?.next.is_some() {
      head = &mut head.as_mut()?.next;
    }
    head.as_mut()?.next = head_clone;
    list1
  }
}
// @lc code=end
