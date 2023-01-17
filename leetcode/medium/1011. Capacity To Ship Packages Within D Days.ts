function shipWithinDays(weights: number[], days: number): number {
  let start_capacity: number = Math.max(...weights);
  let end_capacity: number = weights.reduce(
    (prev_sum, curr_val) => prev_sum + curr_val,
    0
  );
  // console.log(is_enough_capacity(5, days, weights));

  // binary search
  while (start_capacity < end_capacity) {
    const mid_capacity: number = Math.floor(
      (start_capacity + end_capacity) / 2
    );
    if (is_enough_capacity(mid_capacity, days, weights)) {
      end_capacity = mid_capacity;
      continue;
    }
    start_capacity = mid_capacity + 1;
  }
  return start_capacity;

  // linear
  for (let cap = start_capacity; cap <= end_capacity; cap += 1) {
    if (is_enough_capacity(cap, days, weights)) return cap;
  }
}

const is_enough_capacity = (
  capacity: number,
  days: number,
  weights: number[]
): boolean => {
  let total_weight: number = 0;
  let count_days: number = 1;
  for (let weight of weights) {
    total_weight += weight;
    if (total_weight > capacity) {
      total_weight = weight;
      count_days += 1;
      if (count_days > days) return false;
    }
  }
  return true;
};
