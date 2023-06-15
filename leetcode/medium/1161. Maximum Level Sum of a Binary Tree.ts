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

function maxLevelSum(root: TreeNode | null): number {
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
  let max_level_sum = level_sum[1];
  let the_level: number = 1;
  for (let key of Object.keys(level_sum)) {
    if (level_sum[key] > max_level_sum) {
      max_level_sum = level_sum[key];
      the_level = Number(key);
    }
  }

  return the_level;
}
