/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
 */
struct Solution {}
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
  pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref()?.next.is_none() {
      return head;
    }
    let mut root = &mut head;

    while root.is_some() && root.as_mut()?.next.is_some() {
      let mut node = root.as_mut().unwrap();
      let next_node = &mut node.next;
      if next_node.as_ref()?.val == node.val {
        node.next = next_node.as_mut()?.next.take();
      } else {
        root = &mut root.as_mut()?.next;
      }
    }
    head
  }
}
// @lc code=end
