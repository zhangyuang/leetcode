/*
 * @lc app=leetcode.cn id=336 lang=rust
 *
 * [336] 回文对
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[i].len() == 0
                    || words[j].len() == 0
                    || &words[i][0..1] == &words[j][words[j].len() - 1..words[j].len()]
                {
                    let mut source = words[i].clone();
                    source.push_str(&words[j]);
                    let source_rev = rev(&source);
                    if source == source_rev {
                        res.push(vec![i as i32, j as i32])
                    }
                }
                if words[i].len() == 0
                    || words[j].len() == 0
                    || &words[j][0..1] == &words[i][words[i].len() - 1..words[i].len()]
                {
                    let mut source = words[j].clone();
                    source.push_str(&words[i]);
                    let source_rev = rev(&source);
                    if source == source_rev {
                        res.push(vec![j as i32, i as i32])
                    }
                }
            }
        }
        println!("{:?}", res);
        res
    }
}
fn rev(source: &String) -> String {
    source.chars().rev().collect::<String>()
}

// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        Solution::palindrome_pairs(vec![
            "bb".to_string(),
            "bababab".to_string(),
            "baab".to_string(),
            "abaabaa".to_string(),
            "aaba".to_string(),
            "".to_string(),
            "bbaa".to_string(),
            "aba".to_string(),
            "baa".to_string(),
            "b".to_string(),
        ]);
    }
}
