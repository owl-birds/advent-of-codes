//   Definition for a binary tree node.
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

function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
  // First solution : Require  understanding of recursion and BST
  if (p === null || q === null) return p === q;
  if (p.val !== q.val) return false;
  return isSameTree(p.left, q.left) && isSameTree(p.right, q.right);

  // second solution NOOB WAYS
  const traverse_levelorder = (tree_node: TreeNode | null): any[] => {
    const result: any[] = [];
    if (!tree_node) return result;
    const queue: any[] = [];

    //
    queue.push(tree_node);
    while (queue.length > 0) {
      const temp_node: TreeNode | null = queue.shift();
      if (temp_node === null) result.push(null);
      else result.push(temp_node.val);
      if (temp_node) {
        if (temp_node.left === null && temp_node.right === null) continue;
        queue.push(temp_node.left);
        queue.push(temp_node.right);
      }
    }
    return result;
  };
  const tree_p: any[] = traverse_levelorder(p);
  const tree_q: any[] = traverse_levelorder(q);
  if (tree_p.length !== tree_q.length) {
    return false;
  }
  for (let i = 0; i < tree_p.length; i += 1) {
    if (tree_p[i] !== tree_q[i]) {
      return false;
    }
  }
  return true;
}
