/*
 * @lc app=leetcode.cn id=221 lang=rust
 *
 * [221] 最大正方形
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
        let column = matrix[0].len();
        let mut dp = vec![vec![0; column]; row];
        let mut res = 0;
        for i in 0..row {
            for j in 0..column {
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1])) + 1
                    }
                }
                res = res.max(dp[i][j])
            }
        }
        res * res
    }
    // pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    //     let mut res = 0;
    //     let row = matrix.len();
    //     let column = matrix[0].len();
    //     for i in 0..row {
    //         for j in 0..column {
    //             if matrix[i][j] != '1' {
    //                 continue;
    //             }
    //             let mut right = j + 1;
    //             let mut down = i + 1;

    //             while right < column && down < row {
    //                 if Solution::check(&matrix, (i, j, right, down)) {
    //                     res = res.max((down - i + 1) * (right - j + 1));
    //                     right = right + 1;
    //                     down = down + 1;
    //                 } else {
    //                     break;
    //                 }
    //             }
    //             res = res.max(1)
    //         }
    //     }
    //     res as i32
    // }
    // fn check(matrix: &Vec<Vec<char>>, coords: (usize, usize, usize, usize)) -> bool {
    //     let (top, left, right, down) = coords;
    //     if matrix[down][right] != '1' {
    //         return false;
    //     }
    //     for i in left..right {
    //         if matrix[down][i] != '1' {
    //             return false;
    //         }
    //     }
    //     for i in top..down {
    //         if matrix[i][right] != '1' {
    //             return false;
    //         }
    //     }
    //     true
    // }
}
// @lc code=end
#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn name() {
        println!(
            "{}",
            Solution::maximal_square(vec![
                vec!['1', '1', '0'],
                vec!['1', '1', '1'],
                vec!['1', '1', '1'],
                vec!['1', '1', '0']
            ])
        )
    }
}
