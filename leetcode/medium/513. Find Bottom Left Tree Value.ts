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

function findBottomLeftValue(root: TreeNode | null): number | null {
  if (!root) return null;
  const tree: number[][] = traverse_2(root);
  let result: number = tree[tree.length - 1][0];
  return result;
}

// tree level, is_left, level order
const traverse_2 = (tree_node: TreeNode | null): number[][] => {
  const queue: [number, TreeNode][] = [];
  const tree: number[][] = [];
  if (!tree_node) return tree;

  queue.push([0, tree_node]);
  while (queue.length > 0) {
    const temp_node: [number, TreeNode] | undefined = queue.shift();
    if (!temp_node) break;
    if (tree[temp_node[0]]) {
      tree[temp_node[0]].push(temp_node[1].val);
    } else {
      tree.push([temp_node[1].val]);
    }

    if (temp_node[1].left) {
      queue.push([temp_node[0] + 1, temp_node[1].left]);
    }
    if (temp_node[1].right) {
      queue.push([temp_node[0] + 1, temp_node[1].right]);
    }
  }
  return tree;
};
