function isUgly(n: number): boolean {
    if (n <= 0) return false;
    // infinitely better solution
    let temp_n: number = n;
    for (let div of [2,3,5]){
        while (temp_n % div === 0){
            temp_n /= div;
        }
    }
    return temp_n === 1;

    // TLE SOLUTION
    if (n === 1) return true;
    const ugly_factors: {[factor: number]: boolean} = {2:true, 3:true, 5:true};
    let count: number = 0;
    for (let div = 2; div <= Math.floor(n/2); div += 1){
        if (is_prime2(div) && n % div === 0){
            if (!ugly_factors[div]) return false;
        }
        if (n % div === 0) count += 1;
    }
    if (count === 0 && !ugly_factors[n]) return false;
    return true;
};
const is_prime = (num: number): boolean =>{
    for (let i = 2; i <= Math.floor(Math.sqrt(num)); i += 1){
        if (num % i === 0) return false;
    }
    return true;
}
function is_prime2(num) {
  if (num == 2 || num == 3)
    return true;
  if (num <= 1 || num % 2 == 0 || num % 3 == 0)
    return false;  
  for (let i = 5; i * i <= num ; i+=6)
    if (num % i == 0 || num % (i + 2) == 0)
      return false;
  return true;
}
console.log(isUgly(93735177));
