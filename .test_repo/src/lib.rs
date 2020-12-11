/*
 * @lc app=leetcode.cn id=1019 lang=rust
 *
 * [1019] 链表中的下一个更大节点
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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        if head.is_none() {
            return vec![];
        }
        let mut v = vec![];
        let mut head_refer_slow = &head;
        let mut head_refer_fast = &head_refer_slow.as_ref().unwrap().next;
        let mut head_refer_slow_val = head_refer_slow.as_ref().unwrap().val;
        if head_refer_fast.is_none() {
            // 针对只有一个元素的情况
            return vec![0];
        }
        while head_refer_slow.is_some() {
            if head_refer_fast.is_none() {
                // 找到最后一个元素还没找到则 push 0
                // 慢指针+1 快指针=慢指针+1
                v.push(0);
                head_refer_slow = &head_refer_slow.as_ref().unwrap().next;
                if head_refer_slow.is_none() {
                    // 慢指针走到最后一个元素
                    break;
                }
                head_refer_slow_val = head_refer_slow.as_ref().unwrap().val;
                head_refer_fast = &head_refer_slow.as_ref().unwrap().next;
                continue;
            }
            if head_refer_fast.as_ref().unwrap().val > head_refer_slow_val {
                // 找到符合要求的数加入结果
                // 慢指针+1 快指针=慢指针+1
                v.push(head_refer_fast.as_ref().unwrap().val);
                head_refer_slow = &head_refer_slow.as_ref().unwrap().next;
                head_refer_slow_val = head_refer_slow.as_ref().unwrap().val;
                head_refer_fast = &head_refer_slow.as_ref().unwrap().next;
            } else {
                // 没找到 快指针+1
                head_refer_fast = &head_refer_fast.as_ref().unwrap().next;
            }
        }
        v
    }
}
// @lc code=end
