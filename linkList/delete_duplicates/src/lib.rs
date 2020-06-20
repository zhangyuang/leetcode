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
    if head.is_none() {
      return None;
    }

    let mut head = Some(Box::new(ListNode {
      val: 99999, // 防止与用例value冲突
      next: head,
    }));
    let mut head_clone = head.clone();
    let mut hashMap = HashMap::new();
    let mut root = &mut head;
    while let Some(node) = root {
      if hashMap.get(&node.val).is_none() {
        hashMap.insert(node.val, true);
      } else {
        head_clone = delete_node(head_clone, node.val); // 这里不能直接用head，因为所有权暂时借给了root
      }
      root = &mut node.next;
    }
    head_clone.unwrap().next
  }
}

fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
  let mut head = Some(Box::new(ListNode {
    val: 99999,
    next: head,
  }));
  let mut root = &mut head;
  while let Some(node) = root {
    let next_node = &mut node.next;
    match next_node {
      None => break,
      Some(next_node) => {
        if next_node.val == val {
          // 当前节点的下一个节点等于目标节点
          node.next = next_node.next.take();
          break;
        }
      }
    }
    root = &mut node.next;
  }

  head.unwrap().next
}
