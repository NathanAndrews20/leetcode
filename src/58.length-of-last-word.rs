/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let last_word = match s.split_whitespace().last() {
            Some(word) => word,
            None => return -1,
        };
        return last_word.len() as i32;
    }
}
// @lc code=end
