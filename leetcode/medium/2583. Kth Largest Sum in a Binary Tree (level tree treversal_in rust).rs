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
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
type TreeNodeOpt = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut queue: VecDeque<TreeNodeOpt> = VecDeque::new();
        let mut level: VecDeque<usize> = VecDeque::new();
        let mut tree: Vec<i64> = vec![];
        let mut sum: i64 = 0;
        // let mut node: TreeNodeOpt = None;
        // match root {
        //     Some(n) => {
        //         // node = n.borrow();
        //         // let temp = n.borrow();
        //         println!("{:?}", n);
        //     },
        //     None => {}
        // }
        if let Some(ref n) = root {
            // println!("{:?}",n);
            // let temp = Rc::clone(n);
            let temp = Some(Rc::clone(n));
            // Rc::clone() --> MAKES A CLONE OF THE Rc POINTER
            // this creates another pointer to the same allocation,
            // increasing the strong reference count
            queue.push_back(temp);
            level.push_back(0);
            // // println!("{:?}",temp);
            // println!("{:?}",node);
        }
        while queue.len() > 0 {
            let temp = queue.pop_front().unwrap();
            let l = level.pop_front().unwrap();
            if tree.len() as usize != l + 1 {
                tree.push(0);
            }
            // println!("{}-{}", queue.len(),l);
            match &temp {
                Some(n) => {
                    // let n_borrow = n.as_ref().unwrap().try_borrow().is_ok();
                    // println!("{:?}", n_borrow);
                    // let n_borrow = n.as_ref().unwrap().try_borrow();
                    // println!("{:?}", n_borrow.unwrap().val);
                    // // println!("{:?}", n_borrow.unwrap());
                    // let n_borrow = n.as_ref().unwrap().try_borrow();
                    // // println!("{:?}", n_borrow.unwrap().left);
                    // // println!("::::::{:?}", n);
                    // let n_borrow = n.as_ref().unwrap().try_borrow().unwrap();
                    let n_borrow = n.as_ref().try_borrow().unwrap();
                    // tree[l].push(n_borrow.val);
                    tree[l] += n_borrow.val as i64;
                    // println!("{:?}", tree);
                    // println!("{:?}", n_borrow.left);
                    if let Some(ref n_left) = n_borrow.left {
                        queue.push_back(Some(Rc::clone(n_left)));
                        level.push_back(l + 1);
                    }
                    if let Some(ref n_right) = n_borrow.right {
                        queue.push_back(Some(Rc::clone(n_right)));
                        level.push_back(l + 1);
                    }
                }
                None => {}
            }
            // println!("{}-{}\n{:?}", queue.len(),l,tree);
            // break;
        }
        tree.sort_by(|a, b| b.cmp(&a));
        // println!("{:?}",tree);
        // println!("{:?}",node);
        // println!("{:?}",queue[0]);
        // println!("{:?}",root);
        if (tree.len() as i32) < k {
            return -1;
        }
        tree[k as usize - 1]
    }
}
