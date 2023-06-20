/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();

        let mut right = 0;
        let mut len = 0;
        while right < haystack.len() {
            if needle[len] == haystack[right] {
                len += 1;
            } else {
                loop {
                    if haystack[right - len] == needle[0] {
                        right = right - len;
                        len = 0;
                        break;
                    }

                    if len == 0 {
                        break;
                    }

                    len -= 1;
                }
            }

            right += 1;

            if len == needle.len() {
                return (right - len) as i32;
            }
        }

        return -1;
    }
}
// @lc code=end
