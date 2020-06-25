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
pub struct U256(pub [u64; 4]);

// Rust没有原生的bigint类型，所以在测试用例的极端场景会溢出。leetcode不支持引入第三方crate。所以最佳解决方案应该是按位相加然后进位。或者使用其他提供Bigint类型的语言如js,java
// 由于主要目的是熟悉Rust语法，这里就直接使用两数相加的解法来写了, 底部附上使用js的解法
fn add_two_numbers(
  mut l1: Option<Box<ListNode>>,
  mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let l1_sum = get_sum(&mut l1);
  let l2_sum = get_sum(&mut l2);
  let mut sum = l1_sum + l2_sum;
  let mut head = Some(Box::new(ListNode::new(0)));
  let mut root = &mut head;
  if sum == 0 {
    return head;
  }
  while (sum != 0) {
    let node = Some(Box::new(ListNode::new(sum.wrapping_rem(10) as i32)));
    root.as_mut().unwrap().next = node;
    root = &mut root.as_mut().unwrap().next;
    sum = sum.wrapping_div_euclid(10);
  }
  head.unwrap().next
}

fn get_sum(head: &mut Option<Box<ListNode>>) -> i128 {
  let mut root = head;
  let mut sum: i128 = 0;
  let mut bit: i128 = 1;
  while let Some(node) = root {
    sum += node.val as i128 * bit;
    bit *= 10;
    root = &mut node.next;
  }
  sum
}

// fix addTwoNumbers by js use Bigint
// var addTwoNumbers = function (l1, l2) {
//   let sum = getSum(l1) + getSum(l2)
//   let res = new ListNode(0)
//   if (sum === 0n) {
//     return res
//   }
//   let foo = res
//   while (sum != 0) {
//     const node = new ListNode(sum % 10n)
//     res.next = node
//     res = res.next
//     sum = sum / 10n
//   }
//   return foo.next
// };

// const getSum = list => {
//   let sum = 0n;
//   let bit = 1n;
//   while (list) {
//     sum += BigInt(list.val) * bit
//     bit *= 10n
//     list = list.next
//   }
//   return sum
// }
