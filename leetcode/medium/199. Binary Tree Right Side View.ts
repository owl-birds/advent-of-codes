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

function rightSideView(root: TreeNode | null): number[] {
  if (root === null) return [];
  //   let currentNode: TreeNode | null = root;
  const result: number[] = [];

  // there is a better way using the level of the tree
  const tree: number[][] = [];
  //   const queue: [tree_node: TreeNode, level: number][] = [];
  const queue: { tree_node: TreeNode; level: number }[] = [];
  queue.push({ tree_node: root, level: 1 });

  while (queue.length > 0) {
    // const [tree_node, level] = queue.shift();
    const node_level = queue.shift();
    if (node_level) {
      const { tree_node, level } = node_level;
      if (tree[level - 1]) {
        tree[level - 1].push(tree_node.val);
      } else {
        tree.push([tree_node.val]);
      }
      if (tree_node.left) {
        queue.push({ tree_node: tree_node.left, level: level + 1 });
      }
      if (tree_node.right) {
        queue.push({ tree_node: tree_node.right, level: level + 1 });
      }
    }
  }
  // console.log(tree);
  for (let level of tree) {
    result.push(level[level.length - 1]);
  }
  // there is a better way using the level of the tree

  // // below doesnt work
  // const queue: TreeNode[] = [];
  // queue.push(root);
  // while (queue.length > 0) {
  //     const tree_node: TreeNode = queue.shift();
  //     result.push(tree_node.val);
  //     if (tree_node.right) {
  //         queue.push(tree_node.right);
  //     } else if (tree_node.left) {
  //         queue.push(tree_node.left);
  //     }
  // }
  // // above doesnt work

  return result;
}
