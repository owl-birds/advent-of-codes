function generate(numRows: number): number[][] {
    if (numRows === 0) return [];
    const triangle: number[][] = [[1],[1,1]];
    if (numRows === 1) return [[1]];
    if (numRows === 2) return triangle;
    
    let curr_row = 2;

    while (curr_row < numRows){
        const prev_row = triangle[curr_row-1];
        const new_row = [1,1];
        let idx = 0;
        let next_idx = 1;
        while (next_idx <= prev_row.length-1){
            const add = prev_row[idx] + prev_row[next_idx];
            new_row.splice(idx+1, 0, add);
            idx += 1;
            next_idx += 1;
        }
        triangle.push(new_row);
        curr_row += 1;
    }

    return triangle
};
