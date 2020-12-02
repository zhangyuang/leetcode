/*
 * @lc app=leetcode.cn id=225 lang=rust
 *
 * [225] 用队列实现栈
 */

// @lc code=start
use std::collections::VecDeque;
#[derive(Debug)]
struct MyStack {
  queue1: VecDeque<i32>,
  queue2: VecDeque<i32>,
  len: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
  /** Initialize your data structure here. */
  fn new() -> Self {
    MyStack {
      queue1: VecDeque::new(),
      queue2: VecDeque::new(),
      len: 0,
    }
  }
  /** Push element x onto stack. */
  fn push(&mut self, x: i32) {
    self.len = self.len + 1;
    let (_, not_empty_queue) = self.get_queue();
    not_empty_queue.push_back(x);
  }
  /** Removes the element on top of the stack and returns that element. */
  fn pop(&mut self) -> i32 {
    self.len = self.len - 1;
    let (empty_queue, not_empty_queue) = self.get_queue();
    while not_empty_queue.len() != 1 {
      let node = not_empty_queue.pop_front().unwrap();
      empty_queue.push_back(node)
    }
    return not_empty_queue.pop_front().unwrap();
  }

  /** Get the top element. */
  fn top(&mut self) -> i32 {
    let (_, not_empty_queue) = self.get_queue();

    return not_empty_queue[not_empty_queue.len() - 1];
  }

  /** Returns whether the stack is empty. */
  fn empty(&self) -> bool {
    self.len == 0
  }
  fn get_queue(&mut self) -> (&mut VecDeque<i32>, &mut VecDeque<i32>) {
    let not_empty_queue;
    let empty_queue;
    if self.queue1.len() == 0 {
      not_empty_queue = &mut self.queue2;
      empty_queue = &mut self.queue1
    } else {
      not_empty_queue = &mut self.queue1;
      empty_queue = &mut self.queue2
    }
    (empty_queue, not_empty_queue)
  }
}

// /**
//  * Your MyStack object will be instantiated and called as such:
//  * let obj = MyStack::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: bool = obj.empty();
//  */
// @lc code=end
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let mut queue = MyStack::new();
    queue.push(1);
    queue.push(2);
    // println!("{:?}", queue);
    // queue.push(3);
    // queue.push(4);
    println!("{:?}", queue.pop());
    // queue.push(5);
    println!("{:?}", queue.top());
    // println!("{:?}", queue.pop());
    // println!("{:?}", queue.pop());
    // println!("{:?}", queue.pop());
  }
}
