// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/description/
use std::{cell::RefCell, rc::Rc};

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
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn recover(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                n.val = val;
                recover(&n.left, 2 * val + 1);
                recover(&n.right, 2 * val + 2);
            }
        }

        recover(&root, 0);
        Self{ root }
    }

    fn find(&self, target: i32) -> bool {
        fn search(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
            if let Some(n) = node {
                let n = n.borrow();
                if n.val == target {
                    return true;
                }
                return search(&n.left, target) || search(&n.right, target);
            }
            false
        }
        
        search(&self.root, target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
