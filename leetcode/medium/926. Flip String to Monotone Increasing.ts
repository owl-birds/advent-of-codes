// A binary string is monotone increasing if it consists of some number of 0's (possibly none), followed by some number of 1's (also possibly none).

// You are given a binary string s. You can flip s[i] changing it from 0 to 1 or from 1 to 0.

// Return the minimum number of flips to make s monotone increasing.
function minFlipsMonoIncr(s: string): number {
  // count how many flip to turn s intu all one
  let count_flip: number = 0;
  for (let c of s) {
    if (c === "0") {
      count_flip += 1;
    }
  }
  let min_flips: number = count_flip;

  // finding the minimum
  for (let c of s) {
    if (c === "0") {
      count_flip -= 1;
    } else {
      count_flip += 1;
    }
    min_flips = Math.min(min_flips, count_flip);
  }
  return min_flips;
  //
  let one_flips: number = 0;
  let count_flips: number = 0;

  for (let i = 0; i < s.length; i += 1) {
    if (s[i] === "0") {
      if (one_flips === 0) continue;
      count_flips += 1;
    }
    if (s[i] === "1") {
      one_flips += 1;
    }
    if (count_flips > one_flips) {
      count_flips = one_flips;
    }
  }

  return count_flips;
}
