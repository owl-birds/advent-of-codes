class CustomStack {
  protected storage: number[];
  protected max_storage: number;
  protected top: number;
  constructor(maxSize: number) {
    this.top = -1;
    this.max_storage = maxSize;
    this.storage = new Array(this.max_storage);
  }

  push(x: number): void {
    if (this.top < this.max_storage - 1) {
      this.storage[this.top + 1] = x;
      this.top += 1;
    }
  }

  pop(): number {
    if (this.top === -1) return -1;
    const num: number = this.storage[this.top];
    delete this.storage[this.top];
    this.top -= 1;
    return num;
  }

  increment(k: number, val: number): void {
    for (let i = 0; i < k; i += 1) {
      if (i > this.top) break;
      this.storage[i] += val;
    }
  }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * var obj = new CustomStack(maxSize)
 * obj.push(x)
 * var param_2 = obj.pop()
 * obj.increment(k,val)
 */
