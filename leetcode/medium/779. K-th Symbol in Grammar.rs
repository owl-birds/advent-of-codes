impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        Self::depth_first_search(n, k, 0)
        // if n == 0 {return -1}
        // if n == 1 {return 0}
        // let mut rows: Vec<i32> = vec![0]; // initiate with [0] first row

        // // // Iterative TLE
        // // let mut temp: i32 = n;
        // // while temp > 1 {
        // //     if rows.len() >= k as usize {break}
        // //     let mut i: usize = 0;
        // //     while i < rows.len() {
        // //         if rows[i] == 0 {
        // //             rows.splice(i..i+1, [0, 1]);
        // //             i += 2;
        // //             continue;
        // //         }
        // //         rows.splice(i..i+1, [1, 0]);
        // //         if rows.len() >= k as usize {break}
        // //         i += 2;
        // //     }
        // //     temp -= 1;
        // // }
        // // // Iterative

        // // // Recursion TLE
        // // Self::kth_grammar_recursive(&mut rows, n, k);
        // // // Recursion

        // println!("{:?}", rows);

        // rows[(k-1) as usize]
    }
    pub fn depth_first_search(n: i32, k: i32, root_val: i32) -> i32 {
        if n == 1 {
            return root_val;
        }
        let total_nodes = i32::pow(2, (n - 1) as u32);
        // Target node will be present in the right half sub-tree of the current root node.
        if k > total_nodes / 2 {
            let next_root_val = if root_val == 0 { 1 } else { 0 };
            return Self::depth_first_search(n - 1, k - (total_nodes / 2), next_root_val);
        }
        // Otherwise, the target node is in the left sub-tree of the current root node.
        let next_root_val = if root_val == 0 { 0 } else { 1 };
        return Self::depth_first_search(n - 1, k, next_root_val);
    }
    pub fn kth_grammar_recursive(rows: &mut Vec<i32>, temp_n: i32, k: i32) -> () {
        if temp_n == 1 || rows.len() > k as usize {
            return;
        }
        let mut i: usize = 0;
        while i < rows.len() {
            if rows[i] == 0 {
                rows.splice(i..i + 1, [0, 1]);
                i += 2;
                continue;
            }
            rows.splice(i..i + 1, [1, 0]);
            i += 2;
        }
        Self::kth_grammar_recursive(rows, temp_n - 1, k);
    }
}

// Create a method depthFirstSearch which takes n number of rows in the current tree, k target node position in the last row, and rootVal current tree's root's value as parameters:

//     If n is 1, then we will have a single node in our tree and this node is our target node. So, we return its value rootVal.

//     Find the number of nodes in the last row of the current tree, totalNodes, 2(n−1)2^{(n - 1)}2(n−1).

//     If the current target node k lies in the left half of the last row of the current subtree (i.e. k <= totalNodes / 2), we will move to the left sub-tree.
//     If the current node's value rootVal is 0 then the next node's value will be 0, otherwise, the next node's value will be 1.
//     Return depthFirstSearch(n - 1, k, nextRootVal).

//     Otherwise, if the current target node k lies in the right half of the last row of the current subtree (i.e. k > totalNodes / 2), we will move to the right sub-tree.
//     If the current node's value rootVal is 0 then the next node's value will be 1, otherwise, the next node's value will be 0.
//     Additionally, the target's position will change to (k - (totalNodes / 2)).
//     Return depthFirstSearch(n - 1, newPosition, nextRootVal).

// We return the result returned by calling depthFirstSearch(n, k, 0) with the number of rows as n, target node position k, and root node's value 0.
