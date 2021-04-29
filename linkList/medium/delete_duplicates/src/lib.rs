/*
 * @lc app=leetcode.cn id=82 lang=rust
 *
 * [82] 删除排序链表中的重复元素 II
 */
struct Solution {}
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
// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut phead = Some(Box::new(ListNode {
      val: -1,
      next: head,
    }));
    let mut root = &mut phead;
    while root.is_some() && root.as_mut()?.next.is_some() {
      let root_next = &mut root.as_mut()?.next;
      let root_next_ref = root_next.as_mut()?;
      let root_next_next = &mut root_next_ref.next;
      if root_next_next.is_some() {
        if root_next_ref.val == root_next_next.as_ref()?.val {
          let res = root_next_ref.val;
          let mut guard = root_next_next;
          while guard.is_some() {
            // 找到第一个值不等于重复值的节点
            if guard.as_ref()?.val != res {
              break;
            }
            guard = &mut guard.as_mut()?.next;
          }
          if guard.is_some() {
            root.as_mut()?.next = guard.take();
            continue;
          } else {
            root.as_mut()?.next = None;
          }
        }
      }
      root = &mut root.as_mut()?.next;
    }
    phead.as_mut()?.next.take()
  }
}
// @lc code=end
