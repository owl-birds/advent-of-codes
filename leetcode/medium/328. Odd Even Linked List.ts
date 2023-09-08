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

function oddEvenList(head: ListNode | null): ListNode | null {
  if (!head) return null;
  const new_head: ListNode = new ListNode();
  // O(n) time and o(1) space
  let temp_new_node: ListNode | null = new_head;
  let temp_node: ListNode | null = head;
  let idx: number = 0;
  // odd first
  while (temp_node) {
    if ((idx + 1) % 2 !== 0) {
      temp_new_node.val = temp_node.val;
      if (temp_node.next && temp_node.next.next) {
        temp_new_node.next = new ListNode();
        temp_new_node = temp_new_node.next;
      }
    }
    idx += 1;
    temp_node = temp_node.next;
  }

  if (head.next) temp_new_node.next = new ListNode();
  temp_new_node = temp_new_node.next;
  temp_node = head;
  idx = 0;
  // even next
  while (temp_node) {
    if ((idx + 1) % 2 === 0 && temp_new_node) {
      temp_new_node.val = temp_node.val;
      if (temp_node.next && temp_node.next.next) {
        temp_new_node.next = new ListNode();
        temp_new_node = temp_new_node.next;
      }
    }
    idx += 1;
    temp_node = temp_node.next;
  }
  //

  // // O(n) time and O(n) space
  // let idx: number = 0;
  // const odd: number[] = [];
  // const even: number[] = [];
  // while (temp_node) {
  //     if ((idx+1) % 2 !== 0) {
  //         odd.push(temp_node.val);
  //     }else {
  //         even.push(temp_node.val);
  //     }
  //     idx += 1;
  //     temp_node = temp_node.next;
  // }
  // let odd_i: number = 0;
  // let even_i: number = 0;
  // temp_node = new_head;
  // // console.log(odd, "\n", even);
  // while (odd_i < odd.length) {
  //     temp_node.val = odd[odd_i]
  //     if (odd_i + 1 >= odd.length) break;
  //     temp_node.next = new ListNode();
  //     temp_node = temp_node.next;
  //     odd_i += 1;
  // }
  // temp_node.next = new ListNode();
  // temp_node = temp_node.next;
  // while (even_i < even.length) {
  //     temp_node.val = even[even_i]
  //     if (even_i + 1 >= even.length) break;
  //     temp_node.next = new ListNode();
  //     temp_node = temp_node.next;
  //     even_i += 1;
  // }
  // //

  return new_head;
}
