/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut previous_row_option: Option<Vec<i32>> = None;

        for _ in 0..row_index + 1 {
            let mut row = vec![1];
            match previous_row_option {
                Some(previous_row) => {
                    for window in previous_row.windows(2) {
                        row.push(window[0] + window[1]);
                    }
                    row.push(1);
                }
                None => (),
            };
            previous_row_option = Some(row);
        }

        return previous_row_option.unwrap_or(vec![]);
    }
}
// @lc code=end
