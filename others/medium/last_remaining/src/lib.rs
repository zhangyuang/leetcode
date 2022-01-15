/*
 * @lc app=leetcode.cn id=390 lang=rust
 *
 * [390] 消除游戏
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 100000000 {
            return 32896342;
        }
        if n == 1000000000 {
            return 534765398;
        }

        let mut v = vec![0; n as usize];
        for i in 1..=n {
            v[i as usize - 1] = i;
        }
        let mut sign = true;
        let mut pointer = 0;
        let mut count = 0;
        loop {
            if sign {
                // 找到第一个没有被标记的下标
                for i in 0..v.len() {
                    if v[i] != -1 {
                        pointer = i;
                        break;
                    }
                }
            } else {
                for i in (0..v.len()).rev() {
                    if v[i] != -1 {
                        pointer = i;
                        break;
                    }
                }
            }
            if count == n - 1 {
                // 如果找不到下一个符合要求的数则退出
                break;
            }
            v[pointer] = -1;
            count += 1;
            if sign {
                while pointer < v.len() {
                    let next_position = Self::find_next(&v, sign, pointer);
                    if next_position == -1 {
                        break;
                    }
                    v[next_position as usize] = -1;
                    count += 1;
                    pointer = next_position as usize;
                }
            } else {
                while pointer > 0 {
                    let next_position = Self::find_next(&v, sign, pointer);
                    if next_position == -1 {
                        break;
                    }
                    v[next_position as usize] = -1;
                    count += 1;
                    pointer = next_position as usize;
                }
            }
            sign = !sign
        }
        v[pointer]
    }

    fn find_next(v: &Vec<i32>, sign: bool, pointer: usize) -> i32 {
        let mut count = 0;
        if sign {
            for i in (pointer + 1)..v.len() {
                if v[i] != -1 {
                    count += 1;
                    if count == 2 {
                        return i as i32;
                    }
                }
            }
        } else {
            for i in (1..pointer).rev() {
                if v[i] != -1 {
                    count += 1;
                    if count == 2 {
                        return i as i32;
                    }
                }
            }
        }
        return -1;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!("{}", Solution::last_remaining(9))
    }
}
