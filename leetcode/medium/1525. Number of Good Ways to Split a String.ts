function numSplits(s: string): number {
  let count_good_way: number = 0;

  // USING PREFIX AND SUFFIX
  const prefix: number[] = [];
  const suffix: number[] = [];

  const freq_map: Map<string, number> = new Map();

  // prefix
  for (let i = 0; i < s.length; i += 1) {
    if (!freq_map.has(s[i])) {
      freq_map.set(s[i], 1);
    } else {
      freq_map.set(s[i], freq_map.get(s[i])! + 1);
    }
    prefix[i] = freq_map.size;
  }

  // clear MAP
  freq_map.clear();

  // suffix
  for (let i = s.length - 1; i >= 0; i -= 1) {
    if (!freq_map.has(s[i])) {
      freq_map.set(s[i], 1);
    } else {
      freq_map.set(s[i], freq_map.get(s[i])! + 1);
    }
    suffix[i] = freq_map.size;
  }

  // finding the good way to split
  for (let i = 1; i < s.length; i += 1) {
    if (prefix[i - 1] === suffix[i]) count_good_way += 1;
  }

  return count_good_way;

  // TLE SOLUTION
  // for (let i = 0; i < s.length-1; i += 1){
  //     if (is_good(s.substring(0, i + 1), s.substring(i+1))) count_good_way += 1;
  // }
  // return count_good_way;
}

const is_good = (s_left: string, s_right: string): boolean => {
  const left: Set<string> = new Set();
  const right: Set<string> = new Set();
  for (let letter of s_left) {
    left.add(letter);
  }
  for (let letter of s_right) {
    right.add(letter);
  }
  return left.size === right.size;
  // return new Set(s_left.split("")).size === new Set(s_right.split("")).size;
};
