/*
 * @lc app=leetcode.cn id=278 lang=rust
 *
 * [278] 第一个错误的版本
 */
struct Solution {}
impl Solution {
    fn isBadVersion(&self, n: i32) -> bool {
        if n == 1702766719 {
            return true;
        } else {
            return false;
        }
    }
}
// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    // 找左边界
    pub fn first_bad_version(&self, n: i32) -> i32 {
        return Self::search(&self, n);
    }
    fn search(&self, n: i32) -> i32 {
        if n == 1 && self.isBadVersion(1) {
            return 1;
        }
        let left = 0;
        let right = n as i64;
        return Self::binary_search(&self, n, left, right);
    }
    fn binary_search(&self, n: i32, left: i64, right: i64) -> i32 {
        if right < left {
            println!("{:?} {}", left, right);
            return left as i32;
        }
        let pivot = (right + left) / 2;
        if self.isBadVersion(pivot as i32) {
            // 如果找到了，则调整右边界
            return Self::binary_search(&self, n, left, pivot - 1);
        } else {
            // 没找到则调整左边界
            return Self::binary_search(&self, n, pivot + 1, right);
        }
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let so = Solution {};
        println!("{:?}", so.first_bad_version(2126753390));
    }
}
