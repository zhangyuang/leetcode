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
  pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // 由于 Rust safe情况不能拥有两个可变的引用，所以使用clone，否则可使用 unsafe
    if head.is_none() {
      return head;
    }
    let mut head_clone = head.clone();
    let mut slow = &mut head_clone;
    let mut fast = &mut head;
    let mut i = 1;
    while i < n {
      // 快指针先走n步
      fast = &mut fast.as_mut().unwrap().next;
      i += 1;
    }
    while fast.as_mut().unwrap().next.is_some() {
      fast = &mut fast.as_mut().unwrap().next;
      slow = &mut slow.as_mut().unwrap().next;
    }
    let mut slow_clone = slow.clone();
    head_clone = delete_node(head_clone, &mut slow_clone);
    head_clone
  }
}

fn delete_node(
  head: Option<Box<ListNode>>,
  target: &mut Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut head = Some(Box::new(ListNode { val: 1, next: head }));
  let mut root = &mut head;
  while let Some(node) = root {
    let next_node = &mut node.next;
    match next_node {
      None => break,
      Some(next_node) => {
        if next_node == target.as_mut().unwrap() {
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
