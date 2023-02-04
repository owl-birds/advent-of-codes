// TLE : CAUSE ITS SUCKS
function checkInclusion(s1: string, s2: string): boolean {
    const char_freq: {[char: string]: number} = {};
    for (let char of s1){
        if (!char_freq[char]){
            char_freq[char] = 1;
            continue;
        }
        char_freq[char] += 1;
    }
    let idx: number = 0;
    let temp_freq: {[char: string]: number} = {...char_freq};
    let count_char: number = 0;
    // for (let char of s2){
    while (idx < s2.length){
        if (temp_freq[s2[idx]] === undefined) {
            temp_freq = {...char_freq};
            count_char = 0;
            idx += 1;
            continue;
        }
        temp_freq[s2[idx]] -= 1;
        if (temp_freq[s2[idx]] < 0){
            temp_freq = {...char_freq};
            idx -= (count_char-1);
            count_char = 0;
            continue;
        }
        count_char += 1;
        // check
        let count: number = 0;
        for (let char_s1 of s1){
            if  (temp_freq[char_s1] === 0) count += 1;
        }
        // console.log(s2[idx]);
        // console.log(temp_freq);
        // console.log(count);
        // console.log("before",count_char);
        // console.log("#");
        if (count === s1.length) return true;
        idx += 1;
    } 
    return false;
};
// STILL TLE : but i think its little optimized
function checkInclusion_2(s1: string, s2: string): boolean {
    // FECK TLE
    const char_freq: {[char: string]: number} = {};
    for (let char of s1){
        if (!char_freq[char]){
            char_freq[char] = 1;
            continue;
        }
        char_freq[char] += 1;
    }
    let idx: number = 0;
    let temp_freq: {[char: string]: number} = {...char_freq};
    // let count_char: number = 0;
    let last_idx: {[letter: string]: number} = {};
    // for (let char of s2){
    while (idx < s2.length){
        if (temp_freq[s2[idx]] === undefined) {
            // temp_freq = {...char_freq};
            for (let char_s1 of s1){
                temp_freq[char_s1] = char_freq[char_s1];
            }
            // count_char = 0;
            idx += 1;
            last_idx = {};
            continue;
        }
        temp_freq[s2[idx]] -= 1;
        if (temp_freq[s2[idx]] < 0){
            // temp_freq = {...char_freq};
            for (let char_s1 of s1){
                temp_freq[char_s1] = char_freq[char_s1];
            }
            // idx -= (count_char-1);
            idx = last_idx[s2[idx]] + 1;
            // count_char = 0;
            last_idx = {};
            continue;
        }
        // console.log(last_idx);
        // count_char += 1;
        if (last_idx[s2[idx]] === undefined) last_idx[s2[idx]] = idx;
        // check
        let count: number = 0;
        for (let char_s1 of s1){
            if  (temp_freq[char_s1] === 0) count += 1;
        }
        // console.log(s2[idx]);
        // console.log(temp_freq);
        // console.log(count);
        // console.log("before",count_char);
        // console.log("#");
        if (count === s1.length) return true;
        idx += 1;
    } 
    return false;
};
