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

/**
 Do not return anything, modify root in-place instead.
 */
function flatten(root: TreeNode | null): void {
  if (!root) return;
  const temp_nums: number[] = [];
  traverse_pre_order(root, temp_nums);
  // console.log(temp_nums);
  root.left = null;
  root.right = null;
  let temp_node: TreeNode | null = root;
  for (let i = 0; i < temp_nums.length; i += 1) {
    temp_node.val = temp_nums[i];
    if (i + 1 < temp_nums.length) {
      temp_node.right = new TreeNode();
      temp_node = temp_node.right;
    }
  }
}
const traverse_pre_order = (
  node: TreeNode | null,
  container: number[]
): void => {
  if (!node) return;
  container.push(node.val);
  node.left && traverse_pre_order(node.left, container);
  node.right && traverse_pre_order(node.right, container);
};
