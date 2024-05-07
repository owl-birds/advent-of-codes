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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: OptBoxList = Some(Box::new(ListNode::new(0)));
        let mut temp: Vec<i32> = vec![];
        Self::traverse_list(&head, &mut temp);
        if temp.len() == 0 {return None}
        
        // println!("{:?}", temp);
        // println!("{:?}", head);
        // let mut curr_num: i64 = 0;
        // let mut c = temp.len() as i32 - 1;
        // for i in 0..temp.len() {
        //     curr_num += (i64::pow(10, c as u32) * temp[i] as i64);
        //     c -= 1;
        // }
        // // println!("{}", curr_num);
        // curr_num *= 2;
        // // println!("{}", curr_num);
        // let mut temp = Self::num_to_vec(curr_num);
        let mut temp = Self::double(&temp);
        // println!("{:?}", temp);

        Self::add_to_node(&mut res, &temp, 0);
        res
    }
    pub fn add_to_node(node: &mut OptBoxList, nums: &Vec<i32>, idx: usize) {
        if idx == nums.len()-1 {
            match node {
                Some(n) => {
                    n.as_mut().val = nums[idx];
                }, 
                None => {}
            }
            return;
        }
        match node {
            Some(n) => {
                n.as_mut().val = nums[idx];
                n.as_mut().next = Some(Box::new(ListNode::new(0)));
                Self::add_to_node(&mut n.as_mut().next, nums, idx+1);
            },
            None => {}
        }
    }
    pub fn double(nums: &Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut idx: usize = nums.len();
        let mut is_carry: bool = false;

        while idx > 0 {
            idx -= 1;
            if is_carry {
                let curr = (2 * nums[idx]) + 1;
                if curr > 9 {
                    is_carry = true;
                } else {
                    is_carry = false;
                }
                res.push(curr%10);
            } else {
                let curr = 2 * (nums[idx]);
                if curr > 9 {
                    is_carry = true;
                } else {
                    is_carry = false;
                }
                res.push(curr%10);
            }
        }
        if is_carry {
            res.push(1);
        }
        res.reverse();

        res
    }
    pub fn num_to_vec(num: i64) -> Vec<i32> {
        if num == 0 {return vec![0]}
        let mut res: Vec<i32> = vec![];
        let mut temp: i64 = num;
        while temp > 0 {
            res.push((temp % 10) as i32);
            temp /= 10;
        }
        res.reverse();
        res
    }
    pub fn traverse_list(node: &OptBoxList, nums: &mut Vec<i32>) {
        match node {
            Some(n) => {
                nums.push(n.as_ref().val);
                Self::traverse_list(&n.as_ref().next, nums);
            },
            None => {
                return;
            }
        }
    }
}