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
function sortedArrayToBST(nums: number[]): TreeNode | null {
  const build_height_balanced_bst = (
    nums: number[],
    left_idx: number,
    right_idx: number
  ): TreeNode | null => {
    if (left_idx > right_idx) return null;
    // we divide the array into two (sorted array)
    const middle_idx: number = Math.floor((left_idx + right_idx) / 2);
    const root: TreeNode = new TreeNode(nums[middle_idx]);
    root.left = build_height_balanced_bst(nums, left_idx, middle_idx - 1);
    root.right = build_height_balanced_bst(nums, middle_idx + 1, right_idx);
    return root;
  };
  return build_height_balanced_bst(nums, 0, nums.length - 1);
}
