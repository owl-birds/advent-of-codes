function gcdOfStrings(str1: string, str2: string): string {
    if (str1.length === str2.length){
        return str1 === str2 ? str1 : "";
    }
    // more simple solution : BUT SLOW 
    const shorter: string = str1.length > str2.length ? str2 : str1;
    const longer: string = str1.length > str2.length ? str1 : str2;
    let div: string = "";
    for (let length = 1; length <= shorter.length; length += 1){
        const temp_div: string = shorter.substring(0, length);
        const reg_short: string[] | null = shorter.match(new RegExp(temp_div, "g"));
        const reg_long: string[] | null = longer.match(new RegExp(temp_div, "g")); 
        const shorter_res: number = reg_short ? reg_short.length * temp_div.length : 0;
        const longer_res: number = reg_long ? reg_long.length * temp_div.length : 0;
        // console.log("s", shorter_res);
        // console.log("l", longer_res);
        if (shorter_res === shorter.length && longer_res === longer.length) div = temp_div;
    }
    // console.log(div);
    return div;


    // FASTER SOLUTION
    const allDivisorStr1: string[] = [];
    let strLength: number = 1;
    let tempIdx: number = 0;
    let divisor: string = str1.substring(tempIdx, tempIdx+strLength);
    while (strLength <= str1.length){
        if (tempIdx === str1.length){
            if (str2.length % strLength === 0){
                allDivisorStr1.push(divisor);
            }
            strLength += 1;
            tempIdx = 0;
            divisor = str1.substring(tempIdx, tempIdx+strLength);           
            continue;
        }
        if (str1.length % strLength !== 0 || divisor !== str1.substring(tempIdx, tempIdx+strLength)){
            strLength += 1;
            tempIdx = 0;
            divisor = str1.substring(tempIdx, tempIdx+strLength);
            continue;
        }
        tempIdx += strLength;
    }
    let result: string = "";
    // for (let div of allDivisorStr1){
    for (let i = allDivisorStr1.length-1; i >= 0; i -= 1){
        tempIdx = 0;
        strLength = allDivisorStr1[i].length;
        // divisor = str2.substring(tempIdx, tempIdx+allDivisorStr1[0].length);
        while (strLength <= str2.length){
            if (tempIdx === str2.length){
                return allDivisorStr1[i];
                // if (allDivisorStr1[i].length > result.length){
                //     result = allDivisorStr1[i];
                // }
                // break;
            }
            if (allDivisorStr1[i] !== str2.substring(tempIdx, tempIdx + strLength)){
                break;
            }
            tempIdx += strLength;
        }
    }
    return result;
};
