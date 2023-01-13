class NumArray {
  protected storage: number[];
  protected prefix_sum: number[];
  constructor(nums: number[]) {
    this.storage = [...nums];
    this.prefix_sum = new Array(nums.length).fill(0);
    this.prefix_sum[0] = this.storage[0];
    for (let i = 1; i < this.storage.length; i += 1) {
      this.prefix_sum[i] = this.prefix_sum[i - 1] + this.storage[i];
    }
  }

  sumRange(left: number, right: number): number {
    return this.prefix_sum[right] - this.prefix_sum[left] + this.storage[left];

    if (left === right) return this.storage[left];
    if (left === 0) return this.prefix_sum[right];
    return this.prefix_sum[right] - this.prefix_sum[left - 1];
    //
    let sum: number = 0;
    for (let i = left; i <= right; i += 1) {
      sum += this.storage[i];
    }
    return sum;
  }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * var obj = new NumArray(nums)
 * var param_1 = obj.sumRange(left,right)
 */
