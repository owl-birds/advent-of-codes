function findJudge(n: number, trust: number[][]): number {
    if (n===1) return n;
    const people_trust_count: {[people: number]: number} = {};
    const did_trust: {[poeple: number]: number} = {};
    for (let t of trust){
        if (!people_trust_count[t[1]]){
            people_trust_count[t[1]] = 0;
        }
        if (!did_trust[t[0]]){
            did_trust[t[0]] = 1;
        }
        people_trust_count[t[1]] += 1;
    }
    for (let person of Object.keys(people_trust_count)){
        if (people_trust_count[person] === (n-1) && !did_trust[person]) return Number(person);
    }
    return -1;
};
