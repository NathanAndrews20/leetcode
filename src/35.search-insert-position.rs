/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let end = nums.len() - 1;
        return Self::binarySearch(nums, target, 0, end as i32);
    }

    pub fn binarySearch(nums: Vec<i32>, target: i32, start: i32, end: i32) -> i32 {
        if end <= start {
            if target <= nums[start as usize] {
                return start;
            } else {
                return start + 1;
            }
        }

        

        let mid = (end - start / 2) + start;
        let mid_value = nums[mid as usize];
        if mid_value == target {
            return mid;
        } else if mid_value < target {
            return Self::binarySearch(nums, target, mid, end);
        } else {
            return Self::binarySearch(nums, target, start, mid - 1);
        }
    }
}
// @lc code=end
