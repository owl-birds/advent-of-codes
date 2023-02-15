function addToArrayForm(num: number[], k: number): number[] {
    const result_num: number[] = [...num];
    let temp_k: number = k;
    let extra: number = 0;

    let idx: number = result_num.length-1;

    while (temp_k > 0){
        let last_digit: number = temp_k % 10;
        let current_digit: number = idx >= 0 ? result_num[idx] : 0; 
        let temp_add: number = last_digit + current_digit;
        if (extra > 0){
            temp_add += extra;
            extra = 0;
        }
        if (temp_add > 9){
            extra = 1;
        }
        if (idx >= 0){
            result_num[idx] = temp_add % 10;
        }else{
            result_num.unshift(temp_add % 10);
        }

        idx -= 1;
        temp_k = Math.floor(temp_k/10);
        if (temp_k === 0 && extra > 0){
            for (let i = idx; i >= 0; i -= 1){
                temp_add = result_num[i] + extra;
                result_num[i] = temp_add % 10;
                if (temp_add > 9){
                    extra = 1;
                }else{
                    extra = 0;
                    break;
                }
            }
        }
    }
    if (extra > 0) result_num.unshift(1);
    return result_num;
};
