/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut preorder_values = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            match node {
                Some(node) => {
                    preorder_values.push(node.borrow().val);
                    // push right before left so that left will be popped off
                    // the stack and processed first
                    stack.push(node.borrow().right.clone());
                    stack.push(node.borrow().left.clone());
                }
                None => (),
            }
        }

        return preorder_values;
    }
}
// @lc code=end
