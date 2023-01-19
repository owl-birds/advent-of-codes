//**
//  * Definition for singly-linked list.
class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}
//  */

function swapPairs(head: ListNode | null): ListNode | null {
  if (!head || head.next === null) return head;
  let temp_node: ListNode | null = head.next;
  head.next = head.next.next;
  temp_node.next = head;

  const temp_head: ListNode | null = temp_node;
  temp_node = temp_head.next;
  while (temp_node && temp_node.next && temp_node.next.next) {
    const temp: ListNode = temp_node.next;
    temp_node.next = temp_node.next.next;
    temp.next = temp_node.next.next;
    temp_node.next.next = temp;
    temp_node = temp_node.next.next;
  }
  return temp_head;
}
