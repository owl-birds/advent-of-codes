function judgeSquareSum(c: number): boolean {
    const sqrt_c: number = Math.floor(Math.sqrt(c));

    // TYPE 1
    // faster solution : using hash table
    const sqrt_obj: {[num_sqr: number]: boolean} = {};
    for (let num_1 = 0; num_1 <= sqrt_c; num_1 += 1){
        sqrt_obj[num_1**2] = true;
    }
    // linear on part 1 solution
    for (let num_2 = 0; num_2 <= sqrt_c; num_2 += 1){
        const sqr_2: number = num_2**2;
        const diff: number = c - sqr_2;
        // if (sqr_2 === diff) continue;
        if (sqrt_obj[diff]) return true;
    }
    return false;

    // TYPE 2
    // slow solution
    for (let num_1 = 0; num_1 <= sqrt_c; num_1 += 1){
        for (let num_2 = num_1 + 1; num_2 <= sqrt_c; num_2 += 1){
            if ((num_1**2 + num_2**2) === c) return true;
        }
    }
    return false;
};
