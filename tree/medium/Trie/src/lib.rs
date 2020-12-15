/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
#[derive(Default, Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let word_arr: Vec<char> = word.chars().collect();
        let mut root = self;
        for i in 0..word_arr.len() {
            let position_index = word_arr[i] as u8 - b'a';
            root = root.children[position_index as usize].get_or_insert(Box::new(Self::new()));
        }
        root.is_word = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let word_arr: Vec<char> = word.chars().collect();
        let mut root = self;
        for i in 0..word_arr.len() {
            let position_index = word_arr[i] as u8 - b'a';
            if root.children[position_index as usize].is_some() {
                root = &root.children[position_index as usize].as_ref().unwrap();
            } else {
                return false;
            }
        }
        return root.is_word;
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let word_arr: Vec<char> = prefix.chars().collect();
        let mut root = self;
        for i in 0..word_arr.len() {
            let position_index = word_arr[i] as u8 - b'a';
            if root.children[position_index as usize].is_some() {
                root = &root.children[position_index as usize].as_ref().unwrap();
            } else {
                return false;
            }
        }
        true
    }
}

// /**
//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);
//  */
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
    }
}
