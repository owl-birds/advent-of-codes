#[derive(Debug)]
enum IsFound {
    TRUE,
    FALSE,
}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // let mut is_found: IsFound = IsFound::FALSE;
        let mut is_found: [bool; 1] = [false];
        Self::jump(&nums, 0, &mut is_found);
        // println!("{:?}", is_found);
        is_found[0]
    }
    pub fn jump(nums: &Vec<i32>, curr_position: usize, is_found: &mut [bool; 1]) {
        // match is_found {
        //     IsFound::TRUE => {return;},
        //     IsFound::FALSE => {}
        // }
        // println!("{} -- {}", curr_position, is_found[0]);
        if is_found[0] {
            return;
        }
        if curr_position == (nums.len() - 1) {
            // is_found = &mut IsFound::TRUE;
            is_found[0] = true;
            return;
        }
        if curr_position >= nums.len() {
            return;
        }

        let mut j: usize = nums[curr_position] as usize;
        while j >= 1 {
            Self::jump(nums, curr_position + j, is_found);
            j -= 1;
        }

        // for j in 1..(nums[curr_position]+1) as usize {
        //     Self::jump(nums, curr_position + j, is_found);
        // }
    }
}
