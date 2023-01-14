function numOfSubarrays(arr: number[]): number {
  // const prefix_sum: number[] = [];
  // prefix_sum[0] = arr[0];

  // {pref_sum: number; count_odd: number}
  // even - even ::: even number
  // odd - odd ::: even number
  /// IMPORTANT
  // odd - even ::: odd number
  // even - odd ::: odd number
  const prefix_sum: { sum: number; even_before: number; odd_before: number }[] =
    [];
  let odd_before: number = 0;
  let even_before: number = 0;
  prefix_sum.push({ sum: arr[0], odd_before, even_before });
  if (arr[0] % 2 === 0) even_before += 1;
  else odd_before += 1;
  for (let i = 1; i < arr.length; i += 1) {
    // prefix_sum[i] = prefix_sum[i-1] + arr[i];
    const curr_prefix_sum: number = prefix_sum[i - 1]["sum"] + arr[i];
    prefix_sum.push({ sum: curr_prefix_sum, odd_before, even_before });
    if (curr_prefix_sum % 2 === 0) even_before += 1;
    else odd_before += 1;
  }
  let count_odd_sum: number = 0;
  for (let i = 0; i < prefix_sum.length; i += 1) {
    if (prefix_sum[i]["sum"] % 2 === 0) {
      count_odd_sum += prefix_sum[i]["odd_before"];
      continue;
    }
    count_odd_sum += 1;
    count_odd_sum += prefix_sum[i]["even_before"];
  }
  return count_odd_sum % (10 ** 9 + 7);

  // TLE PART
  // for (let i = 0; i < arr.length; i += 1){
  //     for (let j = i; j < arr.length; j += 1){
  //         if ((prefix_sum[j] - prefix_sum[i] + arr[i]) % 2 !== 0) count_odd_sum += 1;
  //     }
  // }
  // return count_odd_sum;
}
