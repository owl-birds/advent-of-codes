function findAnagrams(s: string, p: string): number[] {
    const result_idx: number[] = [];
    const p_len: number = p.length;
    // const p_regex: RegExp = new RegExp(`[${p}]{${p_len}}`);
    // console.log(p_regex);
    const p_freq: {[char: string]: number} = {};
    for (let char of p){
        if (!p_freq[char]){
            p_freq[char] = 1;
            continue;
        }
        p_freq[char] += 1;
    }
    // console.log(p_freq);
    for (let i = 0; i <= s.length-p_len; i += 1){
        if (p_freq[s[i]] === undefined) continue;
        const temp_freq: {[char: string]: number} = {};
        for (let j = i; j < i+p_len; j += 1){
            if (p_freq[s[j]] === undefined) break;
            if (!temp_freq[s[j]]){
                temp_freq[s[j]] = 1;
                continue;
            }
            temp_freq[s[j]] += 1;
        }
        if (is_two_obj_same(p_freq, temp_freq)) result_idx.push(i);
        // wip
        // if (p_regex.test(s.substring(i, i + p_len))){
        //     result_idx.push(i);
        //     console.log(s.substring(i, i + p_len));
        // }
    }   
    return result_idx;
};

const is_two_obj_same = (obj_1: {}, obj_2: {}): boolean=>{
    const key_1: string[] = Object.keys(obj_1);
    const key_2: string[] = Object.keys(obj_2);
    if (key_1.length !== key_2.length) return false;
    let count: number = 0;
    for (let key of key_1){
        if (obj_2[key] !== undefined) count += 1;
    }
    if (count !== key_2.length) return false;
    for (let key of key_1){
        if (obj_1[key] !== obj_2[key]) return false;
    }
    return true;
}
