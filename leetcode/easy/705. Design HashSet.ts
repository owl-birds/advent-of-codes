const simple_hash_operation = (str: string, max_value: number): number => {
  let hash: number = 0;
  for (let i = 0; i < str.length; i += 1) {
    hash += str.charCodeAt(i);
  }
  return hash % max_value;
};

class MyHashSet {
  protected storage: number[][];
  protected storate_limit: number;
  constructor() {
    this.storate_limit = 100; // 100 buckets
    this.storage = new Array(this.storate_limit).fill([]); // populate it with buckets
  }

  add(key: number): void {
    const index: number = simple_hash_operation(`${key}`, this.storate_limit);
    // set unique value
    // search if the key already exist
    for (let key_stored of this.storage[index]) {
      if (key === key_stored) return;
    }
    this.storage[index].push(key);
  }

  remove(key: number): void {
    const index: number = simple_hash_operation(`${key}`, this.storate_limit);
    // finding the key
    for (let i = 0; i < this.storage[index].length; i += 1) {
      if (key === this.storage[index][i]) {
        this.storage[index].splice(i, 1); // remove the key
        return;
      }
    }
  }

  contains(key: number): boolean {
    const index: number = simple_hash_operation(`${key}`, this.storate_limit);
    // search the bucket
    for (let key_stored of this.storage[index]) {
      if (key === key_stored) return true;
    }
    return false;
  }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * var obj = new MyHashSet()
 * obj.add(key)
 * obj.remove(key)
 * var param_3 = obj.contains(key)
 */
