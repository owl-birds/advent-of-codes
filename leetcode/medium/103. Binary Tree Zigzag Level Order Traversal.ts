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
function zigzagLevelOrder(root: TreeNode | null): number[][] {
  // odd level and even level
  if (!root) return [];
  const queue: { tree_node: TreeNode; level: number }[] = [];
  const result: number[][] = [];
  queue.push({ tree_node: root, level: 0 });
  while (queue.length > 0) {
    const { tree_node, level } = queue.shift()!;
    if (result[level] === undefined) {
      result[level] = [tree_node.val];
    } else {
      result[level].push(tree_node.val);
    }
    if (tree_node.left) {
      queue.push({ tree_node: tree_node.left, level: level + 1 });
    }
    if (tree_node.right) {
      queue.push({ tree_node: tree_node.right, level: level + 1 });
    }
  }

  // reversing the odd level on the tree
  for (let i = 0; i < result.length; i += 1) {
    if (i % 2 !== 0) result[i] = result[i].reverse();
  }
  return result;
  /// second solution without reverse
  /// LEvel Size
  //   if (!root) return [];
  //   const result: number[][] = [];
  //   const queue: TreeNode[] = [];
  //   let is_even: boolean = true;
  //   queue.push(root);
  //   while (queue.length > 0) {
  //     let level_length: number = queue.length;
  //     const level: number[] = [];
  //     while (level_length > 0) {
  //       const temp_node: TreeNode = queue.shift()!;
  //       if (is_even) level.push(temp_node.val);
  //       else level.unshift(temp_node.val);

  //       // branches
  //       if (temp_node.left) queue.push(temp_node.left);
  //       if (temp_node.right) queue.push(temp_node.right);
  //       level_length -= 1;
  //     }
  //     is_even = !is_even;
  //     result.push(level);
  //   }
  //   return result;
}
