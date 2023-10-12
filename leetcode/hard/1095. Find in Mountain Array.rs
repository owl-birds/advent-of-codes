/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */
use std::collections::HashMap;
impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let length = mountainArr.length();
        let mut store_map: HashMap<i32, i32> = HashMap::new(); // why use HashMap ? cause the problem have constraint that MountainArr.get() ---> can only be called at most 100 times
                                                               // {idx : num/high}

        // println!("{}", length);
        let mut l: i32 = 0;
        let mut r: i32 = length - 1;

        let mut count_get: i32 = 0;

        // find peak
        while l < r {
            let m = (l + r) / 2;
            // println!("#l:{} --- r:{} --- m:{}", l, r, m);
            let mid = mountainArr.get(m);
            store_map.insert(m, mid);
            let mut mid_after = -1;
            if (m + 1) < length {
                // to find is ascending or descending, if ascending then mid is in the left side, if descending then mid is in the right side
                mid_after = mountainArr.get(m + 1);
                store_map.insert(m + 1, mid_after);
                count_get += 1;
            }
            let left = mountainArr.get(l);
            let right = mountainArr.get(r);
            store_map.insert(l, left);
            store_map.insert(r, right);
            count_get += 3;
            // println!("mid:{} left:{} right:{} mid_after:{}", mid, left, right, mid_after);
            // println!("----");
            if mid > left && mid > right && (mid_after == -1 || mid_after > mid) {
                l = m;
                continue;
            }
            if mid > left && mid > right && (mid_after == -1 || mid_after < mid) {
                r = m;
                continue;
            }
            if mid <= left && mid > right {
                r = m - 1;
                continue;
            }
            l = m + 1;
        }
        let peak_idx: i32 = l;
        // println!("{:?}", store_map);
        // println!("peak idx : {}\nl:{}--r:{}", peak_idx,l,r);
        // println!("finding peak, get called : {}", count_get);

        let mut min_idx: i32 = -1;
        // left side
        // println!("left side");
        let mut l = 0;
        let mut r = peak_idx;
        while l < r {
            let m = (l + r) / 2;
            // let mid = mountainArr.get(m);
            let mid = if let Some(val) = store_map.get(&m) {
                *val
            } else {
                count_get += 1;
                mountainArr.get(m)
            };
            // let left = mountainArr.get(l);
            let left = if let Some(val) = store_map.get(&l) {
                *val
            } else {
                count_get += 1;
                mountainArr.get(l)
            };
            // let right = mountainArr.get(r);
            let right = if let Some(val) = store_map.get(&r) {
                *val
            } else {
                count_get += 1;
                mountainArr.get(r)
            };
            // count_get += 3;
            if target == mid {
                min_idx = m;
                break;
            }
            if mid > target {
                r = m;
                continue;
            }
            l = m + 1;
        }
        // println!("left side, get called : {}", count_get);
        if mountainArr.get(l) == target {
            return l;
        }
        if min_idx != -1 {
            return min_idx;
        }
        // right side
        // println!("right side");
        let mut l = peak_idx;
        let mut r = length - 1;
        while l < r {
            let m = (l + r) / 2;
            // let mid = mountainArr.get(m);
            let mid = if let Some(val) = store_map.get(&m) {
                *val
            } else {
                count_get += 1;
                mountainArr.get(m)
            };
            // let left = mountainArr.get(l);
            let left = if let Some(val) = store_map.get(&l) {
                *val
            } else {
                count_get += 1;
                mountainArr.get(l)
            };
            // let right = mountainArr.get(r);
            let right = if let Some(val) = store_map.get(&r) {
                *val
            } else {
                count_get += 1;
                mountainArr.get(r)
            };
            if target == mid {
                min_idx = m;
                break;
            }
            if mid < target {
                r = m;
                continue;
            }
            l = m + 1;
        }
        // println!("right side, get called : {}", count_get);
        if mountainArr.get(l) == target {
            return l;
        }
        min_idx
    }
}
