//   Definition for node.
class MyNode2 {
  val: number;
  children: MyNode2[];
  constructor(val?: number) {
    this.val = val === undefined ? 0 : val;
    this.children = [];
  }
}

function preorder(root: MyNode2 | null): number[] {
  if (!root) return [];
  // // RECURSIVE SOLUTION
  // const result: number[] = [];

  // const pre_order_traversal = (node: MyNode2) => {
  //   result.push(node.val);
  //   for (let child of node.children) {
  //     pre_order_traversal(child);
  //   }
  // };
  // pre_order_traversal(root);

  // return result;

  // ITERATIVE SOLUTION
  const result: number[] = [];
  const stack: MyNode2[] = [];

  stack.push(root);
  while (stack.length > 0) {
    const temp_node: MyNode2 = stack.pop()!;
    result.push(temp_node.val);
    for (let i = temp_node.children.length - 1; i >= 0; i -= 1) {
      stack.push(temp_node.children[i]);
    }
  }

  return result;
}
