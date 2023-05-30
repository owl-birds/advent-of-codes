const simple_hash_operation_2 = (str: string, max_value: number): number => {
  let hash: number = 0;
  for (let i = 0; i < str.length; i += 1) {
    hash += str.charCodeAt(i);
  }
  return hash % max_value;
};
class MyHashMap {
  protected storage: number[][][];
  protected storate_limit: number;
  constructor() {
    this.storate_limit = 100; // 100 buckets
    this.storage = new Array(this.storate_limit).fill([]); // populate it with buckets
  }

  put(key: number, value: number): void {
    const index: number = simple_hash_operation_2(`${key}`, this.storate_limit);
    // search  the buckets, if the key is already existed
    for (let key_value of this.storage[index]) {
      if (key_value[0] === key) {
        key_value[1] = value;
        return;
      }
    }
    this.storage[index].push([key, value]);
  }

  get(key: number): number {
    const index: number = simple_hash_operation_2(`${key}`, this.storate_limit);
    // search  the buckets
    for (let key_value of this.storage[index]) {
      if (key_value[0] === key) {
        return key_value[1];
      }
    }
    return -1;
  }

  remove(key: number): void {
    const index: number = simple_hash_operation_2(`${key}`, this.storate_limit);
    // search  the buckets
    for (let i = 0; i < this.storage[index].length; i += 1) {
      if (this.storage[index][i][0] === key) {
        this.storage[index].splice(i, 1); // remove the key_value pair
        return;
      }
    }
  }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * var obj = new MyHashMap()
 * obj.put(key,value)
 * var param_2 = obj.get(key)
 * obj.remove(key)
 */
