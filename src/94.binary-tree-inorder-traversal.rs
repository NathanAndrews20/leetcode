/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
 */

use crate::util::solution::Solution;
use crate::util::tree_node::TreeNode;

use std::borrow::Borrow;
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut inorder_values = vec![];
        let mut current_node_option = root;
        loop {
            match current_node_option {
                Some(current_node) => {
                    current_node_option = current_node.borrow_mut().left.clone();
                    stack.push(current_node);
                }
                None => {
                    if let Some(inorder_node) = stack.pop() {
                        inorder_values.push(inorder_node.borrow_mut().val);
                        current_node_option = inorder_node.borrow_mut().right.clone();
                    } else {
                        break
                    }
                },
            }
        }

        return inorder_values;
    }
}
// @lc code=end
