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
                let val = hash_map.get_mut(&i[0]).unwrap();
                val.push(i[1]);
            }
        }
        let mut res = true;
        let hash_map_clone = hash_map.clone();
        for (key, val) in hash_map.iter_mut() {
            *val = add_path(&mut vec![*key], val.clone(), &hash_map_clone);
        }
        for (key, val) in hash_map.iter() {
            if val.contains(key) {
                res = false;
                break;
            }
        }
        res
    }
}

fn add_path(key: &mut Vec<i32>, val_vec: Vec<i32>, hash_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut foo = val_vec.clone();
    for val in val_vec {
        match hash_map.get(&val) {
            Some(next_val) => {
                if check_key_repect(&foo, key) {
                    break;
                }
                key.push(val);
                foo.extend(add_path(key, next_val.to_owned(), hash_map));
            }
            None => {}
        }
    }
    foo
}
fn check_key_repect(foo: &Vec<i32>, key: &mut Vec<i32>) -> bool {
    for i in key {
        if foo.contains(&i) {
            return true;
        }
    }
    return false;
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::can_finish(3, vec![vec![0, 2], vec![1, 2], vec![2, 0]]);
    }
}
