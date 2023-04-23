function filter(arr: number[], fn: (n: number, i: number) => any): number[] {
  // work everytime solution
  // const result: number[] = [];
  // for (let i = 0; i < arr.length; i += 1){
  //     if (fn(arr[i], i)){
  //         result.push(arr[i]);
  //     }
  // }
  // return result;

  // using Array.reduce method
  return arr.reduce((accumulate: number[], curr_num: number, idx: number) => {
    if (fn(curr_num, idx)) {
      accumulate.push(curr_num);
    }
    return accumulate;
  }, []);
}
