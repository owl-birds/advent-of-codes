impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut pref: Vec<Vec<u64>> = vec![];
        let mut suff: Vec<Vec<u64>> = vec![];
        let MOD = 12345u64;

        for i in 0..grid.len() {
            pref.push(vec![]);
            suff.push(vec![]);
        }

        let mut ridx: usize = grid.len();
        let mut pref_mut: u64 = 1;
        let mut suff_mut: u64 = 1;
        for i in 0..grid.len() {
            ridx -= 1;
            for j in 0..grid[0].len() {
                // pref_mut *= (grid[i][j] % MOD);
                pref_mut *= (grid[i][j] as u64);
                pref_mut %= MOD;
                pref[i].push(pref_mut);
            }
            let mut cidx: usize = grid[ridx].len();
            while cidx > 0 {
                cidx -= 1;
                // suff_mut *= (grid[ridx][cidx] % MOD);
                suff_mut *= (grid[ridx][cidx] as u64);
                suff_mut %= MOD;
                suff[ridx].push(suff_mut);
            }
            suff[ridx].reverse();
        }
        // println!("pref : {:?}", pref);
        // println!("suff : {:?}", suff);
        // println!("{}", 449145368 * 767292749);

        for i in 0..grid.len() {
            res.push(vec![]);
            let len: usize = res.len();
            for j in 0..grid[0].len() {
                if i == 0 && j == 0 && grid[0].len() > 1 {
                    res[len - 1].push((suff[i][j + 1] % MOD) as i32);
                    continue;
                }
                if i == grid.len() - 1 && j == grid[0].len() - 1 && grid[0].len() > 1 {
                    res[len - 1].push((pref[i][j - 1] % MOD) as i32);
                    continue;
                }
                if grid[0].len() == 1 && grid.len() > 1 {
                    if i == 0 {
                        res[len - 1].push((suff[i + 1][j] % MOD) as i32);
                    } else if i == grid.len() - 1 {
                        res[len - 1].push((pref[i - 1][j] % MOD) as i32);
                    } else {
                        res[len - 1].push((pref[i - 1][j] * suff[i + 1][j] % MOD) as i32);
                    }
                    continue;
                }
                if j == grid[i].len() - 1 && grid.len() > 1 {
                    res[len - 1].push((pref[i][j - 1] * suff[i + 1][0] % MOD) as i32);
                    continue;
                }
                if j == 0 && i >= 1 {
                    res[len - 1]
                        .push((pref[i - 1][grid[i].len() - 1] * suff[i][j + 1] % MOD) as i32);
                    continue;
                }
                if j > 0 && j < grid[i].len() - 1 {
                    res[len - 1].push((pref[i][j - 1] * suff[i][j + 1] % MOD) as i32);
                    continue;
                }
                res[len - 1].push(0);
            }
        }

        res
    }
}
