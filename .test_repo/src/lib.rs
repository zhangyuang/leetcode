// 定义栈的数据结构，请在该类型中实现一个能够得到栈的最小元素的 min 函数在该栈中，调用 min、push 及 pop 的时间复杂度都是 O(1)。

//

// 示例:

// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.min();   --> 返回 -3.
// minStack.pop();
// minStack.top();      --> 返回 0.
// minStack.min();   --> 返回 -2.

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/bao-han-minhan-shu-de-zhan-lcof
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
struct MinStack {
    top: Option<i32>, // 栈顶指针
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        return MinStack {
            stack: vec![],
            top: None,
            min_stack: vec![],
        };
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        self.top = Some(x);
        if self.min_stack.len() == 0 || x <= self.min_stack[self.min_stack.len() - 1] {
            self.min_stack.push(x)
        }
    }

    fn pop(&mut self) {
        if self.stack.pop().unwrap() == self.min_stack[self.min_stack.len() - 1] {
            self.min_stack.pop();
        }
        if self.stack.len() != 0 {
            self.top = Some(self.stack[self.stack.len() - 1]); // 栈顶指针 -1
        } else {
            self.top = None
        }
    }

    fn top(&self) -> i32 {
        self.top.unwrap()
    }

    fn min(&self) -> i32 {
        self.min_stack[self.min_stack.len() - 1]
    }
}

// /**
//  * Your MinStack object will be instantiated and called as such:
//  * let obj = MinStack::new();
//  * obj.push(x);
//  * obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: i32 = obj.min();
//  */
