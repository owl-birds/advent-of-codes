function mySqrt(x: number): number {
  if (x === 0 || x === 1) return x;

  // def binary_search(array) -> int:
  //     def condition(value) -> bool:
  //         pass

  //     left, right = 0, len(array)
  //     while left < right:
  //         mid = left + (right - left) // 2
  //         if condition(mid):
  //             right = mid
  //         else:
  //             left = mid + 1
  //     return left

  let start: number = 1;
  let end: number = x;
  while (start < end) {
    let mid: number = Math.floor((start + end) / 2);
    if (mid * mid > x) {
      end = mid;
    } else {
      start = mid + 1;
    }
  }
  return start - 1;

  let startIdx: number = 1;
  while (startIdx * startIdx <= x) {
    if (startIdx * startIdx === x) return startIdx;
    startIdx += 1;
    if (startIdx * startIdx > x) {
      startIdx -= 1;
      break;
    }
  }
  return startIdx;
}
