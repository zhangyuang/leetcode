/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */

struct Solution {}
// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in prerequisites {
            if hash_map.get(&i[0]).is_none() {
                hash_map.insert(i[0], vec![i[1]]);
            } else {
                let val = hash_map.get(&i[0]).unwrap();
                let mut foo = val.clone();
                foo.push(i[1]);
                hash_map.insert(i[0], foo);
            }
        }
        println!("{:?}", hash_map);
        let mut res = true;
        let mut hash_map_new: HashMap<i32, Vec<i32>> = HashMap::new();

        for (key, val) in hash_map.iter() {
            let mut foo = val.clone();
            for i in val {
                if hash_map.get(i).is_some() {
                    foo.extend(hash_map.get(i).unwrap());
                }
            }
            hash_map_new.insert(*key, foo);
        }
        println!("{:?}", hash_map_new);

        println!("{:?}", res);
        res
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::can_finish(2, vec![vec![1, 0], vec![2, 1], vec![0, 2]]);
    }
}
