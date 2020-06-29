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
    let mut l1 = head;
    let mut result = ListNode::new(0);
    let mut res = &mut result;
    while let Some(mut l1_node) = l1 {
      l1 = l1_node.next.take(); // 使用take获取所有权,l1 = 2->3->4, l1_node = 1 此时l1_node.next=None
      if let Some(mut l2_node) = l1 {
        l1 = l2_node.next.take(); // l1 = 3->4, l2 = 2, 此时l2_node.next=None
        l2_node.next = Some(l1_node); // l2 = 2->1
        res.next = Some(l2_node); // res = 0->2->1,
        res = res.next.as_mut().unwrap().next.as_mut().unwrap(); // res = 1, res指向末尾
      } else {
        // 走到这里说明总的节点个数为奇数个, l1为最后一个节点，没有下一个节点。直接将l1添加到末尾
        res.next = Some(l1_node);
        res = res.next.as_mut().unwrap();
      }
    }
    result.next
  }
}
