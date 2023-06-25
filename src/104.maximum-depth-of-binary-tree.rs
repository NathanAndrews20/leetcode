/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */

use crate::util::solution::Solution;
use crate::util::tree_node::TreeNode;

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let left = root.borrow().left.clone();
                let right = root.borrow().right.clone();
                match (&left, &right) {
                    (None, None) => 1,
                    (None, Some(_)) => 1 + Self::max_depth(right),
                    (Some(_), None) => 1 + Self::max_depth(left),
                    (Some(_), Some(_)) => 1 + max(Self::max_depth(left), Self::max_depth(right)),
                }
            }
            None => 0,
        }
    }
}
// @lc code=end
