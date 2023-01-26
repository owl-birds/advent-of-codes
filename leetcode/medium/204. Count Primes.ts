function countPrimes(n: number): number {
    if (n <= 2) return 0
    // TLE solution
    // const all_number: number[] = [];
    // for (let num = 2; num < n; num += 1){
    //     all_number.push(num);
    // }
    // let count_prime: number = 0;
    // let i: number = 0;
    // let curr_length: number = all_number.length;
    // // for (let i = 0; i < all_number.length; i += 1){
    // while (i < curr_length){
    //     if (!is_prime(all_number[i])) continue;
    //     count_prime += 1;
    //     let j: number = i + 1;
    //     // for (let j = i + 1; j < all_number.length; j += 1){
    //     while (j < curr_length){
    //         if (all_number[j] % all_number[i] === 0) {
    //             all_number.splice(j, 1);
    //             curr_length -= 1;
    //             continue;
    //         }
    //         j += 1;
    //     }
    //     i += 1;
    // }
    // return count_prime;

    // slow solution : but ACCEPTED
    let count: number = 0;
    for (let num = 2; num < n; num += 1){
        if (num % 2 === 0 && num !== 2) continue;
        if (is_prime(num)) count += 1;
    }
    return count;
};
// source : wikipedia, i forgot the algo
function is_prime(num: number) {
  if (num == 2 || num == 3)
    return true;
  if (num <= 1 || num % 2 == 0 || num % 3 == 0)
    return false;  
  for (let i = 5; i * i <= num ; i+=6)
    if (num % i == 0 || num % (i + 2) == 0)
      return false;
  return true;
}
