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

function inorderTraversal(root: TreeNode | null): number[] {
  const ans: number[] = [];

  // RECURSION
  if (root) traverse(root, ans);

  // ITERATION

  return ans;
}
const traverse = (node: TreeNode | null, arr: number[]) => {
  if (!node) {
    return;
  }
  if (node.left) {
    traverse(node.left, arr);
  }
  arr.push(node.val);
  if (node.right) {
    traverse(node.right, arr);
  }
};
