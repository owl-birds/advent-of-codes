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
impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut max_twin_sum: i32 = 0;
        let mut num_list: Vec<i32> = vec![];
        // println!("{}", head.as_ref().unwrap().val);

        let mut temp_node: Option<Box<ListNode>> = head;
        // let mut temp_node: &Option<Box<ListNode>> = &head;

        while temp_node.is_some(){
        // while let Some(node) = temp_node{
            num_list.push(temp_node.as_ref().unwrap().val);
            // num_list.push(node.val);
            temp_node = temp_node.unwrap().next;
            // temp_node = &node.next;
        }

        let mut left: usize = 0;
        let mut right: usize = num_list.len() - 1;
        while left < right{
            let twin_sum: i32 = num_list[left] + num_list[right];
            if max_twin_sum < twin_sum{max_twin_sum = twin_sum}
            left += 1;
            right -= 1;
        }

        max_twin_sum
    }
}