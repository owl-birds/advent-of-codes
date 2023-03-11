class Solution {
    protected nums_object: {[num: number]: number[]} = {};
    constructor(nums: number[]) {
        for (let i = 0; i < nums.length; i += 1){
            if (!this.nums_object[nums[i]]){
                this.nums_object[nums[i]] = [i];
                continue;
            }
            this.nums_object[nums[i]].push(i);
        }
    }

    pick(target: number): number {
        const idx = Math.floor(Math.random()* this.nums_object[target].length)
        return this.nums_object[target][idx];
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(nums)
 * var param_1 = obj.pick(target)
 */
