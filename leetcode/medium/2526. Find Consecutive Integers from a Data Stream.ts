class DataStream {
  protected parsed_nums: number[];
  protected value: number;
  protected k: number;
  // protected idx: number = 0; // capped at k
  protected count: number;
  constructor(value: number, k: number) {
    this.parsed_nums = [];
    // this.parsed_nums = new Array(k);
    this.value = value;
    this.k = k;
    this.count = 0;
  }

  check(): boolean {
    return this.parsed_nums.length === this.k && this.count === this.k;
  }

  consec(num: number): boolean {
    if (num === this.value) this.count += 1;
    this.parsed_nums.push(num);
    let first_num: undefined | number;
    if (this.parsed_nums.length > this.k) {
      first_num = this.parsed_nums.shift();
    }
    if (first_num === this.value) this.count -= 1;
    // console.log(this.parsed_nums);
    return this.check();

    // SLOW AS HECK
    // this.parsed_nums.push(num);
    // if (this.parsed_nums.length > this.k) {
    //     this.parsed_nums.shift();
    //     // this.parsed_nums = this.parsed_nums.slice(1); // TLE
    // }

    // // TLE
    // // if (this.idx === this.k){
    // //     let shifted_i = 1;
    // //     for (let i = 0; i < this.parsed_nums.length-1; i += 1){
    // //         this.parsed_nums[i] = this.parsed_nums[shifted_i];
    // //         shifted_i += 1;
    // //     }
    // //     this.parsed_nums[shifted_i-1] = num;
    // // }
    // // if (this.idx < this.k){
    // //     this.parsed_nums[this.idx] = num;
    // //     this.idx += 1;
    // // }
    // // console.log(this.parsed_nums, this.idx)
    // // TLE

    // if (this.parsed_nums.length < this.k) return false;

    // // let i = this.parsed_nums.length-this.k; // faster so we not manipulate the array further
    // let i = 0; // if we remove the first in queue everytime the quueue excedd the k
    // while (i < this.parsed_nums.length){
    //     if (this.parsed_nums[i] !== this.value) return false;
    //     i += 1;
    // }
    // return true;
  }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * var obj = new DataStream(value, k)
 * var param_1 = obj.consec(num)
 */
