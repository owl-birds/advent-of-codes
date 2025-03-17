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
use std::collections::VecDeque;
use std::rc::Rc;

type TreeNodeOpt = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue: VecDeque<TreeNodeOpt> = VecDeque::new();
        let mut level: VecDeque<usize> = VecDeque::new();
        let mut result: Vec<i32> = vec![];

        if let Some(ref n) = root {
            let temp = Some(Rc::clone(n));
            queue.push_back(temp);
            level.push_back(0);
            while queue.len() > 0 {
                let temp_node = queue.pop_front().unwrap();
                let lev = level.pop_front().unwrap();
                // println!("{:?}", temp_node);
                match &temp_node {
                    Some(n) => {
                        // println!("{:?}", n);
                        let n_borrow = n.as_ref().try_borrow().unwrap();
                        // println!("{:?}", n_borrow);
                        if result.len() != lev + 1 {
                            result.push(n_borrow.val);
                        } else {
                            if result[lev] < n_borrow.val {
                                result[lev] = n_borrow.val;
                            }
                        }
                        if let Some(ref left) = n_borrow.left {
                            queue.push_back(Some(Rc::clone(left)));
                            level.push_back(lev + 1);
                        }
                        if let Some(ref right) = n_borrow.right {
                            queue.push_back(Some(Rc::clone(right)));
                            level.push_back(lev + 1);
                        }
                    }
                    None => {}
                }
            }
        }

        result
    }
}
