/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function minDepth(root: TreeNode | null): number {
  if (!root) return 0;
  let min_dept: number = 1;

  // LEVEL ORDER
  const queue: [TreeNode, number][] = [];
  queue.push([root, 1]);

  while (queue.length > 0) {
    const curr_node_depth = queue.shift();
    if (curr_node_depth) {
      const curr_node: TreeNode = curr_node_depth[0]!;
      const curr_depth: number = curr_node_depth[1]!;
      if (curr_node.left === null && curr_node.right === null) {
        min_dept = curr_depth;
        break;
      }
      if (curr_node.left) queue.push([curr_node.left, curr_depth + 1]);
      if (curr_node.right) queue.push([curr_node.right, curr_depth + 1]);
    }
  }
  return min_dept;
}
