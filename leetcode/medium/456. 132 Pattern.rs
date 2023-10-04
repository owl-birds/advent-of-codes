impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        // //BRUTE FORCE ;: TLE ofcourse
        // for i in 0..nums.len()-2 {
        //     for j in i+1..nums.len()-1 {
        //         if nums[i] >= nums[j] {continue}
        //         for k in j+1..nums.len() {
        //             if nums[k] > nums[i] && nums[k] < nums[j] {
        //                 return true;
        //             }
        //         }
        //     }
        // }
        // //BRUTE FORCE

        // inspiration : https://leetcode.com/problems/132-pattern/solutions/94089/java-solutions-from-o-n-3-to-o-n-for-132-pattern-updated-with-one-pass-slution/?envType=daily-question&envId=2023-09-30

        // improved -> o(n^2) : STILL TLE
        // what we want is to find if 132 pattern exist or not
        // so we can combine the first two loops together, by keeping track of the minimum number before j
        // let mut min: i32 = i32::MAX;
        // for j in 0..nums.len() {
        //     min = if nums[j] < min {nums[j]} else {min}; // i
        //     if min == nums[j] {continue}
        //     let mut k = nums.len()-1;
        //     while k > j {
        //         if nums[k] > min && nums[k] < nums[j] {return true}
        //         k -= 1;
        //     }
        // }
        // While this solution can be accepted, it runs slow. One obvious drawback is that in the second loop we are throwing away information about elements in the right part of nums that may be "useful" for later combinations. It turns out we can retain this "useful" information by applying the classic space-time tradeoff, which leads to the following O(n) time and O(n) space solution.
        // improved -> o(n^2)

        // o (n)
        //         As I mentioned, to further reduce the time complexity, we need to record the "useful" information about elements in the right part of the input array nums. Since these elements are located at the right part of nums, it will be hard to do so if we are scanning the array from the beginning. So the idea is to scan it from the end while in the meantime keep track of the "useful" information. But still at each index j, we need to know the minimum element for subarray nums[0, j). This can be done by doing a pre-scan in the forward direction and memorize the results for each index in an auxiliary array (we will call the array as arr whose element arr[j] will denote the minimum element in the subarray nums[0, j)).

        // Until now we are kinda vague about the exact meaning of "useful" information, so let's try to be more specific. Assume we're currently scanning (from the end) the element with index j, our task is to find two elements nums[i] and nums[k] to determine if there exists a 132 pattern, with i < j < k. The left element nums[i], as it has been shown in part II, will be chosen as arr[j], the minimum element of subarray nums[0, j). What about the right element nums[k]?

        // The answer to that will address the meaning of "useful" information. First note we are only interested in elements that are greater than arr[j], so it is sensible to maintain only those elements. Second, among all these qualified elements, which one will be the most probable to fall into the range (nums[i], nums[j])? I would say it is the smallest one (i.e., if the smallest one is out of the range, all others will also be out of range). So to sum up, the "useful" information for current index j will be a collection of scanned elements that are greater than arr[j], and nums[k] will be chosen as the smallest one if the collection is not empty.

        // From the analyses above, it looks like we have to do some sorting stuff for the retained elements (or at least find a way to figure out its smallest element). Well, it turns out these elements will be sorted automatically due to the fact that arr[j'] >= arr[j] as long as j' < j. Here is how it goes, which is a proof by induction.

        // At the beginning we have an empty collection and of course it is sorted. Now suppose we are at index j and the corresponding collection is still sorted, let's see if it remains so at index j - 1. First we will check if nums[j] is greater than arr[j]. If not, we simply continue to j - 1. Since the collection is intact so it will be sorted at j - 1. Otherwise, we need to remove elements in the collection that are no greater than arr[j] (this is necessary because some smaller elements may be left over in the collection from previous steps). After removal, we then compare the first element in the collection with nums[j] to see if a 132 pattern has been found, provided the collection is not empty. If so, return true. Otherwise one of the following must be true: the collection is empty or nums[j] is no greater than the first element in the collection. In either case the collection is sorted. Now if we have arr[j - 1] < nums[j], we need to add nums[j] to the collection since it is a qualified number for arr[j - 1]. Again in either case the collection will remain sorted after addition (if it is empty, after addition there is only one element; otherwise since the added element is no greater than the first element in the collection before addition, it will become the new first element after addition and the collection stays sorted).

        // Here is the program with O(n) time and space complexity. There is one minor optimization based on the observation that the total number of elements in the collection will never exceed the total number of elements scanned so far. Therefore the right part of the arr array can be used to serve as the collection. For time complexity, each element in the input array nums will be pushed into and popped out from the collection (or stack to be exact) at most once, the time complexity will be O(n) despite of the nested loop.

        let mut temp_arr: Vec<i32> = nums.to_vec();

        for i in 1..nums.len() {
            temp_arr[i] = if temp_arr[i - 1] < nums[i - 1] {
                temp_arr[i - 1]
            } else {
                nums[i - 1]
            };
        }
        let mut top: usize = nums.len();
        let mut j: usize = nums.len();
        while j > 0 {
            j -= 1;
            if nums[j] <= temp_arr[j] {
                continue;
            }
            while (top < nums.len() && temp_arr[top] <= temp_arr[j]) {
                top += 1;
            }
            if top < nums.len() && nums[j] > temp_arr[top] {
                return true;
            }
            temp_arr[top - 1] = nums[j];
            top -= 1;
        }

        false
    }
}
