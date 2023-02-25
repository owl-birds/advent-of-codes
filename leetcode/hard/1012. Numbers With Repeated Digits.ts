// TLE
function numDupDigitsAtMostN(n: number): number {

    

    // TLE
    // const n1 = 1976301
    // console.log(n1, is_repeated(n1));
    let temp_num = 1;
    let count = 0;
    while (temp_num <= n){
        if (is_repeated(temp_num)) count += 1;
        temp_num += 1;
    }

    return count;
};


const is_repeated = (num: number)=>{
    let temp_num = num;
    const set: Set<number> = new Set();
    while (temp_num > 0){
        const digit = temp_num % 10;
        if (set.has(digit)) return true;
        set.add(digit);
        temp_num = Math.floor(temp_num/10);
    }
    return false;
}
