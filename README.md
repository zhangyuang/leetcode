![](https://static.leetcode-cn.com/cn-mono-assets/production/head/assets/logo-dark-cn.c42314a8.svg)

# leetcode

Solve questions in [leetcode](https://leetcode-cn.com/) by Rust  

## 前言

由于 Rust 写数据结构相关的资料特别少并且理解非常困难，所以专门建了个 Repo 用来记录 Rust 刷 leetcode 的解法并包含心得体会，欢迎star会长期稳定更新。  
https://github.com/zhangyuang/leetcode  

`注: 以下代码并没有刻意追求最优解，主要目的在于熟悉 Rust 语法以及使用可读性强便于理解的代码来解决问题。欢迎 Star 长期稳定保持更新。`

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

Rust 解决数据结构问题相比于其他语言十分的困难，就在于变量所有权的move(转移)与borrow(借用)。

##### 遍历链表

通常使用可变引用来遍历

```rust

let mut root = &mut head;
while let Some(node) = root {
  let next_node = &mut node.next;
  // 使用as_mut获取next_node的引用，使用&mut获取.next的引用。以此来获取root下一个节点的下一个节点的引用。直接使用unwrap会导致所有权的move
  let next_node_next = &mut next_node.as_mut().unwrap().next 
  // 这里面不能再直接使用head，因为head的所有权已经借给了root，在循环体中未归还
  // other code...
  root = &mut node.next;
}

```

##### 转移获取链表节点所有权

- take方法使用方式见[文档](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
- Copy以及Clone的区别可查看该[文章](https://zhuanlan.zhihu.com/p/21730929)  

``` rust

// 因为next为Box智能指针存储在堆上的节点，不具备Copy属性，无法直接从堆上转移数据否则会造成多次释放的问题。使用take方法将所有权转移出去，并且在原位置留下了None。
let next_node = node.next.take(); 

```

#### 解题代码

皆通过 leetcode 测试用例，可直接粘贴到 leetcode 编辑器中调试，刷题建议由浅入深，按知识点来刷,不要左右横跳。

#### Easy

简单难度的链表题

[回文链表|is_palindrome](./linkList/easy/is_palindrome/src/lib.rs)  
[反转链表|reverse_list](./linkList/easy/reverse_list/src/lib.rs)  
[链表的中间节点|middle_node](./linkList/easy/middle_node/src/lib.rs)  
[删除链表节点|delete_node](./linkList/easy/delete_node/src/lib.rs)  
[删除链表重复节点|delete_duplicates](./linkList/easy/delete_duplicates/src/lib.rs)

#### Medium

中等难度的链表题 

[两数相加|add_two_numbers](./linkList/medium/add_two_numbers/src/lib.rs)

### Dynamic Programing

动态规划

#### Rust 解动态规划题思路

主要思路与其他语言类似。还是通过寻找状态转移方程(递推关系)，通常要使用 vec 来保存之前的结果来提升性能

#### Easy

简单难度的动态规划题

[爬楼梯|climb_stairs](./dynamic-programing/easy/climb_stairs/src/lib.rs)  

