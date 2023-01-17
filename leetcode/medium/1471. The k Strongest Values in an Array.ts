/// TLE SOLUTION
function getStrongest(arr: number[], k: number): number[] {
  const temp_arr: number[] = [...arr].sort((a, b) => a - b);
  // const temp_arr: number[] = [...arr];
  const temp_arr_obj: { num: number; score: number }[] = [];
  // const new_arr: number[] = new Array(arr.length);
  const median: number = temp_arr[Math.floor((temp_arr.length - 1) / 2)];
  for (let i = 0; i < temp_arr.length; i += 1) {
    let score: number = 0;
    for (let j = 0; j < temp_arr.length; j += 1) {
      // if (i === j) continue;
      if (is_strong_then(temp_arr[i], temp_arr[j], median)) score += 1;
    }
    const temp_obj: { num: number; score: number } = {
      score,
      num: temp_arr[i],
    };
    temp_arr_obj.push(temp_obj);
  }
  temp_arr_obj.sort((a, b) => b.score - a.score);
  return temp_arr_obj.map((obj) => obj.num).slice(0, 0 + k);
}

const is_strong_then = (
  num_1: number,
  num_2: number,
  median: number
): boolean => {
  const diff_1: number = Math.abs(num_1 - median);
  const diff_2: number = Math.abs(num_2 - median);
  if (diff_1 > diff_2) return true;
  else if (diff_1 < diff_2) return false;
  else {
    if (num_1 > num_2) return true;
    else return false;
  }
};

const start = performance.now();
console.log(
  getStrongest(
    [
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 11, 7, 6, 8, 11, 12,
      11, 2, 2, 2, 2, 3,
    ],
    20
  )
);
const end = performance.now();
console.log(`TIME ${end - start} millisecons`);
