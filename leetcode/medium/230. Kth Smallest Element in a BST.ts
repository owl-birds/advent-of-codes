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

function kthSmallest(root: TreeNode | null, k: number): number {
  const nums: number[] = [];
  traverse(root, nums);
  nums.sort((a, b) => {
    if (a > b) {
      return 1;
    }
    if (a < b) {
      return -1;
    }
    return 0;
  });
  return nums[k - 1];
}

const traverse = (root: TreeNode | null, nums: number[]) => {
  if (!root) return;
  nums.push(root.val);
  if (root.left) traverse(root.left, nums);
  if (root.right) traverse(root.right, nums);
};
