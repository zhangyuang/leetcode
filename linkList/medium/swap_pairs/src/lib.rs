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

// 给定 1->2->3->4, 你应该返回 2->1->4->3.
impl Solution {
  pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
      return head;
    }
    let mut l1 = &mut head;
    while l1.is_some() {
      let mut node = l1.as_mut().unwrap();
      let mut l2 = l1.as_mut().unwrap().next.take();
      if l2.is_none() {
        break;
      }
      let mut v = swap_node(l1.take(), l2);
      println!("{:?}", v[1]);
      l1 = &mut v[0];
    }
    None
  }
}

fn swap_node(
  mut l1: Option<Box<ListNode>>,
  mut l2: Option<Box<ListNode>>,
) -> Vec<Option<Box<ListNode>>> {
  match l2.as_mut().unwrap().next.take() {
    None => vec![l1, l2],
    Some(node) => {
      // 2->1->3->4
      // l1调用take后剩下None
      l2.as_mut().unwrap().next = l1.take(); // l2指向l1
      l1 = Some(node); // l1指向原l2的下一个节点，即直接跳到第三个节点
      vec![l1, l2]
    }
  }
}
