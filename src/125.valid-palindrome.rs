/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;
        while left < right {
            if !chars[left].is_alphanumeric() {
                left += 1;
            } else if !chars[right].is_alphanumeric() {
                right -= 1;
            } else {
                if !chars[left].eq_ignore_ascii_case(&chars[right]) {
                    return false;
                }
                left += 1;
                right -= 1;
            }
        }
        return true;
    }
}

// @lc code=end
