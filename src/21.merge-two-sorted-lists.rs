/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

use crate::util::{list_node::ListNode, solution::Solution};

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(list2)) => Some(list2),
            (Some(list1), None) => Some(list1),
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    let mut merged_head = ListNode::new(list1.val);
                    merged_head.next = Self::merge_two_lists(list1.next, Some(list2));
                    return Some(Box::new(merged_head));
                } else {
                    let mut merged_head = ListNode::new(list2.val);
                    merged_head.next = Self::merge_two_lists(Some(list1), list2.next);
                    return Some(Box::new(merged_head));
                }
            }
        }
    }
}
// @lc code=end
