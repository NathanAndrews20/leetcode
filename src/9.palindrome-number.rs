/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome_p9(x: i32) -> bool {
        let x_string = x.to_string();
        let x_string_reversed: String = x_string.chars().rev().collect();
        return x_string == x_string_reversed;
    }
}
// @lc code=end
