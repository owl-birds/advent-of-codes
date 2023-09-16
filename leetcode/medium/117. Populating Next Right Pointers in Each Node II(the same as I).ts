/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     left: Node | null
 *     right: Node | null
 *     next: Node | null
 *     constructor(val?: number, left?: Node, right?: Node, next?: Node) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function connect(root: Node | null): Node | null {
  // Follow-up:

  // You may only use constant extra space.
  // The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.
  // the first one is a perfect binary tree where each node will always have 2 children but here its a binary tree where there is no such guarantee.

  if (!root) return null;

  // solution below modify the input
  const nodes: Node[][] = [];
  const queue: { tree_node: Node; level: number }[] = [];
  queue.push({ tree_node: root, level: 0 });
  while (queue.length > 0) {
    const node_level = queue.shift();
    if (node_level) {
      const { tree_node, level } = node_level;
      if (nodes[level]) {
        nodes[level].push(tree_node);
      } else {
        nodes.push([tree_node]);
      }
      if (tree_node.left) {
        queue.push({ tree_node: tree_node.left, level: level + 1 });
      }
      if (tree_node.right) {
        queue.push({ tree_node: tree_node.right, level: level + 1 });
      }
    }
  }
  for (let level of nodes) {
    for (let i = 0; i < level.length - 1; i += 1) {
      level[i].next = level[i + 1];
    }
  }

  return root;
}
