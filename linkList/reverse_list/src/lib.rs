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

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  if head.is_none() {
    return None;
  }
  let mut prev: Option<Box<ListNode>> = None;
  let mut curr: Option<Box<ListNode>> = head;
  while curr.is_some() {
    let mut node = curr.take().unwrap();
    curr = node.next;
    node.next = prev;
    prev = Some(node);
  }
  prev
}
