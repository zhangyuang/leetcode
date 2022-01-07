/*
 * @lc app=leetcode.cn id=2022 lang=rust
 *
 * [2022] 将一维数组转变成二维数组
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let sum = n * m;
        if original.len() != sum as usize {
            return vec![];
        };
        let (m, n) = (m as usize, n as usize);
        let mut v = vec![];
        let mut foo = vec![];
        for i in 0..original.len() {
            foo.push(original[i]);
            if foo.len() == n {
                v.push(foo);
                foo = vec![]
            }
        }
        v
    }
}
// @lc code=end
