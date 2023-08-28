//  Definition for node.
class MyNode {
  val: number;
  children: MyNode[];
  constructor(val?: number) {
    this.val = val === undefined ? 0 : val;
    this.children = [];
  }
}

function postorder(root: MyNode | null): number[] {
  if (root === null) return [];

  // recursive
  const result: number[] = [];
  const traversePostOrder = (node: MyNode): void => {
    for (let child of node.children) {
      traversePostOrder(child);
    }
    result.push(node.val);
  };
  traversePostOrder(root);
  return result;
}
