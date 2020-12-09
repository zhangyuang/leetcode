/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第N个节点
 */
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut slow = &head;
        let mut fast = &head;
        let mut i = 1;
        while i < n {
            // 快指针先走n步
            fast = &fast.as_ref()?.next;
            i += 1;
        }
        while fast.as_ref()?.next.is_some() {
            fast = &fast.as_ref()?.next;
            slow = &slow.as_ref()?.next;
        }
        // 这里必须clone一下因为head已经borrow了，没办法再move
        head = delete_node(head.clone(), slow);
        head
    }
}

fn delete_node(
    mut head: Option<Box<ListNode>>,
    target: &Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut phead = Some(Box::new(ListNode { val: 1, next: head }));
    let mut root = &mut phead;
    while root.as_mut()?.next.is_some() {
        if &root.as_mut()?.next == target {
            let target_next = &target.as_ref()?.next;
            root.as_mut()?.next = target_next.clone();
            break;
        }
        root = &mut root.as_mut()?.next;
    }
    phead?.next
}
// @lc code=end
