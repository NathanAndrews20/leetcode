/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut head_node) => {
                head_node.next = delete_duplicates_helper(head_node.val, head_node.next);
                return Some(head_node);
            }
            None => None,
        }
    }
}

fn delete_duplicates_helper(
    target_value: i32,
    node_option: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match node_option {
        Some(mut node) => {
            if node.val == target_value {
                return delete_duplicates_helper(target_value, node.next);
            }
            node.next = delete_duplicates_helper(node.val, node.next);
            return Some(node);
        }
        None => None,
    }
}
// @lc code=end
