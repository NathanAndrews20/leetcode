/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
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
use std::cmp::min;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let left = &root.borrow().left;
                let right = &root.borrow().right;
                match (&left, &right) {
                    (None, None) => 1,
                    (Some(_), None) => 1 + Self::min_depth(left.clone()),
                    (None, Some(_)) => 1 + Self::min_depth(right.clone()),
                    (Some(_), Some(_)) => {
                        1 + min(
                            Self::min_depth(left.clone()),
                            Self::min_depth(right.clone()),
                        )
                    }
                }
            }
            None => 0,
        }
    }
}

// @lc code=end
