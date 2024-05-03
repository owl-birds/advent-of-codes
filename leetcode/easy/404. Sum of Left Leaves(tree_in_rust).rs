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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // match &root {
        //     Some(node) => {},
        //     None => {return 0}
        // }
        let mut sum: i32 = 0;
        // let mut queue: VecDeque<TreeNodeOpt> = VecDeque::new();
        // let mut is_left: VecDeque<bool> = VecDeque::new();
        // match &root {
        //     Some(node) => {
        //         let test1 = node.borrow();
        //         println!("{:?}", test1.val);
        //     },
        //     None => {}
        // }
        // println!("{:?}", root.unwrap().borrow().val);
        // match &root {
        //     Some(node) => {
        //         println!("{:?}", node.borrow().val)
        //     },
        //     None => {}
        // }
        // // TEST
        // let mut test1: i32 = 0;
        // println!("test:{}", test1);
        // Self::test(&mut test1);
        // println!("test:{}", test1);
        // // TEST

        match &root {
            Some(node) => {
                let mut temp = node.borrow();
                if !temp.left.is_none() {
                    Self::traverse(&temp.left, true, &mut sum);
                }
                if !temp.right.is_none() {
                    Self::traverse(&temp.right, false, &mut sum);
                }
            }
            None => {}
        }

        sum
    }
    pub fn traverse(node: &TreeNodeOpt, is_left: bool, sum: &mut i32) {
        match node {
            Some(n) => {
                let temp_node = n.borrow();
                if is_left && temp_node.left.is_none() && temp_node.right.is_none() {
                    *sum += temp_node.val;
                }
                if !temp_node.left.is_none() {
                    Self::traverse(&temp_node.left, true, sum);
                }
                if !temp_node.right.is_none() {
                    Self::traverse(&temp_node.right, false, sum);
                }
            }
            None => {}
        }
    }
    pub fn test(the_num: &mut i32) {
        *the_num += 10;
    }
}
