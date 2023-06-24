/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 2;

        if n == a {
            return a;
        }

        if n == b {
            return b;
        }

        for _ in 2..n {
            let tmp = b;
            b = b + a;
            a = tmp;
        }

        return b
    }
}
// @lc code=end
