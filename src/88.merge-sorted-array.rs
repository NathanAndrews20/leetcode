/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(nums1.len() - nums2.len());

        let mut index1 = nums1.len();
        while index1 > 0 && !nums2.is_empty() {
            if nums2[nums2.len() - 1] >= nums1[index1 - 1] {
                let num2 = match nums2.pop() {
                    Some(v) => v,
                    None => return,
                };
                nums1.insert(index1, num2);
            } else {
                index1 -= 1;
            }
        }

        while let Some(num2) = nums2.pop() {
            nums1.insert(0, num2)
        }
    }
}
// @lc code=end
