// need more cleaning
function insert(intervals: number[][], newInterval: number[]): number[][] {
  const overlap_intervals_idx: number[] = [];
  const result: number[][] = [...intervals];
  for (let i = 0; i < intervals.length; i += 1) {
    if (
      newInterval[0] <= intervals[i][1] &&
      newInterval[1] >= intervals[i][1]
    ) {
      overlap_intervals_idx.push(i);
      continue;
    }
    if (newInterval[1] >= intervals[i][0] && newInterval[0] < intervals[i][0]) {
      overlap_intervals_idx.push(i);
      continue;
    }
    if (
      newInterval[0] >= intervals[i][0] &&
      newInterval[1] <= intervals[i][1]
    ) {
      overlap_intervals_idx.push(i);
      continue;
    }
  }
  if (overlap_intervals_idx.length === 0) {
    // if there are/is no overlap interval
    if (
      (intervals[0] && newInterval[1] < intervals[0][0]) ||
      result.length === 0
    ) {
      result.unshift(newInterval);
      return result;
    }
    for (let i = 0; i < result.length; i += 1) {
      if (
        i === result.length - 1 ||
        (newInterval[0] > result[i][1] && newInterval[1] < result[i + 1][0])
      ) {
        result.splice(i + 1, 0, newInterval);
        break;
      }
    }
    return result;
  }
  const new_interval: number[] = new Array(2);
  // fiding the smallest replacemet for the newe interval ,,, starting index [0]
  if (newInterval[0] > intervals[overlap_intervals_idx[0]][0]) {
    new_interval[0] = intervals[overlap_intervals_idx[0]][0];
  } else {
    new_interval[0] = newInterval[0];
  }
  // finding the biggest replacement for the new interval ,,,, ending index [1]
  if (
    newInterval[1] <
    intervals[overlap_intervals_idx[overlap_intervals_idx.length - 1]][1]
  ) {
    new_interval[1] =
      intervals[overlap_intervals_idx[overlap_intervals_idx.length - 1]][1];
  } else {
    new_interval[1] = newInterval[1];
  }

  // replacing the old interval to the new interval
  result.splice(
    overlap_intervals_idx[0],
    overlap_intervals_idx.length,
    new_interval
  );

  return result;
}
