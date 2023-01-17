function countVowelSubstrings(word: string): number {
  let count_vowels_str: number = 0;
  const vowels_map: Map<string, number> = new Map();

  // BRUTE FORCE
  const vowel_regex: RegExp = /[aeiou]/;
  for (let i = 0; i < word.length; i += 1) {
    if (!vowel_regex.test(word[i])) continue;
    for (let j = i; j < word.length; j += 1) {
      if (!vowel_regex.test(word[j])) break;
      if (vowels_map.has(word[j])) {
        vowels_map.set(word[j], vowels_map.get(word[j])! + 1);
      } else {
        vowels_map.set(word[j], 1);
      }
      if (vowels_map.size === 5) count_vowels_str += 1;
    }
    vowels_map.clear();
  }

  return count_vowels_str;
}
