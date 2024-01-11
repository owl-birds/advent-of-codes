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

function rangeSumBST(root: TreeNode | null, low: number, high: number): number {
  let count: number[] = [0];
  count_in_range(root, low, high, count);
  return count[0];
}
const count_in_range = (
  root: TreeNode | null,
  low: number,
  high: number,
  count: number[]
) => {
  if (!root) return;

  if (low <= root.val && root.val <= high) count[0] += root.val;
  console.log(root.val);
  if (root.left) count_in_range(root.left, low, high, count);
  if (root.right) count_in_range(root.right, low, high, count);
};
