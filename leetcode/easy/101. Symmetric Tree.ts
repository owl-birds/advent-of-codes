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
// using level order
const level_order_traverse = (
  root: TreeNode | null,
  is_start_left: boolean = true
): any[] => {
  const result: any[] = [];
  if (!root) return result;
  const queue: any[] = [];
  queue.push(root);
  while (queue.length > 0) {
    const temp_node: TreeNode | null = queue.shift();
    if (!temp_node) result.push(null);
    else result.push(temp_node.val);
    if (temp_node) {
      // if (!temp_node.left && !temp_node.right) continue; // we want all of the branches and leafs, even its null
      if (is_start_left) {
        queue.push(temp_node.left);
        queue.push(temp_node.right);
        continue;
      }
      // start right
      queue.push(temp_node.right);
      queue.push(temp_node.left);
    }
  }
  return result;
};
function isSymmetric(root: TreeNode | null): boolean {
  if (!root) return true;
  if (root.left === null && root.right) return false;
  if (root.left && root.right === null) return false;
  const left_sub_tree: any[] = level_order_traverse(root.left, true);
  const right_sub_tree: any[] = level_order_traverse(root.right, false);
  if (left_sub_tree.length !== right_sub_tree.length) return false;
  for (let i = 0; i < left_sub_tree.length; i += 1) {
    if (left_sub_tree[i] !== right_sub_tree[i]) return false;
  }
  return true;
}
