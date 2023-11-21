use std::collections::HashMap;
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut time: i32 = 0;
        let mut grbg_types: Vec<char> = vec!['G', 'P', 'M'];
        let mut grbg_last_idx: Vec<i32> = vec![-1, -1, -1];
        // let mut grbg_map: HashMap<usize, Vec<i32>> = HashMap::new();
        // [G_longest_idx, P_longest_idx, M_longest_idx]
        for t in 0..grbg_types.len() {
            for i in 0..garbage.len() {
                for c in garbage[i].chars() {
                    if c == grbg_types[t] {
                        grbg_last_idx[t] = i as i32;
                        break;
                    }
                }
            }
        }
        for i in 0..garbage.len() {
            // grbg_map.insert(i, vec![0, 0, 0]);
            time += garbage[i].len() as i32;

            // for c in garbage[i].chars() {
            //     time += 1;
            //     // for t_i in 0..grbg_types.len() {
            //     //     if c == grbg_types[t_i] {
            //     //         if let Some(freq_vec) = grbg_map.get_mut(&i) {
            //     //             freq_vec[t_i] += 1;

            //     //         }
            //     //         break;
            //     //     }
            //     // }
            // }
        }
        // println!("grbgs:{:?}", grbg_map);
        // println!("types:{:?}\nlast:{:?}", grbg_types, grbg_last_idx);

        for i in 0..grbg_last_idx.len() {
            if grbg_last_idx[i] == -1 {
                continue;
            }
            for t_i in 0..grbg_last_idx[i] as usize {
                time += travel[t_i];
            }
        }

        // for t in 0..grbg_types.len() {
        //     let mut travel_idx: usize = 0;
        //     for house_idx in 0..grbg_last_idx[t] as usize {
        //         if house_idx == 0 {
        //             if let Some(freq_vec) = grbg_map.get_mut(&house_idx) {
        //                 time += freq_vec[t];
        //             }
        //             continue;
        //         }
        //         if let Some(freq_vec) = grbg_map.get_mut(&house_idx) {
        //             time += freq_vec[t];
        //         }
        //         time += travel[travel_idx];
        //         travel_idx += 1;
        //     }
        // }

        time
    }
}
