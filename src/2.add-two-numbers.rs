/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return add_two_numbers_with_carry(l1, l2, false);
    }
}

fn add_two_numbers_with_carry(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: bool,
) -> Option<Box<ListNode>> {
    let mut sum = if carry { 1 } else { 0 };
    match (l1, l2) {
        (None, None) => {
            if carry {
                Some(Box::new(ListNode { val: 1, next: None }))
            } else {
                None
            }
        }
        (None, Some(node)) | (Some(node), None) => {
            sum += node.val;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: add_two_numbers_with_carry(node.next, None, sum >= 10),
            }))
        }
        (Some(node1), Some(node2)) => {
            sum += node1.val + node2.val;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: add_two_numbers_with_carry(node1.next, node2.next, sum >= 10),
            }))
        }
    }
}
// @lc code=end
