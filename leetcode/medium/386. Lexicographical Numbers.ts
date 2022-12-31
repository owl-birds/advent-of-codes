function lexicalOrder(n: number): number[] {
  const nums: string[] = [];
  for (let i = 1; i <= n; i += 1) {
    nums.push(String(i));
  }
  nums.sort((a, b) => {
    for (let i = 0; i < a.length; i += 1) {
      if (a[i] && b[i] === undefined) return 1;
      if (b[i] && a[i] === undefined) return -1;
      if (a[i].charCodeAt(0) > b[i].charCodeAt(0)) return 1;
      if (a[i].charCodeAt(0) < b[i].charCodeAt(0)) return -1;
    }
    return 0;
  });
  return nums.map((num) => Number(num));
}
