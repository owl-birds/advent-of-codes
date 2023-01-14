function numSub(s: string): number {
  // or [1-nth] * ([1-nth] + 1) / 2
  let count: number = 0;
  let temp_count_1: number = 0;
  for (let i = 0; i <= s.length; i += 1) {
    if (i === s.length || s[i] !== "1") {
      count += (temp_count_1 * (temp_count_1 + 1)) / 2;
      temp_count_1 = 0;
      continue;
    }
    temp_count_1 += 1;
  }
  return count % (10 ** 9 + 7);

  // 1
  // 1 :: 1

  // 11
  // 12 :: 3

  // 111
  // 123 :: 6

  // below is the solution
  // let count: number = 0;
  // let temp_count: number = 1;
  // for (let i = 0; i <= s.length; i += 1){
  //     if (i === s.length || s[i] !== "1" ) {
  //         temp_count = 1;
  //         continue;
  //     }
  //     count += temp_count;
  //     temp_count += 1;
  // }
  // return count % (10**9+7);
}
