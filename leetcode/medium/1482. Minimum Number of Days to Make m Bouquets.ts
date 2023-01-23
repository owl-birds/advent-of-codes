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

function minDays2(bloomDay: number[], m: number, k: number): number {
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
    const unique_day: number[] = [...unique_day_set].sort((a,b)=>a-b);
    // const unique_day: number[] = Object.keys(unique_day_obj).map((day)=>Number(day));
    // const max_day: number = Math.max(...bloomDay);
    // console.log(is_feasible(3, m, k, bloomDay));

    // Binary Search
    let start: number = 0;
    let end: number = unique_day.length-1;

    while (start < end){
        const mid: number = Math.floor((start+end)/2);
        if (is_feasible(unique_day[mid], m, k, bloomDay)){
            end = mid;
            continue;
        }
        start = mid + 1;
    }
    return is_feasible(unique_day[start], m, k, bloomDay) ? unique_day[start] : -1;
    // TLE
    const temp_flower_bloom: number[] = new Array(bloomDay.length).fill(0);
    for (let day of unique_day){
        let bouquet_made: number = 0; 
        // if (!unique_day_set.has(day)) continue;
        for (let i = 0; i < bloomDay.length; i += 1){
            if (day === bloomDay[i]){
                temp_flower_bloom[i] = 1;
            }
        }
        // console.log(temp_flower_bloom);
        let count_flower: number = 0;
        for (let i = 0; i < temp_flower_bloom.length; i += 1){
            if (temp_flower_bloom[i] === 1){
                count_flower += 1;
            }else{
                if (count_flower === k){
                    bouquet_made += 1;
                }
                count_flower = 0;
                continue;
            }
            if (count_flower === k){
                bouquet_made += 1;
                count_flower = 0;
            }
            if (bouquet_made === m) return day;
        }
        // if (bouquet_made >= m) return day;
    }
    return -1;
};

const is_feasible = (day: number, bouquets: number, flowers: number, bloomDay: number[]): boolean =>{
    const flower_bloom: number[] = bloomDay.map((d)=>{
        if (day >= d) return 1;
        else return 0;
    });
    // console.log(flower_bloom);
    let count_bouquets: number = 0;
    let count_flowers: number = 0;
    for (let i = 0; i < flower_bloom.length; i += 1){
        if (i === flower_bloom.length-1){
            count_flowers += flower_bloom[i]
            count_bouquets += Math.floor(count_flowers/flowers);
            break;
        }
        if (flower_bloom[i] === 0){
            count_bouquets += Math.floor(count_flowers/flowers);
            count_flowers = 0;
            continue;
        }
        count_flowers += flower_bloom[i]
    }
    return count_bouquets >= bouquets;
}
