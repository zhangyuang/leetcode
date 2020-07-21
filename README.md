![](https://static.leetcode-cn.com/cn-mono-assets/production/head/assets/logo-dark-cn.c42314a8.svg)

# leetcode

Solve questions in [leetcode](https://leetcode-cn.com/) by Rust

## 前言

由于 Rust 写数据结构相关的资料特别少并且理解非常困难，所以专门建了个 Repo 用来记录 Rust 刷 leetcode 的解法并包含心得体会，欢迎 Star✨ 会长期稳定更新。  
努力写出最容易理解的 Rust 代码。
[https://github.com/zhangyuang/leetcode](https://github.com/zhangyuang/leetcode)

`注: 以下代码并没有刻意追求最优解，主要目的在于熟悉 Rust 语法以及使用可读性强便于理解的代码来解决问题。欢迎 Star✨ 长期稳定保持更新。`

## 相关资料

[官方 API 文档](https://doc.rust-lang.org/std/index.html)  
[Rust 程序设计语言中文版](https://kaisery.github.io/trpl-zh-cn/)

## 分类

[链表(linkedList)](#linkedList)  
[二叉树(binaryTree)](#Tree)  
[动态规划(dynamic programing)](#DynamicPrograming)  
[HOT100🔥](#hot100)

### linkedList

链表

#### Rust 解链表题思路

> Go 程序员已经下班
> Cpp 程序员还在打断点
> Rust 程序员还在编译

Rust 解决数据结构问题相比于其他语言十分的困难，就在于变量所有权的 move(转移)与 borrow(借用)。

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

- take 方法使用方式见[文档](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
- Copy 以及 Clone 的区别可查看该[文章](https://zhuanlan.zhihu.com/p/21730929)

```rust

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
[两两交换链表中的节点|swap_pairs](./linkList/medium/swap_pairs/src/lib.rs)  
[删除链表的倒数第 N 个节点|remove_nth_from_end](./linkList/medium/remove_nth_from_end/src/lib.rs)

### Tree

树，二叉树

#### 解题思路

Rust 构造树需要使用 [Rc<T>引用计数智能指针](https://kaisery.github.io/trpl-zh-cn/ch15-04-rc.html)以及 [RefCell](https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html)，使得一个数据具有多个可变的所有者。因为一个子节点可能被多个父节点所共享。

#### Easy

简单难度的树题
二叉搜索树解题思路：中序遍历的结果是递增数组

[二叉树的层次遍历 II|level_order_bottom](./tree/easy/level_order_bottom/src/lib.rs)  
[二叉树的层平均值|average_of_levels](./tree/easy/average_of_levels/src/lib.rs)  
[相同的树|is_symmetric](./tree/easy/is_symmetric/src/lib.rs)  
[对称二叉树|is_same_tree](./tree/easy/is_same_tree/src/lib.rs)  
[平衡二叉树|is_balanced](./tree/easy/is_balanced/src/lib.rs)  
[二叉树的所有路径|binary_tree_paths](./tree/easy/binary_tree_paths/src/lib.rs)  
[二叉树的最小深度|min_depth](./tree/easy/min_depth/src/lib.rs)  
[左叶子之和|sum_of_left_leaves](./tree/easy/sum_of_left_leaves/src/lib.rs)  
[二叉搜索树中的众数|find_mode](./tree/easy/find_mode/src/lib.rs)  
[二叉搜索树中的搜索|search_bst](./tree/easy/search_bst/src/lib.rs)  
[二叉搜索树的第 k 大节点|kth_largest](./tree/easy/kth_largest/src/lib.rs)  
[二叉搜索树的范围和|range_sum_bst](./tree/easy/range_sum_bst/src/lib.rs)  
[二叉搜索树节点最小距离|min_diff_in_bst](./tree/easy/min_diff_in_bst/src/lib.rs)  
[把二叉搜索树转换为累加树|convert_bst](./tree/easy/convert_bst/src/lib.rs)

#### Medium

中等难度的树题

[二叉树前序遍历|preorder_traversal](./tree/medium/preorder_traversal/src/lib.rs)  
[二叉树中序遍历|inorder_traversal](./tree/medium/inorder_traversal/src/lib.rs)  
[二叉树层次遍历|level_order](./tree/medium/level_order/src/lib.rs)

#### Hard

[二叉树后序遍历|postorder_traversal](./tree/hard/postorder_traversal/src/lib.rs)

### DynamicPrograming

动态规划

#### Rust 解动态规划题思路

主要思路与其他语言类似。还是通过寻找状态转移方程(递推关系)，通常要使用 vec 来保存之前的结果来提升性能。
常用到的空间优化方式有滚动数组，来将二维数组压成一维或减少数组空间大小。大部分情况都是背包问题(01背包，完全背包，多重背包)问题的变种。
学习资料: [liweiwei leetcode 经典动规解析](https://leetcode-cn.com/problems/coin-change/solution/dong-tai-gui-hua-shi-yong-wan-quan-bei-bao-wen-ti-/)

#### Easy

简单难度的动态规划题

[爬楼梯|climb_stairs](./dynamic-programing/easy/climb_stairs/src/lib.rs)  
[三步问题|ways_to_step](./dynamic-programing/easy/ways_to_step/src/lib.rs)  
[连续数列|max_sub_array](./dynamic-programing/easy/max_sub_array/src/lib.rs)  
[按摩师|massage](./dynamic-programing/easy/massage/src/lib.rs)  
[打家劫舍|rob](./dynamic-programing/easy/rob/src/lib.rs)  
[使用最小花费爬楼梯|min_cost_climbing_stairs](./dynamic-programing/easy/min_cost_climbing_stairs/src/lib.rs)  
[买卖股票的最佳时机|max_profit](./dynamic-programing/easy/max_profit/src/lib.rs)  

### HOT100🔥

Hot100类型题

#### Easy

简单难度的HOT100题

[柠檬水找零|lemonade_change](./hot100/easy/lemonade_change/src/lib.rs)  

#### Medium

中等难度的HOT100题

[除自身以外数组的乘积|product_except_self](./hot100/medium/product_except_self/src/lib.rs)  
[分割等和子集|can_partition](./hot100/medium/can_partition/src/lib.rs)  
[全排列|permute](./hot100/medium/permute/src/lib.rs)  
[括号生成|generate_parenthesis](./hot100/medium/generate_parenthesis/src/lib.rs)  
[子集|subsets](./hot100/medium/subsets/src/lib.rs)  
[零钱兑换|coin_change](./hot100/medium/coin_change/src/lib.rs)  