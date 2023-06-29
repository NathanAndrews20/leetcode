/*
 * @lc app=leetcode id=191 lang=rust
 *
 * [191] Number of 1 Bits
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut count = 0;
        let mut n = n;
        for _ in 0..32 {
            count += n & 1;
            n >>= 1;
        }
        return count as i32;
    }
}
// @lc code=end
