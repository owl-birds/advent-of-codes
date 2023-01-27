function minimizedMaximum(n: number, quantities: number[]): number {
    // genius testing right here :: ALL HAIL THE CONSOLE.LOG(); im joking
    // console.log("2/store", is_enough(2, quantities, n));
    // console.log("3/store", is_enough(3, quantities, n));
    // console.log("4/store", is_enough(4, quantities, n));
    // console.log("5/store", is_enough(5, quantities, n));
    // console.log("6/store", is_enough(6, quantities, n));
    // console.log("100000/store", is_enough(100000, quantities, n));
    // console.log("10000/store", is_enough(10000, quantities, n));
    let min_prod: number = 0;
    let max_prod: number = Math.max(...quantities);

    while (min_prod < max_prod){
        const mid: number = Math.floor((min_prod+max_prod)/2);
        if (is_enough(mid, quantities, n)){
            max_prod = mid;
            continue;
        }
        min_prod = mid + 1;
    }
    return min_prod;
};

const is_enough = (quantity: number, products: number[], stores: number): boolean=>{
    let prod_idx: number = 0;
    let count_store: number = 0;
    while (prod_idx < products.length){
        let prod_quan: number = products[prod_idx];
        count_store += Math.ceil(prod_quan/quantity); // why ceil, cause each store can only have one product or non at all, but can have any amount of it
        // console.log(count_store, prod_quan/quantity);
        if (count_store > stores) return false;
        // while (prod_quan > 0){
        //     prod_quan -= quantity;
        //     count_store += 1;
        //     if (count_store > stores) return false;
        // }
        prod_idx += 1;
    }
    return true;
}
