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
class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}
const has_path_sum = (
  root: TreeNode | null,
  target_sum: number,
  curr_sum: number = 0
): boolean => {
  if (root === null) return false;
  if (
    curr_sum + root.val === target_sum &&
    root.left === null &&
    root.right === null
  )
    return true;
  return (
    has_path_sum(root.left, target_sum, curr_sum + root.val) ||
    has_path_sum(root.right, target_sum, curr_sum + root.val)
  );
};

function hasPathSum(root: TreeNode | null, targetSum: number): boolean {
  // solution one using stack and iteration
  if (!root) return false;
  const stack: { tree_node: TreeNode; curr_sum: number }[] = [];
  stack.push({ tree_node: root, curr_sum: 0 });
  while (stack.length > 0) {
    const { tree_node, curr_sum } = stack.pop()!;
    if (
      tree_node.left === null &&
      tree_node.right === null &&
      curr_sum + tree_node.val === targetSum
    )
      return true;
    if (tree_node.left)
      stack.push({
        tree_node: tree_node.left,
        curr_sum: curr_sum + tree_node.val,
      });
    if (tree_node.right)
      stack.push({
        tree_node: tree_node.right,
        curr_sum: curr_sum + tree_node.val,
      });
  }
  return false;

  // solution 2 : using recursive
  return has_path_sum(root, targetSum, 0);
}
