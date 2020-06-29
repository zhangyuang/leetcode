struct Solution {}

impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    let nsize = n as usize;
    let mut v: Vec<i32> = vec![1, 2];
    let mut i: usize = 2;
    while i < nsize {
      v.push(v.get(i - 1).unwrap() + v.get(i - 2).unwrap());
      i += 1;
    }
    *v.get(nsize - 1).unwrap() // 这里要返回i32而不是&i32所以使用解引用
  }
}
