function commonFactors(a: number, b: number): number {
    if (a === 1 && b === 1) return 1;
    let count: number = 1;
    const max_num: number = Math.max(a,b);
    const min_num: number = Math.min(a,b);
    if (max_num % min_num === 0) count += 1;

    for (let div = 2; div <= Math.floor(min_num/2); div += 1){
        if (max_num % div === 0 && min_num % div === 0) count += 1;
    }
    return count;
};
