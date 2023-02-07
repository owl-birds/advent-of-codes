// REAL SLOW SOLUTION 9.5 SECONDS LMAO
function totalFruit(fruits: number[]): number {
    interface Bucket{
        type: number; fruits: number; is_empty: boolean
    }
    let max_fruits: number = 0;
    
    for (let i = 0; i < fruits.length; i += 1){
        const bucket_1: Bucket = {
            type: -1,
            fruits: 0,
            is_empty: true
        }
        const bucket_2: Bucket = {
            type: -1,
            fruits: 0,
            is_empty: true
        }
        for (let j = i; j < fruits.length; j += 1){
            if (bucket_1.is_empty || bucket_1.type === fruits[j]){
                bucket_1.fruits += 1;
                bucket_1.type = fruits[j];
                bucket_1.is_empty = false;
                continue;
            }
            if (bucket_2.is_empty || bucket_2.type === fruits[j]){
                bucket_2.fruits += 1;
                bucket_2.type = fruits[j];
                bucket_2.is_empty = false;
                continue;
            }
            break;
        }
        if (bucket_1.fruits + bucket_2.fruits > max_fruits) max_fruits = bucket_1.fruits + bucket_2.fruits;
    }

    return max_fruits;
};
