/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

use crate::util::solution::Solution;
// @lc code=start

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                'I' => match chars.peek() {
                    Some(next_c) => match next_c {
                        'V' => {
                            sum += 4;
                            chars.next();
                        }
                        'X' => {
                            sum += 9;
                            chars.next();
                        }
                        _ => sum += 1,
                    },
                    None => sum += 1,
                },
                'X' => match chars.peek() {
                    Some(next_c) => match next_c {
                        'L' => {
                            sum += 40;
                            chars.next();
                        }
                        'C' => {
                            sum += 90;
                            chars.next();
                        }
                        _ => sum += 10,
                    },
                    None => sum += 10,
                },
                'C' => match chars.peek() {
                    Some(next_c) => match next_c {
                        'D' => {
                            sum += 400;
                            chars.next();
                        }
                        'M' => {
                            sum += 900;
                            chars.next();
                        }
                        _ => sum += 100,
                    },
                    None => sum += 100,
                },
                'V' => sum += 5,
                'L' => sum += 50,
                'D' => sum += 500,
                'M' => sum += 1000,
                _ => (),
            }
        }

        return sum;
    }
}
// @lc code=end
