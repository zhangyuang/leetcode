/*
 * @lc app=leetcode.cn id=304 lang=rust
 *
 * [304] 二维区域和检索 - 矩阵不可变
 */

// @lc code=start
struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    subscriptSum: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 {
            return NumMatrix {
                matrix,
                subscriptSum: vec![vec![]],
            };
        }
        let mut subscriptSum = vec![vec![0; matrix[0].len()]; matrix.len()];
        // 思路： 类似 https://leetcode-cn.com/problems/range-sum-query-immutable/
        // 分别计算每一行以每个元素为结束位置的和, 换成二维矩阵后把每一行的计算结果相加即可
        for i in 0..matrix.len() {
            let mut sum = 0;
            for j in 0..matrix[0].len() {
                sum += matrix[i][j];
                subscriptSum[i][j] = sum
            }
            sum = 0;
        }
        NumMatrix {
            matrix,
            subscriptSum,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let cols = (col2 - col1) as usize;
        let rows = (row2 - row1) as usize;
        let mut sum = 0;
        let mut row = row1 as usize;
        let col = col1 as usize;
        for i in 0..=rows {
            if col == 0 {
                sum += self.subscriptSum[row][col + cols]
            } else {
                sum += self.subscriptSum[row][col + cols] - self.subscriptSum[(row)][col - 1];
            }

            row += 1;
        }
        sum
    }
}

// /**
//  * Your NumMatrix object will be instantiated and called as such:
//  * let obj = NumMatrix::new(matrix);
//  * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
//  */
// @lc code=end
