/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

use crate::util::solution::Solution;
// @lc code=start

use std::{cmp::min, collections::HashMap, iter::FromIterator};

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first_word = strs[0].as_str();
        let first_word_map: HashMap<usize, char> =
            HashMap::from_iter(first_word.chars().enumerate());
        let mut prefix_length = first_word.len();

        for str in strs.iter().skip(1) {
            if str.len() < prefix_length {
                prefix_length = str.len()
            }

            for (i, c) in str.chars().enumerate() {
                match first_word_map.get(&i) {
                    Some(first_word_char) => {
                        if *first_word_char != c {
                            prefix_length = min(prefix_length, i)
                        }
                    }
                    None => prefix_length = min(prefix_length, i),
                }
            }
        }

        let answer = &first_word[0..prefix_length];
        return String::from(answer);
    }
}
// @lc code=end
