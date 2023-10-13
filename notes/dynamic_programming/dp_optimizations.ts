/*
```typescript
const paid_staircase = (n: number, prices: number[]) => {
  // n : the length of price
  //
  const dp: number[] = new Array(n + 1);
  // specify base cases
  dp[0] = 0;
  dp[1] = prices[1];
  for (let i = 2; i <= n; i += 1) {
    dp[i] = Math.min(dp[i - 1], dp[i - 2]) + prices[i];
  }
  return dp[n];
};
```
*/

// cheapest, longest, shorthest, most expensive
// optimization problems -- can implement dynamic programming

// n = 3
// k = 2 // can only take one or two steps at a time

// p = [3, 2, 4] // price per steps

// 0 3 2 4 :: prices
// 0 1 2 3 :: steps

// 1. F(i) (this function is the minimum cost to get to the ith steps) --> objective function
// 2. base cases -->
//    F(0) --> 0 at 0th step : base case 1
//    F(1) --> 3 at 1th step : base case

//    F(2) --> 2 at 2th step 2
//    F(3) --> 6 at 3th step 3

//    step 0, step 2, step 3

// 3. F(n) = P(n) + min(F(n-1), F(n-2))
// 4. bottom-up
// 5. F(n) :: is the answer

// problem : paid staicase/steps

// you rare climbing a paid staircase. it takes n steps to reach the top and you have to pay p[i] to step on the ith stair. each time you can climb 1 or 2 steps. whtas the cheapest amount you have to pay to get to the top of the staircase

const min_cost = (step_prices: number[], max_step_per_move: number): number => {
  //
  const step_length = step_prices.length;
  const dp: number[] = new Array(step_length + 1);
  //   const dp: number[][] = [];
  //   for (let i = 0; i < step_length + 1; i += 1) {
  //     dp.push([]);
  //   }

  dp[0] = 0;
  dp[1] = step_prices[0];
  //   dp[0] = [0, -1];
  //   dp[1] = [step_prices[0], 0];
  for (let i = 2; i < dp.length; i += 1) {
    //
    const f_n = function_n(dp, step_prices[i - 1], i, max_step_per_move);
    dp[i] = f_n;
    // const f_n = function_n(dp, step_prices[i - 1], i, max_step_per_move);
    // dp[i] = f_n;
  }
  // console.log("-----------");
  // console.log(step_prices, " steps : ", max_step_per_move);
  // console.log(dp);
  // //   //   console.log(
  // //   // `prices:[${step_prices}]-${step_prices.length}\ndp:[${dp}]-${dp.length}`
  // //   //   );
  // console.log("-----------");
  // if (step_prices.length == max_step_per_move) {
  //   return dp[dp.length - 1];
  // }
  // console.log("111");
  let result: number = Number.MAX_VALUE;
  let idx = dp.length - 1;
  while (idx > 0 && idx >= dp.length - max_step_per_move) {
    if (result > dp[idx]) {
      result = dp[idx];
    }
    idx -= 1;
  }
  //   if (dp.length == 2) {
  //     return dp[1][0];
  //   }
  //   let result: number =
  //     dp[dp.length - 1][0] < dp[dp.length - 2][0]
  //       ? dp[dp.length - 1][0]
  //       : dp[dp.length - 2][0];
  //   const temp_dp: number[][] = [];

  ////
  return result;
};
const function_n = (
  dp: number[],
  //   dp: number[][],
  curr_price: number,
  curr_idx: number,
  max_step: number
  // ): number[] => {
): number => {
  //
  let temp_idx = curr_idx - 1;
  let min_prices = dp[temp_idx];
  while (temp_idx >= curr_idx - max_step) {
    if (dp[temp_idx] < min_prices) {
      min_prices = dp[temp_idx];
    }
    temp_idx -= 1;
  }
  return curr_price + min_prices;
  //   let temp_idx = curr_idx - 1;
  //   let min_prices = dp[temp_idx];
  //   while (temp_idx >= curr_idx - max_step) {
  //     //
  //     if (dp[temp_idx][0] < min_prices[0]) {
  //       min_prices = [dp[temp_idx][0], temp_idx];
  //     }
  //     temp_idx -= 1;
  //   }
  //   return [min_prices[0] + curr_price, min_prices[1]];
};

// TEST CASE
const test_case_1 = [
  1, 100, 1, 1, 1,
  //
  100, 1, 1, 100, 1,
];
const test_case_2 = [10, 15, 20];
const test_case_3 = [10];
const test_case_4 = [10, 15];
const test_case_5 = [10, 15, 20, 25, 1, 3, 4, 5];
const test_case_6 = [10, 15, 20, 25, 1, 3, 4, 5, 1];
console.log(min_cost(test_case_1, 2));
console.log("------");
console.log(min_cost(test_case_2, 2));
console.log(min_cost(test_case_2, 3));
console.log("------");
console.log(min_cost(test_case_3, 2));
console.log(min_cost(test_case_4, 2));
console.log(min_cost(test_case_4, 3));
console.log(min_cost(test_case_5, 4));
console.log(min_cost(test_case_6, 4));
