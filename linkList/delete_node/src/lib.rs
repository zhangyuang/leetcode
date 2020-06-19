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

fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
  let mut head = Some(Box::new(ListNode { val: 1, next: head }));

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
