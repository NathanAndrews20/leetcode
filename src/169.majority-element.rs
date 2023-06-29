/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */


use crate::util::solution::Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut frequency_table: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            match frequency_table.get_key_value(&num) {
                Some((key, value)) => frequency_table.insert(*key, *value + 1),
                None => frequency_table.insert(num, 1),
            };
        }

        let mut majority_element = -1;
        let mut max_frequency = -1;
        for (num, frequency) in frequency_table {
            if frequency > max_frequency {
                max_frequency = frequency;
                majority_element = num;
            }
        }

        return majority_element;
    }
}
// @lc code=end
