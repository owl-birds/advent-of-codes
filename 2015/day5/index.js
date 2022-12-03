"use strict";
exports.__esModule = true;
var modules_1 = require("../../modules");
var stringsArr = (0, modules_1.readTxtFileToArr)("input.txt");
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
var checkThreeVowels = function (str) {
    var vowelRegex = /[aeiou]/;
    var count = 0;
    for (var i = 0; i < str.length; i += 1) {
        if (vowelRegex.test(str[i]))
            count += 1;
        if (count >= 3)
            return true;
    }
    return false;
};
// console.log(checkThreeVowels("ugknbfddgicrmopn"));
// forbidden two strings
var isForbiddenTwoStrings = function (str) {
    var forbidden = {
        ab: true,
        cd: true,
        pq: true,
        xy: true
    };
    for (var i = 0; i < str.length - 1; i += 1) {
        if (forbidden["".concat(str[i]).concat(str[i + 1])])
            return true;
    }
    return false;
};
// console.log(isForbiddenTwoStrings("ugknbfddgicrmopn"));
// console.log(isForbiddenTwoStrings("haegwjzuvuyypxyu"));
// one letter that appear twice in a row
var isContainedTwoSameLetters = function (str) {
    for (var i = 0; i < str.length - 1; i += 1) {
        if (str[i] === str[i + 1])
            return true;
    }
    return false;
};
var isNiceString = function (str) {
    return (checkThreeVowels(str) &&
        !isForbiddenTwoStrings(str) &&
        isContainedTwoSameLetters(str));
};
var countNiceString = function (strs) {
    var countNice = 0;
    for (var i = 0; i < strs.length; i += 1) {
        if (isNiceString(strs[i]))
            countNice += 1;
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
var isPassFirstRule = function (str) {
    // finding the consecutive letters that at least appear twice
    for (var i = 0; i < str.length - 1; i += 1) {
        var pairRegEx = new RegExp("".concat(str[i]).concat(str[i + 1]));
        for (var j = i + 2; j < str.length - 1; j += 1) {
            if (pairRegEx.test("".concat(str[j]).concat(str[j + 1])))
                return true;
        }
    }
    return false;
};
// second Rule
var isPassSecondRule = function (str) {
    for (var i = 0; i < str.length - 2; i += 1) {
        if (str[i] === str[i + 2])
            return true;
    }
    return false;
};
// combined
var isNiceString2 = function (str) {
    return isPassFirstRule(str) && isPassSecondRule(str);
};
// count in an array of strings
var countNiceString2 = function (strs) {
    var count = 0;
    for (var i = 0; i < strs.length; i += 1) {
        if (isNiceString2(strs[i]))
            count += 1;
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
