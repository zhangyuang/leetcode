struct Solution {}

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
  pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
    if s.len() != t.len() {
      return false;
    }
    let s_arr: Vec<&str> = s.split("").collect(); // string转vector
    let mut s_arr_new = vec![];
    for i in 1..s_arr.len() - 1 {
      let item = s_arr[i];
      s_arr_new.push(item.bytes().next().unwrap())
    }

    let sign_arr: Vec<&str> = t.split("").collect();
    let mut i = 1;
    let mut sign_arr_new = vec![];
    while i < sign_arr.len() - 1 {
      let item = sign_arr[i];

      if item.bytes().next().unwrap() >= s_arr_new[i - 1] {
        sign_arr_new.push(item.bytes().next().unwrap() - s_arr_new[i - 1])
      } else {
        sign_arr_new.push(26 - (s_arr_new[i - 1] - item.bytes().next().unwrap()))
      }
      i += 1;
    }
    println!("{:?}", sign_arr_new);

    let mut hash_map: HashMap<u8, i32> = HashMap::new();
    let mut res: i32 = 0;
    for item in sign_arr_new {
      if item != 0 {
        let get_item = hash_map.get_mut(&item);
        if get_item.is_none() {
          hash_map.insert(item, 1);
          res = max(res, item as i32);
        } else {
          let get_item_val = get_item.unwrap().clone();
          let item_new = item as i32 + 26 * get_item_val;
          hash_map.insert(item, get_item_val + 1);
          res = max(res as i32, item_new);
        }
      }
    }
    println!("{:?}", res);
    return res <= k;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    Solution::can_convert_string("input".to_string(), "ouput".to_string(), 10);
  }
}
