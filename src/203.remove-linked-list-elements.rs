/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            Some(node) => match node.val == val {
                true => Self::remove_elements(node.next, val),
                false => Some(Box::new(ListNode {
                    val: node.val,
                    next: Self::remove_elements(node.next, val),
                })),
            },
            None => None,
        }
    }
}
// @lc code=end
