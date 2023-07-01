/*
 * @lc app=leetcode id=205 lang=rust
 *
 * [205] Isomorphic Strings
 */

use crate::util::solution::Solution;

// @lc code=start
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();

        for (s_char, t_char) in s.chars().zip(t.chars()) {
            match map.get(&s_char) {
                Some(should_be_t_char) => {
                    if t_char != *should_be_t_char {
                        return false;
                    }
                }
                None => drop(map.insert(s_char, t_char)),
            }
        }

        let unique_keys: HashSet<&char> = HashSet::from_iter(map.keys());
        let unique_values: HashSet<&char> = HashSet::from_iter(map.values());

        return unique_keys.len() == unique_values.len();
    }
}
// @lc code=end
