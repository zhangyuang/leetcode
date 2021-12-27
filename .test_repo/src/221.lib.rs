/*
 * @lc app=leetcode.cn id=221 lang=rust
 *
 * [221] 最大正方形
 */

// @lc code=start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut val = 0;
        let row = matrix.len();
        let column = matrix[0].len();
        for i in 0..row {
            for j in 0..column {
                let current_val = matrix[i][j];
                let right = j + 1;
                let down = i + 1;
                while right < column && down < row {}
            }
        }
    }
}
// @lc code=end
