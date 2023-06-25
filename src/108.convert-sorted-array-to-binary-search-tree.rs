/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return sorted_array_to_bst_helper(&nums);
    }
}

fn sorted_array_to_bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }

    if nums.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
    }

    let middle_index = nums.len() / 2;
    let (left_nums, right_nums) = nums.split_at(middle_index);
    let (middle_element, right_nums) = right_nums.split_first()?;
    let mut node = TreeNode::new(*middle_element);

    let left = sorted_array_to_bst_helper(left_nums);
    let right = sorted_array_to_bst_helper(right_nums);

    node.left = left;
    node.right = right;

    return Some(Rc::new(RefCell::new(node)));
}
// @lc code=end
