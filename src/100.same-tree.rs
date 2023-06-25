/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                if p.borrow().val == q.borrow().val {
                    return Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                        && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone());
                } else {
                    return false;
                }
            }
            (None, None) => true,
            _ => false,
        }
    }
}
// @lc code=end
