import { readTxtFileToArr } from "../../modules";
const stringsArr: string[] = readTxtFileToArr("input.txt");
console.log(stringsArr[stringsArr.length - 1]);
// A nice string is one with all of the following properties:

//    It contains at least three vowels (aeiou only),
//      like aei, xazegov, or aeiouaeiouaeiou.
//    It contains at least one letter that appears
//      twice in a row, like xx, abcdde(dd), or
//      aabbccdd(aa, bb, cc, or dd).
//    It does not contain the strings ab, cd, pq,
//      or xy, even if they are part of
//      one of the other requirements.

// at least three vowels
const checkThreeVowels = (str: string): boolean => {
  const vowelRegex: RegExp = /[aeiou]/;
  let count: number = 0;
  for (let i = 0; i < str.length; i += 1) {
    if (vowelRegex.test(str[i])) count += 1;
    if (count >= 3) return true;
  }
  return false;
};
// console.log(checkThreeVowels("ugknbfddgicrmopn"));

// forbidden two strings
const isForbiddenTwoStrings = (str: string): boolean => {
  const forbidden: { [str: string]: true } = {
    ab: true,
    cd: true,
    pq: true,
    xy: true,
  };
  for (let i = 0; i < str.length - 1; i += 1) {
    if (forbidden[`${str[i]}${str[i + 1]}`]) return true;
  }
  return false;
};
// console.log(isForbiddenTwoStrings("ugknbfddgicrmopn"));
// console.log(isForbiddenTwoStrings("haegwjzuvuyypxyu"));

// one letter that appear twice in a row
const isContainedTwoSameLetters = (str: string): boolean => {
  for (let i = 0; i < str.length - 1; i += 1) {
    if (str[i] === str[i + 1]) return true;
  }
  return false;
};

const isNiceString = (str: string): boolean => {
  return (
    checkThreeVowels(str) &&
    !isForbiddenTwoStrings(str) &&
    isContainedTwoSameLetters(str)
  );
};

const countNiceString = (strs: string[]): number => {
  let countNice: number = 0;
  for (let i = 0; i < strs.length; i += 1) {
    if (isNiceString(strs[i])) countNice += 1;
  }
  return countNice;
};

console.log("PART I");
console.log(isNiceString("dvszwmarrgswjxmb"));
console.log(isNiceString("jchzalrnumimnmhp"));
console.log(isNiceString("ugknbfddgicrmopn"));

// Now, a nice string is one with all of the following properties:

//    It contains a pair of any two letters that
//      appears at least twice in the string
//      without overlapping, like xyxy(xy) or
//      aabcdefgaa(aa), but not like aaa(aa,
//      but it overlaps).
//    It contains at least one letter which repeats
//      with exactly one letter between them,
//      like xyx, abcdefeghi(efe), or even aaa.

// first Rule
const isPassFirstRule = (str: string): boolean => {
  // finding the consecutive letters that at least appear twice
  for (let i = 0; i < str.length - 1; i += 1) {
    const pairRegEx: RegExp = new RegExp(`${str[i]}${str[i + 1]}`);
    for (let j = i + 2; j < str.length - 1; j += 1) {
      if (pairRegEx.test(`${str[j]}${str[j + 1]}`)) return true;
    }
  }
  return false;
};
// second Rule
const isPassSecondRule = (str: string): boolean => {
  for (let i = 0; i < str.length - 2; i += 1) {
    if (str[i] === str[i + 2]) return true;
  }
  return false;
};
// combined
const isNiceString2: (str: string) => boolean = (str: string): boolean => {
  return isPassFirstRule(str) && isPassSecondRule(str);
};
// count in an array of strings
const countNiceString2: (strs: string[]) => number = (
  strs: string[]
): number => {
  let count: number = 0;
  for (let i = 0; i < strs.length; i += 1) {
    if (isNiceString2(strs[i])) count += 1;
  }
  return count;
};

console.log("PART II");
console.log("//");
console.log(isPassFirstRule("qjhvhtzxzqqjkmpb"));
console.log(isPassSecondRule("qjhvhtzxzqqjkmpb"));
console.log("//");
console.log(isPassFirstRule("uurcxstgmygtbstg"));
console.log(isPassSecondRule("uurcxstgmygtbstg"));
console.log("//");
console.log(isPassFirstRule("ieodomkazucvgmuy"));
console.log(isPassSecondRule("ieodomkazucvgmuy"));
console.log("//");
console.log(isPassFirstRule("xxyxx"));
console.log(isPassSecondRule("xxyxx"));
console.log("//");

console.log("PART ONE AND PART TWO RESULTS");
console.log(countNiceString(stringsArr));
console.log(countNiceString2(stringsArr));
