struct Solution {}
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
impl Solution {
  pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head
  }
}

fn sort(left: Option<Box<ListNode>>, right: Option<Box<ListNode>>) {}
