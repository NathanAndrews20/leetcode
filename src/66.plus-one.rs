/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits.clone();
        digits.reverse();

        digits[0] = digits[0] + 1;

        for i in 0..digits.len() {
            let digit = digits[i];
            if digit >= 10 {
                digits[i] = digit % 10;
                match digits.get(i + 1) {
                    Some(next_digit) => {
                        digits[i + 1] = next_digit + 1;
                    }
                    None => {
                        digits.push(1);
                        break;
                    }
                }
            }
        }

        digits.reverse();
        return digits;
    }
}
// @lc code=end
