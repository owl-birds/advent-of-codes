
//Definition for a binary tree node.
  class TreeNode {
      val: number
      left: TreeNode | null
      right: TreeNode | null
      constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
          this.val = (val===undefined ? 0 : val)
          this.left = (left===undefined ? null : left)
          this.right = (right===undefined ? null : right)
      }
  }
 

function minDiffInBST(root: TreeNode | null): number {
    if (root === null) return -1;
    
    const in_order_modified = (node: TreeNode): number=>{
        let min_distance = Infinity;
        let prev_node: TreeNode | null = null;
        const traverse = (node: TreeNode | null)=>{
            node.left && traverse(node.left);
            if (prev_node) min_distance = Math.min(min_distance, node.val-prev_node.val);
            prev_node = node;
            node.right && traverse(node.right);     
        }
        traverse(node);
        return min_distance
    }
    return in_order_modified(root)

    const inOrder = (root: TreeNode): number[]=>{
        const result: number[] = [];
        const inOrderTraverse = (node: TreeNode): void=>{
            node.left && inOrderTraverse(node.left);
            result.push(node.val);
            node.right && inOrderTraverse(node.right);
        }
        inOrderTraverse(root);
        return result;
    }
    const allNodesVal: number[] = inOrder(root);
    let minDistance: number = Infinity;
    for (let i = 0; i < allNodesVal.length; i += 1){
        for (let j = i + 1; j < allNodesVal.length; j += 1){
            if (Math.abs(allNodesVal[j]-allNodesVal[i]) < minDistance) minDistance = Math.abs(allNodesVal[j] - allNodesVal[i]);
        }
    }
    return minDistance;
};
