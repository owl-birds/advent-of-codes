# Dynamic Programming

// cheapest, longest, shorthest, most expensive
// optimization problems -- can implement dynamic programming

n = 3
k = 2 // can only take one or two steps at a time

p = [3, 2, 4] // price per steps

0 3 2 4 :: prices
0 1 2 3 :: steps

1. F(i) (this function is the minimum cost to get to the ith steps) --> objective function
2. base cases -->
   F(0) --> 0 at 0th step : base case 1
   F(1) --> 3 at 1th step : base case

   F(2) --> 2 at 2th step 2
   F(3) --> 6 at 3th step 3

   step 0, step 2, step 3

3. F(n) = P(n) + min(F(n-1), F(n-2))
4. bottom-up
5. F(n)

problem : paid staicase

you rare climbing a paid staircase. it takes n steps to reach the top and you have to pay p[i] to step on the ith stair. each time you can climb 1 or 2 steps. whtas the cheapest amount you have to pay to get to the top of the staircase

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
