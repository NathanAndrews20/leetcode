/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */

use crate::util::solution::Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut num = n;
        loop {
            // calculate the sum of the squares of the digits of num;
            let mut sum_of_squares_of_digits = 0;
            while num > 0 {
                let last_digit = num % 10;
                sum_of_squares_of_digits += last_digit * last_digit;
                num /= 10;
            }

            if sum_of_squares_of_digits == 1 {
                return true;
            }

            if set.contains(&sum_of_squares_of_digits) {
                return false;
            }

            set.insert(sum_of_squares_of_digits);
            num = sum_of_squares_of_digits;
        }
    }
}
// @lc code=end
