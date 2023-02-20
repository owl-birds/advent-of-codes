// TLE
function countDigitOne(n: number): number {
    let count = 0;
    for (let i = 1; i <= n; i += 1){
        count += count_digit_one(i);
    }
    return count;
};

const count_digit_one = (num: number)=>{
    let temp_num = num;
    let count = 0;
    while (temp_num > 0){
        if (temp_num % 10 === 1) count += 1;
        temp_num = Math.floor(temp_num/10);
    }
    return count;
}
