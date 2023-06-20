/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|n| *n != val);
        return nums.len() as i32;
    }
}
// @lc code=end
