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

function getMinimumDifference(root: TreeNode | null): number {
  const all_nodes: number[] = [];
  const queue: TreeNode[] | null = [];
  root && queue.push(root);
  while (queue.length > 0) {
    let node_shifted = queue.shift();
    if (!node_shifted) break;
    let curr_node: TreeNode = node_shifted;
    all_nodes.push(curr_node.val);
    if (curr_node.left) queue.push(curr_node.left);
    if (curr_node.right) queue.push(curr_node.right);
  }
  // console.log("all nodes", all_nodes);

  let min_abs_diff = Infinity;

  for (let i = 0; i < all_nodes.length; i += 1) {
    for (let j = i + 1; j < all_nodes.length; j += 1) {
      if (min_abs_diff === 1) return min_abs_diff;
      min_abs_diff = Math.min(
        Math.abs(all_nodes[i] - all_nodes[j]),
        min_abs_diff
      );
    }
  }

  return min_abs_diff;
}
