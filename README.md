![](https://static.leetcode-cn.com/cn-mono-assets/production/head/assets/logo-dark-cn.c42314a8.svg)

# leetcode

Solve questions in [leetcode](https://leetcode-cn.com/) by Rust

## 分类

[链表(linkedList)](#linkedList)
[二叉树(binaryTree)]()
[动态规划(dynamic programing)]()

### linkedList

链表

#### Rust 解链表题思路

> Go 程序员已经下班
Cpp 程序员还在打断点
Rust 程序员还在编译

Rust 解决数据结构问题相比于其他语言十分的困难，就在于变量所有权的borrow与引用。

##### 遍历链表

通常使用可变引用来遍历

```rust

let mut root = &mut head;
while let Some(node) = root {
  let next_node = &mut node.next;
  // 这里面不能再直接使用head，因为head的所有权已经借给了root，在循环体中未归还
  // other code...
  root = &mut node.next;
}

```

##### 同时获取链表多个所有权

``` rust

// take()将node打断，这样node只有一个值，返回值是除n1节点外的剩余节点
// node.next是Option<T>
// take()是用默认值替换原有的值，所以n1.next就变为None
next_node = node.next.take(); 

```

#### 解题代码

皆通过 `leetcode` 测试用例，可直接粘贴到 `leetcode` 编辑器中调试

[回文链表|is_palindrome](./linkList/is_palindrome/src/lib.rs)  
[删除链表节点|delete_node](./linkList/delete_node/src/lib.rs)  
[删除链表重复节点|delete_duplicates](./linkList/delete_duplicates/src/lib.rs)