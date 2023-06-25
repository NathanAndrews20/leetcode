/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows: Vec<Vec<i32>> = vec![];

        for _ in 0..num_rows {
            let mut row = vec![1];
            match rows.last() {
                Some(previous_row) => {
                    for window in previous_row.windows(2) {
                        row.push(window[0] + window[1]);
                    }
                    row.push(1);
                }
                None => (),
            };

            rows.push(row);
        }

        return rows;
    }
}
// @lc code=end
