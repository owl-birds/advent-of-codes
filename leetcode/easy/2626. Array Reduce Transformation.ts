type Fn = (accum: number, curr: number) => number;

function reduce(nums: number[], fn: Fn, init: number): number {
  let accumulate = init;
  for (let num of nums) {
    accumulate = fn(accumulate, num);
  }
  return accumulate;
}
