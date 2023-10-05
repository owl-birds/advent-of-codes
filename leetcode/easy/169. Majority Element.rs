impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // A MAJOR ELEMENT
        // MOORE'S VOTING ALGORITHM
        //https://leetcode.com/problems/majority-element/solutions/3015428/c-moore-s-voting-algorithm-explained-in-super-simple-fast-way/
        // 1.This algorithm states that if any element is to be occuring more than n/2 times strictly, then I can help u
        // 2.Steps are quite simple :
        // 3.Initialize the possible predicted candidate for the answer to be 0, and mark its freq as 0;
        // 4.Now iterating over the nums array, if the element==candidate, then increase its count
        // 5.If element is different than candidate, do freq--
        // 6.And if freq becomes 0, then update candidate, So the candidate is updated and the older one hods no longer good choice for answer.
        // WHY??
        // Because if it was a majority element then it would have not been cancelled by the other minor elements ie. As we are doing freq-- when we dont get the same element
        // 7.The element stored in candidate is indeed our required answer!!
        // 8.(As it is guarrenteed that answer exists, so there is no need for further checking).
        // 9.Return ans

        // ASSUME MAJORITY ELEMENT EXIST
        // The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

        let mut num: i32 = nums[0];
        let mut freq: u32 = 0;

        for i in 0..nums.len() {
            if nums[i] != num {
                freq -= 1;
            } else {
                freq += 1;
            }
            if freq == 0 && i < nums.len() - 1 {
                freq = 0;
                num = nums[i + 1];
            }
        }

        if freq == 0 {
            return -1;
        }
        num
    }
}
