declare global {
  interface Array<T> {
    groupBy(fn: (item: T) => string): Record<string, T[]>;
  }
}

Array.prototype.groupBy = function (fn) {
  const group_object: { [key: string]: any[] } = {};
  for (let item of this) {
    const key = fn(item);
    // console.log(key);
    if (!group_object[key]) {
      group_object[key] = [item];
      continue;
    }
    group_object[key].push(item);
  }
  return group_object;
};

/**
 * [1,2,3].groupBy(String) // {"1":[1],"2":[2],"3":[3]}
 */
