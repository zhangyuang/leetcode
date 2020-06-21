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
        let mut next_node_next = next_node.next.take();
        // next_node.next = node.next;
        // next_node_next = next_node.next; // 记录第三个节点
        //                                  // next_node.next = node;
        // node.next = head_next;
        root = &mut next_node_next;
        continue;
      }
    }
  }
  head
}
