//   Definition for Node.
class Node2 {
  val: number;
  next: Node2 | null;
  random: Node2 | null;
  constructor(val?: number, next?: Node2, random?: Node2) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
    this.random = random === undefined ? null : random;
  }
}

function copyRandomList(head: Node2 | null): Node2 | null {
  if (!head) return null;
  // arr ::: [[val, ran_pointer_idx]]
  const arr_copy: [number, Node2 | null, number][] = [];
  let temp_node: Node2 | null = head;
  let list_length: number = 0;

  while (temp_node) {
    list_length += 1;
    arr_copy.push([temp_node.val, temp_node.random, -1]);
    temp_node = temp_node.next;
  }
  // console.log(arr_copy);
  console.log("list length:", list_length);
  for (let i = 0; i < arr_copy.length; i += 1) {
    temp_node = arr_copy[i][1];
    let temp_length: number = 0;
    while (temp_node) {
      temp_length += 1;
      temp_node = temp_node.next;
    }
    if (arr_copy[i][1]) {
      arr_copy[i][2] = list_length - temp_length;
    }
  }
  console.log(arr_copy);

  let new_list: Node2 | null = null;
  if (arr_copy.length > 0) new_list = new Node2(arr_copy[0][0]);
  temp_node = new_list;
  if (!temp_node) return null;
  for (let i = 1; i < arr_copy.length; i += 1) {
    temp_node.next = new Node2(arr_copy[i][0]);
    temp_node = temp_node.next;
  }
  // making the randomized pointer
  let idx: number = 0;
  temp_node = new_list;
  while (temp_node) {
    if (arr_copy[idx][1]) {
      let point_rand: Node2 | null = new_list;
      let count: number = 0;
      while (point_rand) {
        if (count === arr_copy[idx][2]) break;
        count += 1;
        point_rand = point_rand.next;
      }
      temp_node.random = point_rand;
    }
    idx += 1;
    temp_node = temp_node.next;
  }
  return new_list;
}
