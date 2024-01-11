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

function leafSimilar(root1: TreeNode | null, root2: TreeNode | null): boolean {
  const leafs_1: number[] = [];
  const leafs_2: number[] = [];
  find_leafs(root1, leafs_1);
  find_leafs(root2, leafs_2);
  // console.log(leafs_1);
  // console.log(leafs_2);

  if (leafs_1.length !== leafs_2.length) return false;
  for (let i = 0; i < leafs_1.length; i += 1) {
    if (leafs_1[i] !== leafs_2[i]) return false;
  }

  return true;
}

const find_leafs = (node: TreeNode | null, leafs: number[]) => {
  if (!node) return;
  if (!node.left && !node.right) leafs.push(node.val);
  if (node.left) find_leafs(node.left, leafs);
  if (node.right) find_leafs(node.right, leafs);
};
