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

function kthLargestLevelSum(root: TreeNode | null, k: number): number {
  const level_sum: { [level: string]: number } = {};
  const queue: [TreeNode | null, number][] = [];
  // [[TreeNode, level], ...]
  queue.push([root, 1]);
  while (queue.length > 0) {
    // const curr_node = queue.shift();
    const curr_node = queue.pop();
    // const [node, level] = queue.pop();
    if (curr_node && curr_node[0] && curr_node[0].left)
      queue.push([curr_node[0].left, curr_node[1] + 1]);
    if (curr_node && curr_node[0] && curr_node[0].right)
      queue.push([curr_node[0].right, curr_node[1] + 1]);
    if (curr_node && curr_node[1] && curr_node[0] && !level_sum[curr_node[1]]) {
      level_sum[curr_node[1]] = curr_node[0].val;
      continue;
    }
    curr_node &&
      curr_node[0] &&
      curr_node[1] &&
      (level_sum[curr_node[1]] += curr_node[0].val);
  }
  // console.log("level sum", level_sum);
  const sum_arr: number[] = Object.values(level_sum);
  if (sum_arr.length < k) return -1;
  sum_arr.sort((a, b) => b - a);
  return sum_arr[k - 1];
}
