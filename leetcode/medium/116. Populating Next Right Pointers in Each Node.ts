//   Definition for Node.
class MyNode4 {
  val: number;
  left: MyNode4 | null;
  right: MyNode4 | null;
  next: MyNode4 | null;
  constructor(val?: number, left?: MyNode4, right?: MyNode4, next?: MyNode4) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
    this.next = next === undefined ? null : next;
  }
}

function connect(root: MyNode4 | null): MyNode4 | null {
  if (!root) return null;

  // this algorith will not make any copy of the tree
  // it will modify the tree that u inputed
  const node_arr_level: MyNode4[][] = [];
  const queue: { tree_node: MyNode4; level: number }[] = [];
  queue.push({ tree_node: root, level: 0 });
  while (queue.length > 0) {
    const node_level = queue.shift();
    if (node_level) {
      const { tree_node, level } = node_level;
      if (node_arr_level[level]) {
        node_arr_level[level].push(tree_node);
      } else {
        node_arr_level.push([tree_node]);
      }
      if (tree_node.left) {
        queue.push({ tree_node: tree_node.left, level: level + 1 });
      }
      if (tree_node.right) {
        queue.push({ tree_node: tree_node.right, level: level + 1 });
      }
    }
  }
  // console.log(node_arr_level);
  for (const level of node_arr_level) {
    for (let i = 0; i < level.length - 1; i += 1) {
      level[i].next = level[i + 1];
    }
  }

  return root;
}
