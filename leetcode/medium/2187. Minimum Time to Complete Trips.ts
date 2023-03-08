function minimumTime(time: number[], totalTrips: number): number {
    let start = 1;
    let end = Math.max(...time) * totalTrips; // the maximum time that complete the target trips

    while (start < end){
        const mid = Math.floor((start+end)/2);
        if (is_enough(time, mid, totalTrips)) {
            end = mid;
            continue;
        }
        start = mid + 1;
    }
    return start;
};

const is_enough = (times: number[], time: number, target_trip: number)=>{
    let count_trip = 0;
    for (let t of times){
        count_trip += Math.floor(time/t);
    }
    return count_trip >= target_trip;
}
