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

fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut len: f64 = 0.0;
  let mut root = &mut head;
  while root.is_some() {
    len += 1.0;
    root = &mut root.as_mut().unwrap().next;
  }
  let middle_index = (len / 2.0).floor();
  let mut i = 0.0;
  root = &mut head;
  while i < middle_index {
    i += 1.0;
    root = &mut root.as_mut().unwrap().next;
  }
  root.take()
}
