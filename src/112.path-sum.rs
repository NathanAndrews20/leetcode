/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(root) => {
                let left = &root.borrow().left;
                let right = &root.borrow().right;
                let val = root.borrow().val;

                match (left, right) {
                    (None, None) => target_sum == val,
                    (Some(_), None) => Self::has_path_sum(left.clone(), target_sum - val),
                    (None, Some(_)) => Self::has_path_sum(right.clone(), target_sum - val),
                    (Some(_), Some(_)) => {
                        Self::has_path_sum(left.clone(), target_sum - val)
                            || Self::has_path_sum(right.clone(), target_sum - val)
                    }
                }
            }
            None => false,
        }
    }
}
// @lc code=end
