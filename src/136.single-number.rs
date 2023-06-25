/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 */

use crate::util::solution::Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut frequency_table = HashMap::new();
        for n in nums {
            match frequency_table.get_key_value(&n) {
                Some((_, frequency)) => frequency_table.insert(n, frequency + 1),
                None => frequency_table.insert(n, 1),
            };
        }

        for (n, frequency) in frequency_table {
            if frequency == 1 {
                return n;
            }
        }

        return -1;
    }
}
// @lc code=end
