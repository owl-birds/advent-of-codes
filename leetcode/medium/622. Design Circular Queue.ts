class DoublyNode {
  val: number | undefined;
  next: DoublyNode | null;
  prev: DoublyNode | null;
  constructor(value: number) {
    this.val = value !== undefined ? value : undefined;
    this.next = null;
    this.prev = null;
  }
}

class MyCircularQueue {
  protected size: number;
  protected max_size: number;
  protected head: DoublyNode | null;
  protected tail: DoublyNode | null;

  // array
  protected container: number[];
  protected head_idx: number;
  protected tail_idx: number;

  constructor(k: number) {
    this.size = 0;
    this.max_size = k;
    this.head = null;
    this.tail = null;

    // array
    this.container = new Array(k);
    this.head_idx = -1;
    this.tail_idx = -1;
  }

  enQueue(value: number): boolean {
    // return true
    if (this.size === this.max_size) return false;
    this.size += 1;
    if (this.size === 1) {
      this.container[0] = value;
      this.head_idx = 0;
      this.tail_idx = 0;
      return true;
    }
    if (this.tail_idx + 1 === this.container.length) {
      this.tail_idx = 0;
      this.container[this.tail_idx] = value;
      return true;
    }
    this.container[this.tail_idx + 1] = value;
    this.tail_idx += 1;
    return true;
    // if (!this.head && !this.tail) {
    //     this.head = new DoublyNode(value);
    //     this.tail = this.head;
    //     return true;
    // }
    // this.tail.next = new DoublyNode(value);
    // this.tail.next.prev = this.tail;
    // this.tail = this.tail.next;
    // return true
  }

  deQueue(): boolean {
    if (this.size === 0) return false;
    this.size -= 1;
    if (this.size === 0) {
      this.head_idx -= 1;
      this.tail_idx -= 1;
      return true;
    }
    if (this.head_idx + 1 === this.container.length) {
      this.head_idx = 0;
      return true;
    }
    this.head_idx += 1;
    return true;

    // if (this.size === 0) {
    //     this.head = null;
    //     this.tail = null;
    //     return true;
    // }
    // this.head = this.head.next;
    // this.head.prev = null;
    // return true
  }

  Front(): number {
    if (this.size === 0) return -1;
    return this.container[this.head_idx];
    // return this.head.val;
  }

  Rear(): number {
    if (this.size === 0) return -1;
    return this.container[this.tail_idx];
    // return this.tail.val;
  }

  isEmpty(): boolean {
    return this.size === 0;
  }

  isFull(): boolean {
    return this.size === this.max_size;
  }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * var obj = new MyCircularQueue(k)
 * var param_1 = obj.enQueue(value)
 * var param_2 = obj.deQueue()
 * var param_3 = obj.Front()
 * var param_4 = obj.Rear()
 * var param_5 = obj.isEmpty()
 * var param_6 = obj.isFull()
 */
