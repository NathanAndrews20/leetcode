/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (is_balanced, _) = is_balanced_helper(&root);
        return is_balanced;
    }
}

fn is_balanced_helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
    match root {
        Some(root) => {
            let left = &root.borrow().left;
            let right = &root.borrow().right;
            match (&left, &right) {
                (None, None) => (true, 1),
                (Some(_), None) => {
                    let (left_is_balanced, left_depth) = is_balanced_helper(left);
                    return (left_is_balanced && left_depth < 2, left_depth + 1);
                }
                (None, Some(_)) => {
                    let (right_is_balanced, right_depth) = is_balanced_helper(right);
                    return (right_is_balanced && right_depth < 2, right_depth + 1);
                }
                (Some(_), Some(_)) => {
                    let (left_is_balanced, left_depth) = is_balanced_helper(left);
                    let (right_is_balanced, right_depth) = is_balanced_helper(right);

                    return (
                        left_is_balanced
                            && right_is_balanced
                            && (left_depth - right_depth).abs() < 2,
                        1 + max(left_depth, right_depth),
                    );
                }
            }
        }
        None => (true, 0),
    }
}
// @lc code=end
