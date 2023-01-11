function canCompleteCircuit(gas: number[], cost: number[]): number {
  // for (let i = 0; i < gas.length; i += 1){
  //     if (gas[i] < cost[i]) continue;
  //     let current_gas: number = gas[i] - cost[i];
  //     let temp_idx: number = i + 1 >= gas.length ? 0 : i + 1;
  //     let is_travel_able: boolean = false;
  //     while (!is_travel_able){
  //         if (temp_idx === i){
  //             is_travel_able = true;
  //             break;
  //         }
  //         if (temp_idx >= gas.length){
  //             temp_idx = 0;
  //             continue;
  //         }
  //         current_gas += gas[temp_idx];
  //         if (current_gas < cost[temp_idx]){
  //             break;
  //         }
  //         current_gas -= cost[temp_idx];
  //         temp_idx += 1;
  //     }
  //     if (is_travel_able) return i
  // }
  // return -1;

  // unique solution GURANTEED
  // so the sum of all difference should be bigger or the same as zero
  // so when we encounter a
  let total_gas: number = 0;
  let start: number = 0;
  let temp_gas: number = 0;
  for (let i = 0; i < gas.length; i += 1) {
    temp_gas += gas[i] - cost[i];
    total_gas += gas[i] - cost[i];
    if (temp_gas < 0) {
      // we assume before the net positive difference
      start = i + 1;
      temp_gas = 0; // resetting when we got negative value
    }
  }
  return total_gas >= 0 ? start : -1;
}
console.log(canCompleteCircuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]));
