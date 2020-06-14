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

use std::collections::HashMap;

impl Solution {
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let root = &head;
    if root.is_none() {
      return None;
    }
    let mut hashMap = HashMap::new();

    while let Some(node) = root {
      if hashMap.get(&node.val).is_none() {
        hashMap.insert(&node.val, true);
      } else {
        delete_node(&head, &node);
      }
    }
    head
  }
}

fn delete_node(head: &Option<Box<ListNode>>, target: &Box<ListNode>) {
  let root = head;
  while let Some(mut node) = root {
    if node.next.as_ref().unwrap().val == target.val {
      // 当前节点的下一个节点等于目标节点
      let next_node = node.next.unwrap().next;
      node.next = next_node;
      break;
    }
    root = &node.next;
  }
}
