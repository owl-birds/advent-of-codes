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

function findMode(root: TreeNode | null): number[] {
  if (!root) return [];
  const result: number[] = [];
  const num_freq: { [index: number]: number } = {};
  // let max_freq: number = traverse_bst(root, num_freq, 0);
  let obj_info: { [index: string]: number } = { max: 0 };
  traverse_bst(root, num_freq, obj_info, "max");
  console.log(obj_info, " - ", num_freq);

  for (let key of Object.keys(num_freq)) {
    if (num_freq[Number(key)] === obj_info["max"]) {
      result.push(Number(key));
    }
  }

  return result;
}

const traverse_bst = (
  tree_node: TreeNode | null,
  num_freq: { [index: number]: number },
  obj_info: { [index: string]: number },
  max_ind: string = "max"
) => {
  if (tree_node == null) return;
  if (num_freq[tree_node.val] !== undefined) {
    num_freq[tree_node.val] += 1;
    if (num_freq[tree_node.val] > obj_info[max_ind]) {
      obj_info[max_ind] = num_freq[tree_node.val];
    }
  } else {
    num_freq[tree_node.val] = 1;
    if (1 > obj_info[max_ind]) obj_info[max_ind] = 1;
  }
  if (tree_node.left) {
    traverse_bst(tree_node.left, num_freq, obj_info, max_ind);
  }
  if (tree_node.right) {
    traverse_bst(tree_node.right, num_freq, obj_info, max_ind);
  }
  return;
};
