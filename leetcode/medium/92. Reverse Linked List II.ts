/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function reverseBetween(
  head: ListNode | null,
  left: number,
  right: number
): ListNode | null {
  // this function will make a new copy of linkedList
  if (!head) return null;
  const new_head: ListNode = new ListNode();
  let temp_node1: ListNode | null = head;
  let temp_node2: ListNode | null = new_head;
  while (temp_node1) {
    temp_node2.val = temp_node1.val;
    if (!temp_node1.next) break;
    temp_node2.next = new ListNode();
    temp_node2 = temp_node2.next;
    temp_node1 = temp_node1.next;
  }
  if (left === right) return new_head;
  temp_node1 = new_head;
  // temp_node2 = null;
  let count: number = 1;
  const rev_part: number[] = [];
  while (temp_node1 && count < left) {
    // if (temp_node1.next && count + 1 === left) temp_node2 = temp_node1;
    temp_node1 = temp_node1.next;
    count += 1;
  }
  // if (count == left) {
  temp_node2 = temp_node1;
  while (temp_node2 && count <= right) {
    rev_part.push(temp_node2.val);
    count += 1;
    temp_node2 = temp_node2.next;
  }
  // }
  rev_part.reverse();
  let idx: number = 0;
  count = left;
  while (temp_node1 && count <= right) {
    temp_node1.val = rev_part[idx];

    temp_node1 = temp_node1.next;
    idx += 1;
    count += 1;
  }
  // console.log("1:",temp_node1);
  // console.log("2:",temp_node2);
  console.log("-->", rev_part);

  return new_head;
}
