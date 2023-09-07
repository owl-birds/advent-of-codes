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

function splitListToParts(
  head: ListNode | null,
  k: number
): Array<ListNode | null> {
  const result: (ListNode | null)[] = [];
  for (let i = 0; i < k; i += 1) {
    result.push(null);
  }
  if (!head) return result;
  let temp_node: ListNode | null = head;
  // const list: number[] = [];
  let list_length: number = 0;
  while (temp_node) {
    // list.push(temp_node.val);
    list_length += 1;
    temp_node = temp_node.next;
  }
  const min_items: number =
    Math.floor(list_length / k) === 0 ? 1 : Math.floor(list_length / k);
  let left_items: number =
    Math.floor(list_length / k) === 0 ? 0 : list_length % k;
  // console.log("list_length: ", list_length, " how many:", list_length/k, "--->", min_items, " left:", left_items);
  let idx: number = 0;
  let res_idx: number = 0;
  temp_node = head;
  while (temp_node && idx < list_length) {
    let limit = min_items;
    if (left_items > 0) {
      limit += 1;
      left_items -= 1;
    }
    let new_node: ListNode = new ListNode();
    let temp_new_node: ListNode = new_node;
    for (let i = idx; i < idx + limit; i += 1) {
      if (!temp_node) break;
      temp_new_node.val = temp_node.val;
      if (i + 1 < idx + limit) {
        temp_new_node.next = new ListNode();
        temp_new_node = temp_new_node.next;
      }
      temp_node = temp_node.next;
    }
    result[res_idx] = new_node;
    res_idx += 1;
  }

  return result;
}
