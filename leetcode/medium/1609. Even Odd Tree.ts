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

function isEvenOddTree(root: TreeNode | null): boolean {
  if (!root) return false;
  const tree: number[][] = traverse_level(root);
  let is_true = true;
  // console.log(tree);
  // console.log("---");
  for (let i = 0; i < tree.length; i += 1) {
    // console.log(tree[i]);
    let temp: number = tree[i][0];
    if (
      ((i + 1) % 2 !== 0 && temp % 2 === 0) ||
      ((i + 1) % 2 === 0 && temp % 2 !== 0)
    ) {
      is_true = false;
      break;
    }
    if ((i + 1) % 2 !== 0) {
      for (let j = 1; j < tree[i].length; j += 1) {
        if (tree[i][j] <= temp || tree[i][j] % 2 === 0) {
          is_true = false;
          break;
        }
        temp = tree[i][j];
      }
    } else {
      for (let j = 1; j < tree[i].length; j += 1) {
        if (tree[i][j] >= temp || tree[i][j] % 2 !== 0) {
          is_true = false;
          break;
        }
        temp = tree[i][j];
      }
    }
    if (!is_true) break;
  }
  return is_true;
}

const traverse_level = (tree_node: TreeNode | null): number[][] => {
  const result: number[][] = [];
  if (!tree_node) return result;
  const queue: [number, TreeNode][] | undefined = [];
  queue.push([0, tree_node]);

  while (queue.length > 0) {
    const temp: [number, TreeNode] | undefined = queue.shift();
    if (!temp) break;
    if (result[temp[0]]) {
      result[temp[0]].push(temp[1].val);
    } else {
      result.push([temp[1].val]);
    }

    if (temp[1].left) {
      queue.push([temp[0] + 1, temp[1].left]);
    }
    if (temp[1].right) {
      queue.push([temp[0] + 1, temp[1].right]);
    }
  }

  return result;
};
