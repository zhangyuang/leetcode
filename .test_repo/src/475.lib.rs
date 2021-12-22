/*
 * @lc app=leetcode.cn id=475 lang=rust
 *
 * [475] 供暖器
 */

// @lc code=start
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut res = -1;
        houses.iter().for_each(|h| {
            heaters.iter().for_each(|k| {
                res = res.max(h-k)
            })
        })   
    }
}
// @lc code=end

