function longestCommonPrefix(strs: string[]): string {
    if (strs.length === 1) return strs[0];
    let longest_prefix: string = "";
    const first_word: string = strs[0];
    for (let length = 1; length <= first_word.length; length += 1){
        const temp_prefix: string = first_word.substring(0, length);
        // console.log("temp PREFIX:",temp_prefix);
        const regex: RegExp = new RegExp(`^${temp_prefix}`)
        for (let i = 1; i < strs.length; i += 1){
            if (strs[i].length < length) return longest_prefix;
            if (!regex.test(strs[i])) return longest_prefix;
        }
        longest_prefix = temp_prefix;
    }
    return longest_prefix;
};
