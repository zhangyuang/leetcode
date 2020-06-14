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

impl Solution {
  pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
      return true;
    }
    let mut vec = Vec::new();
    let mut root = &head;

    while let Some(node) = root {
      if node.next.is_none() {
        vec.push(node);
        break;
      }
      vec.push(node);
      root = &node.next;
    }
    let mut root = &head;
    while let Some(node) = root {
      if node.next.is_none() {
        let mut top = vec.pop().unwrap();
        if top.val != node.val {
          return false;
        }
        break;
      }
      let mut top = vec.pop().unwrap();
      if top.val != node.val {
        return false;
      }
      root = &node.next;
    }
    true
  }
}
