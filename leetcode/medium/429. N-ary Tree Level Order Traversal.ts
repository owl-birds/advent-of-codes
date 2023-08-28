//   Definition for node.
class MyNode3 {
  val: number;
  children: MyNode3[];
  constructor(val?: number) {
    this.val = val === undefined ? 0 : val;
    this.children = [];
  }
}

function levelOrder(root: MyNode3 | null): number[][] {
  if (!root) return [];

  const result: number[][] = [];
  const stack: { temp_node: MyNode3; level: number }[] = [];
  // [MyNode3, level_number][]

  stack.push({ temp_node: root, level: 0 });

  while (stack.length > 0) {
    const { temp_node, level } = stack.shift()!;
    if (result[level]) {
      result[level].push(temp_node.val);
    } else {
      result.push([temp_node.val]);
    }
    for (let child of temp_node.children) {
      stack.push({ temp_node: child, level: level + 1 });
    }
  }

  return result;
}
