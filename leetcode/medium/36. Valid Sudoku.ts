function isValidSudoku(board: string[][]): boolean {
  const unique_nums: Set<string> = new Set();
  // row first
  for (const row of board) {
    for (const num of row) {
      if (num === ".") continue;
      if (unique_nums.has(num)) {
        console.log(row);
        return false;
      }
      unique_nums.add(num);
    }
    unique_nums.clear();
  }

  // column second
  unique_nums.clear();
  for (let c = 0; c < 9; c += 1) {
    for (let r = 0; r < 9; r += 1) {
      if (board[r][c] === ".") continue;
      if (unique_nums.has(board[r][c])) {
        console.log("column:", c);
        return false;
      }
      unique_nums.add(board[r][c]);
    }
    unique_nums.clear();
  }

  // boxes
  unique_nums.clear();
  let box_start_row = 0;
  while (box_start_row < 9) {
    let box_start_col = 0;
    while (box_start_col < 9) {
      // iterate num in each box
      // console.log(box_start_col)
      for (let r = box_start_row; r < box_start_row + 3; r += 1) {
        for (let c = box_start_col; c < box_start_col + 3; c += 1) {
          if (board[r][c] === ".") continue;
          if (unique_nums.has(board[r][c])) return false;
          unique_nums.add(board[r][c]);
        }
      }
      unique_nums.clear();
      box_start_col += 3;
    }
    // console.log(box_start_row)
    box_start_row += 3;
  }

  return true;
}
