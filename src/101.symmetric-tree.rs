/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => is_symmetric_helper(&root.borrow().left, &root.borrow().right),
            None => true,
        }
    }
}

fn is_symmetric_helper(
    node_a: &Option<Rc<RefCell<TreeNode>>>,
    node_b: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (node_a, node_b) {
        (Some(a), Some(b)) => {
            a.borrow().val == b.borrow().val
                && is_symmetric_helper(&a.borrow().left, &b.borrow().right)
                && is_symmetric_helper(&a.borrow().right, &b.borrow().left)
        }
        (None, None) => true,
        _ => false,
    }
}
// @lc code=end
