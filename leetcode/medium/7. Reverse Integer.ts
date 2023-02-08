function reverse(x: number): number {
    // if (x >= 1534236469 || x <= -1563847412) return 0;
    let num = x < 0 ? x * -1 : x;
    let new_num = 0;
    while (num > 0){
        new_num = new_num*10 + num%10;
        // num%10 ::: we wanna take the last digit
        num = Math.floor(num/10); 
        // ex : 123/10 : 12,3 floored 12 and so on
        if (new_num > 2**31-1) return 0;
    }
    return x < 0 ? -1 * new_num : new_num;
};
