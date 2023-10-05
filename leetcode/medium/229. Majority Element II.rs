use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        // sorting ??? or hashmap, which one is better ?
        let limit = (nums.len() / 3) as u32;
        // println!("limit: {}", limit);

        let mut nums_freq: HashMap<i32, u32> = HashMap::new();

        for num in nums {
            if let Some(freq) = nums_freq.get_mut(&num) {
                *freq += 1;
            } else {
                nums_freq.insert(num, 1);
            }
        }

        let mut result: Vec<i32> = vec![];
        for (num, freq) in nums_freq.iter() {
            if *freq > limit {
                result.push(*num);
            }
        }

        result

        // a better solution, INTUITION
        // Intuitive idea:
        // Let's start with the basic case: 169. Majority Element
        // One can think of the algorithm as a battle process, whenever two different elements encountered, the battle occurs, and they both vanishes. The candidate is whoever that survives this battle process. The point here is that whenever a majority element vanishes (no matter it is the current candidate or not), there is at least (exactly) one other element dies with it. Hence, the majority element survives, since there are at least n/2 of them.

        // Now for this problem, the only difference is that there are two candidates, corresponding to two survivors that we pick. And whenever an element vanishes in a battle, there are at least (exactly) 2 other elements die with it. Hence, whenever an element with occurences > n/3 dies (no matter it is a current candidate or not), there are at least 2 other elements die with it. So it survives.

        // Rigorous proof this problem:
        // The above intuitive idea can easily be turned into a rigorous proof.
        // Let's prove by contradiction, suppose an element with occurences > n/3 is not a candidate after the algorithm finishes. Then, they must be all cancelled out at a certain point (either got counted down as a candidate element, or as a non-candidate to cause two candidates counted down), this corresponds to all of them vanishes in the battle. But whenever a cancellation occurs, three elements are recorded (two candidates counted down, and one non-candidate got passed), this corresponds to whenever an element dies, two others die with it. So in total, there are > 3*(n/3) elements recorded, but there are only n elements. Contradiction.

        // Generalization:
        // As one can see, the point is whenever an element dies, 2 others die with it. For general k, just choose k candidates, and there will be (k-1) elements die with it. The proof is almost verbatim.
    }
}
