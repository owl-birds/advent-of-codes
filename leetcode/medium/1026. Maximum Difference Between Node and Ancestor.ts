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

function maxAncestorDiff(root: TreeNode | null): number {
  const maxs: number[] = [0];
  find_max_abs_diff(root, maxs);

  return maxs[0];
}

const find_max_abs_diff = (node: TreeNode | null, maxs: number[]) => {
  if (!node) return;
  if (node.left) {
    traverse(node.left, maxs, node.val);
    find_max_abs_diff(node.left, maxs);
  }
  if (node.right) {
    traverse(node.right, maxs, node.val);
    find_max_abs_diff(node.right, maxs);
  }
};

const traverse = (node: TreeNode | null, maxs: number[], root_num: number) => {
  if (!node) return;
  if (Math.abs(node.val - root_num) > maxs[0])
    maxs[0] = Math.abs(node.val - root_num);
  if (node.left) traverse(node.left, maxs, root_num);
  if (node.right) traverse(node.right, maxs, root_num);
};
