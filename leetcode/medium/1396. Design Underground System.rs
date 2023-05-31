use std::collections::HashMap;

// check in : 
// {station name : {user_id:time}}
// {user_id: (station_name, time)}
// check out 
// {station name : {user_id:time}}

// average time
// {first_station-second_station : average time}
// {first_station-second_station : [total, count]}


struct UndergroundSystem {
    check_in: HashMap<i32, (String, i32)>,
    // check_out: HashMap<String, HashMap<i32, i32>>,
    average_time: HashMap<String, Vec<f64>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            check_in: HashMap::new(),
            // check_out: HashMap::new(),
            average_time: HashMap::new()
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in.insert(id, (station_name, t));
        // println!("{:?}", self.check_in);
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let station_time = self.check_in.get(&id);
        match station_time{
            Some(station_time)=>{
                let (depart_station, depart_time) = station_time;
                let mut average_key = depart_station.clone();
                average_key.push('-');
                average_key.push_str(&station_name);
                if let Some(average_times) = self.average_time.get_mut(&average_key){
                    // average_times.push((t-depart_time) as f64);
                    average_times[0] += (t-depart_time) as f64;
                    average_times[1] += 1.0;
                    
                    // println!("{:?}", self.average_time);
                    self.check_in.remove(&id);
                    return;
                }
                self.average_time.insert(average_key, vec![(t-depart_time) as f64, 1.0]);
                self.check_in.remove(&id);
                // println!("{:?}", self.average_time);
                // println!("{:?} \ntime:{}", average_key, depart_time);
            },
            None=>{println!("USER HASNT CHECK IN")}
        }
        // if self.check_in.contains_key(&id){
        // }
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let mut average_key = start_station.clone();
        average_key.push('-');
        average_key.push_str(&end_station);
        // println!("{:?}", average_key);
        let average_time = self.average_time.get(&average_key);
        match average_time{
            Some(time_count_vec)=>{
                return time_count_vec[0] / time_count_vec[1];
                // let mut average_time: f64 = 0.0;
                // for i in 0..times.len(){
                //     average_time += times[i];
                // }
                // return average_time / (times.len() as f64); 
            },
            None => return -1.0
        }
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */