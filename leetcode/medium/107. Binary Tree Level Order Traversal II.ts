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

function levelOrderBottom(root: TreeNode | null): number[][] {
  if (root === null) return [];
  const result: number[][] = [];

  const queue: [TreeNode, number][] = [];
  queue.push([root, 0]);

  while (queue.length > 0) {
    const curr_node_depth = queue.shift();
    if (curr_node_depth) {
      const curr_node = curr_node_depth[0];
      const curr_depth = curr_node_depth[1];
      if (result[curr_depth]) {
        result[curr_depth].push(curr_node.val);
      } else {
        result.push([curr_node.val]);
      }
      if (curr_node.left) {
        queue.push([curr_node.left, curr_depth + 1]);
      }
      if (curr_node.right) {
        queue.push([curr_node.right, curr_depth + 1]);
      }
    }
  }

  return result.reverse();
}
