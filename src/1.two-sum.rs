/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result = vec![];
        for cur_index in 0..nums.len() {
            let cur_num = nums[cur_index];
            match map.get(&(target - cur_num)) {
                Some(other_index) => {
                    result = vec![*other_index, cur_index as i32];
                    break;
                }
                None => map.insert(cur_num, cur_index as i32),
            };
        }
        return result;
    }
}
// @lc code=end
