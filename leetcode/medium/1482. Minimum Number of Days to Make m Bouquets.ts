// TLE SOLUTION
function minDays(bloomDay: number[], m: number, k: number): number {
  const total_flowers: number = bloomDay.length;
  const need_flowers: number = m * k;
  if (need_flowers > total_flowers) return -1;
  const unique_day_set: Set<number> = new Set(bloomDay);
  // const unique_day_obj: {[day: number]: boolean} = {};
  // for (let day of bloomDay){
  //     if (!unique_day_obj[day]){
  //         unique_day_obj[day] = true;
  //     }
  // }
  const unique_day: number[] = [...unique_day_set].sort((a, b) => a - b);
  // const unique_day: number[] = Object.keys(unique_day_obj).map((day)=>Number(day));
  // const max_day: number = Math.max(...bloomDay);
  const temp_flower_bloom: number[] = new Array(bloomDay.length).fill(0);
  for (let day of unique_day) {
    let bouquet_made: number = 0;
    // if (!unique_day_set.has(day)) continue;
    for (let i = 0; i < bloomDay.length; i += 1) {
      if (day === bloomDay[i]) {
        temp_flower_bloom[i] = 1;
      }
    }
    // console.log(temp_flower_bloom);
    let count_flower: number = 0;
    for (let i = 0; i < temp_flower_bloom.length; i += 1) {
      if (temp_flower_bloom[i] === 1) {
        count_flower += 1;
      } else {
        if (count_flower === k) {
          bouquet_made += 1;
        }
        count_flower = 0;
        continue;
      }
      if (count_flower === k) {
        bouquet_made += 1;
        count_flower = 0;
      }
      if (bouquet_made === m) return day;
    }
    // if (bouquet_made >= m) return day;
  }
  return -1;
}
