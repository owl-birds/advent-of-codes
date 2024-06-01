// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
type OptBoxList = Option<Box<ListNode>>;
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: OptBoxList = Some(Box::new(ListNode::new(0)));
        let mut temp: Vec<i32> = vec![];
        let mut temp_res: Vec<i32> = vec![];
        Self::traverse_list(&head, &mut temp);

        let mut idx: usize = temp.len();
        let mut max: i32 = temp[idx - 1];
        temp_res.push(max);
        idx -= 1;
        while idx > 0 {
            idx -= 1;
            if temp[idx] >= max {
                max = temp[idx];
                temp_res.push(max);
            }
        }
        temp_res.reverse();
        let mut temp: &mut OptBoxList = &mut res;
        for i in 0..temp_res.len() {
            match temp {
                Some(n) => {
                    n.as_mut().val = temp_res[i];
                }
                None => {}
            }
            if i < temp_res.len() - 1 {
                match temp {
                    Some(n) => {
                        n.as_mut().next = Some(Box::new(ListNode::new(0)));
                        temp = &mut n.as_mut().next;
                    }
                    None => {}
                }
            }
        }

        // println!("{:?}\n{:?}\n{:?}", head, temp, temp_res);
        res
    }
    pub fn traverse_list(node: &OptBoxList, list_vec: &mut Vec<i32>) {
        match node {
            Some(n) => {
                // let mut temp: &Option<&Box<ListNode>> = &Some(n);
                // match new_node {
                //     Some(new) => {
                //         new.as_mut().val = n.as_ref().val;
                //     },
                //     None => {}
                // }
                // // println!("{:?}", new_node);
                // while temp.is_none() &&
                list_vec.push(n.as_ref().val);
                Self::traverse_list(&n.as_ref().next, list_vec);
            }
            None => {
                return;
            }
        }
    }
}
