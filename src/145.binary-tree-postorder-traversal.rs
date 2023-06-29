/*
 * @lc app=leetcode id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut postorder_values = vec![];
        postorder_traversal(&root, &mut postorder_values);
        return postorder_values;
    }
}

fn postorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, postorder_values: &mut Vec<i32>) {
    match root {
        Some(node) => {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            let val = node.borrow().val;
            match (left, right) {
                (None, None) => postorder_values.push(val),
                (None, Some(_)) => {
                    postorder_traversal(right, postorder_values);
                    postorder_values.push(val);
                }
                (Some(_), None) => {
                    postorder_traversal(left, postorder_values);
                    postorder_values.push(val);
                }
                (Some(_), Some(_)) => {
                    postorder_traversal(left, postorder_values);
                    postorder_traversal(right, postorder_values);
                    postorder_values.push(val);
                }
            }
        }
        None => (),
    }
}
// @lc code=end
