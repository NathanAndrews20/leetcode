/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

use crate::util::solution::Solution;

// @lc code=start
use std::{cmp::max, collections::HashSet};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut length = 0;
        let mut max_length = 0;
        let mut left_index = 0;
        let mut right_index = 0;

        let mut set: HashSet<char> = HashSet::new();

        while right_index < s.len() {
            if left_index == right_index || !set.contains(&s[right_index]) {
                set.insert(s[right_index]);
                right_index += 1;
                length += 1;
            } else {
                max_length = max(length, max_length);
                set.remove(&s[left_index]);
                left_index += 1;
                length -= 1;
            }
        }

        return max(max_length, length);
    }
}
// @lc code=end
