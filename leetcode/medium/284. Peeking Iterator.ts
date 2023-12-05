/**
 * // This is the Iterator's API interface.
 * // You should not implement it, or speculate about its implementation
 */
class Iterator {
  hasNext(): boolean {}
  next(): number {}
}

class PeekingIterator {
  protected iterator: Iterator;
  protected chached: number[];
  protected curr_pointer: number;
  constructor(iterator: Iterator) {
    this.iterator = iterator;
    this.chached = [];
    this.curr_pointer = -1;
  }

  peek(): number {
    let peeked_num: number;
    // const next_num = this.iterator.next();
    if (this.curr_pointer == -1) {
      this.curr_pointer = 0; // pointing to the first num in the chached array
      peeked_num = this.iterator.next();
      this.chached.push(peeked_num);
    } else if (this.curr_pointer < this.chached.length) {
      peeked_num = this.chached[this.curr_pointer];
    } else {
      peeked_num = this.iterator.next();
      this.chached.push(peeked_num);
    }
    // console.log(`peek-${this.chached}--pointer:${this.curr_pointer}`);
    return peeked_num;
  }

  next(): number {
    let next_num: number;
    if (this.curr_pointer == -1) {
      this.curr_pointer = 1;
      this.chached.push(this.iterator.next());
      if (this.iterator.hasNext()) {
        this.chached.push(this.iterator.next());
      }
      next_num = this.chached[this.curr_pointer - 1];
    } else if (this.curr_pointer >= this.chached.length) {
      next_num = this.iterator.next();
      this.curr_pointer += 1;
      this.chached.push(next_num);
    } else {
      next_num = this.chached[this.curr_pointer];
      this.curr_pointer += 1;
    }
    // console.log(`next-${this.chached}--pointer:${this.curr_pointer}`);
    return next_num;
  }

  hasNext(): boolean {
    if (this.chached.length > 0 && this.chached[this.curr_pointer] != undefined)
      return true;
    // if (this.iterator.hasNext()) return true;
    return this.iterator.hasNext();
  }
}

/**
 * Your PeekingIterator object will be instantiated and called as such:
 * var obj = new PeekingIterator(iterator)
 * var param_1 = obj.peek()
 * var param_2 = obj.next()
 * var param_3 = obj.hasNext()
 */
