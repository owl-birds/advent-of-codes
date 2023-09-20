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

function detectCycle(head: ListNode | null): ListNode | null {
  if (!head) return null;
  // // solution using hashtable -> Set
  // const node_set: Set<ListNode> = new Set(); // storing the references
  // let temp_node: ListNode | null = head;

  // while (temp_node) {
  //     if (node_set.has(temp_node)) return temp_node;
  //     node_set.add(temp_node);
  //     temp_node = temp_node.next;
  // }
  // return null;
  // // solution using hashtable -> Set

  // solution using floyd cycle detection
  let slow: ListNode | null = head;
  let fast: ListNode | null = head;
  let is_cycle: boolean = false;
  while (slow && fast && fast.next) {
    slow = slow.next;
    fast = fast.next.next;
    // console.log(slow.val, "->", fast.val);
    if (slow === fast) {
      is_cycle = true;
      break;
    }
  }
  if (!is_cycle) return null;
  let result_node: ListNode | null = head;
  while (slow && result_node && result_node !== slow) {
    slow = slow.next;
    result_node = result_node.next;
  }
  return result_node;
  // solution using floyd cycle detection
}

// all the explanations below are not mine

// 2 (X + Y) - (X + Y) = NC

// slow moves 1 step at a time, fast moves 2 steps at a time.
// when slow and fast meet each other, they must be on the cycle
//     x denotes the length of the linked list before starting the circle
//     y denotes the distance from the start of the cycle to where slow and fast met
//     C denotes the length of the cycle
//     when they meet, slow traveled (x + y) steps while fast traveled 2 * (x + y) steps, and the extra distance (x + y) must be a multiple of the circle length C
//         note that x, y, C are all lengths or the number of steps need to move.
//         head, slow, fast are pointers.
//         head moves x steps and arrives at the start of the cycle.
// so we have x + y = N * C, let slow continue to travel from y and after x more steps, slow will return to the start of the cycle.
// At the same time, according to the definition of x, head will also reach the start of the cycle after moving x steps.
// so if head and slow start to move at the same time, they will meet at the start of the cycle, that is the answer.

/////////////
// Define two pointers slow and fast. Both start at
// head node, fast is twice as fast as slow.If it
// reaches the end it means there is no cycle,
// otherwise eventually it will eventually catch up
// to slow pointer somewhere in the cycle.

// Let the distance from the first node to the the
// node where cycle begins be A, and let say the slow
// pointer travels travels A + B.The fast pointer
// must travel 2A + 2B to catch up.The cycle size is
// N.Full cycle is also how much more fast pointer
// has traveled than slow pointer at meeting point.

// A + B + kN = 2A + 2B
// kN = A +B

// From our calculation slow pointer traveled
// exactly full cycle when it meets fast pointer,
// and since originally it travled A before starting
// on a cycle, it must travel A to reach the point
// where cycle begins! We can start another slow
// pointer at head node, and move both pointers
// until they meet at the beginning of a cycle.
