// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
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
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut s: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        let re = Regex::new(r"(-*)(\d+)").unwrap();
        for cap in re.captures_iter(traversal.as_str()) {
            let d = cap[1].len();
            s.truncate(d);
        
            let v = cap[2].parse().unwrap();
            let n: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(v)));
        
            if let Some(p) = s.last() {
                let mut p = p.borrow_mut();
                if p.left.is_none() {
                    p.left = Some(n.clone());
                } else {
                    p.right = Some(n.clone());
                }
            }

            s.push(n);
        }

        Some(s[0].clone())
    }
}
