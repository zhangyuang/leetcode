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
fn main() {
  let mut head = Some(Box::new(ListNode { next: None, val: 0 }));

  print!("{:?}", head.unwrap().next);
}
