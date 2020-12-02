/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 * [232] 用栈实现队列
 */

// @lc code=start
struct MyQueue {
  stack1: Vec<i32>,
  stack2: Vec<i32>,
  len: i32,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl MyQueue {
  /** Initialize your data structure here. */
  fn new() -> Self {
    return MyQueue {
      stack1: vec![],
      stack2: vec![],
      len: 0,
    };
  }
  /** Push element x to the back of queue. */
  fn push(&mut self, x: i32) {
    self.stack1.push(x);
    self.len = self.len + 1;
  }
  /** Removes the element from in front of queue and returns that element. */
  fn pop(&mut self) -> i32 {
    self.len = self.len - 1;
    if self.stack2.len() == 0 {
      while self.stack1.len() != 0 {
        let element = self.stack1.pop().unwrap();
        self.stack2.push(element);
      }
    }
    return self.stack2.pop().unwrap();
  }

  /** Get the front element. */
  fn peek(&mut self) -> i32 {
    if self.stack1.len() == 0 {
      return self.stack2[self.stack2.len() - 1];
    } else {
      return self.stack1[0];
    }
  }

  /** Returns whether the queue is empty. */
  fn empty(&self) -> bool {
    self.len == 0
  }
}

// /**
//  * Your MyQueue object will be instantiated and called as such:
//  * let obj = MyQueue::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.peek();
//  * let ret_4: bool = obj.empty();
//  */
// @lc code=end
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    println!("{:?}", queue.pop());
    queue.push(5);
    println!("{:?}", queue.pop());
    println!("{:?}", queue.pop());
    println!("{:?}", queue.pop());
    println!("{:?}", queue.pop());
  }
}
