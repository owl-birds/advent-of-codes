function minEatingSpeed(piles: number[], h: number): number {
  const max_banana: number = Math.max(...piles);
  // const min_banana: number = Math.min(...piles);

  // can_eat : not optimized algorithm
  // can_eat_2 : more optimized

  // binary search
  let start_banana: number = 1;
  let end_banana: number = max_banana;
  while (start_banana < end_banana) {
    const mid: number = Math.floor((start_banana + end_banana) / 2);
    if (can_eat_2(mid, piles, h)) {
      end_banana = mid;
      continue;
    }
    start_banana = mid + 1;
  }
  return start_banana;

  // linear : TLE
  // for (let eat = 1; eat <= max_banana; eat += 1){
  //     if (can_eat_2(eat, piles, h)) return eat;
  // }
}

const can_eat_2 = (eat: number, piles: number[], hour: number): boolean => {
  let count_hour: number = 0;
  let idx: number = 0;
  while (idx < piles.length) {
    count_hour += Math.ceil(piles[idx] / eat);
    if (count_hour > hour) return false;
    idx += 1;
  }
  return true;
};

const can_eat = (eat: number, piles: number[], hour: number): boolean => {
  let count_hour: number = 0;
  let idx: number = 0;
  let curr_piles: number = piles[idx];
  while (idx < piles.length) {
    curr_piles -= eat;
    count_hour += 1;
    if (count_hour > hour) return false;
    if (curr_piles <= 0) {
      idx += 1;
      curr_piles = piles[idx];
    }
  }
  return true;
};
