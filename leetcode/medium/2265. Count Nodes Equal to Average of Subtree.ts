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

function averageOfSubtree(root: TreeNode | null): number {
  if (root === null) return 0;
  const tree_info: { [index: string]: number } = {};
  const total_ind = "total";
  const freq_ind = "freq";
  const count_ind = "count";
  tree_info[total_ind] = 0;
  tree_info[freq_ind] = 0;
  tree_info[count_ind] = 0;

  count_eq_node_avg(root, tree_info, total_ind, freq_ind, count_ind);

  // TEST
  // subtree_info(root, tree_info, total_ind, freq_ind);
  // console.log(tree_info);
  // TEST
  return tree_info[count_ind];
}

const count_eq_node_avg = (
  tree_node: TreeNode | null,
  tree_info: { [index: string]: number },
  total_ind: string = "total",
  freq_ind: string = "freq",
  count_ind: string = "count"
) => {
  if (tree_node === null) return;
  tree_info[total_ind] = 0;
  tree_info[freq_ind] = 0;
  subtree_info(tree_node, tree_info, total_ind, freq_ind);
  // console.log("-",tree_info);
  if (Math.floor(tree_info[total_ind] / tree_info[freq_ind]) === tree_node.val)
    tree_info[count_ind] += 1;
  if (tree_node.left)
    count_eq_node_avg(
      tree_node.left,
      tree_info,
      total_ind,
      freq_ind,
      count_ind
    );
  if (tree_node.right)
    count_eq_node_avg(
      tree_node.right,
      tree_info,
      total_ind,
      freq_ind,
      count_ind
    );
};

const subtree_info = (
  tree_node: TreeNode | null,
  tree_info: { [index: string]: number },
  total_ind: string = "total",
  freq_ind: string = "freq"
) => {
  if (tree_node === null) return;
  tree_info[total_ind] += tree_node.val;
  tree_info[freq_ind] += 1;
  if (tree_node.left)
    subtree_info(tree_node.left, tree_info, total_ind, freq_ind);
  if (tree_node.right)
    subtree_info(tree_node.right, tree_info, total_ind, freq_ind);
};
