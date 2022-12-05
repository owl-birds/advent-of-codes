import { readTxtFileToString, readTxtFileToArr } from "../../modules/index.ts";

const text: string = readTxtFileToString("./input.txt");
const instructionArr: string[] = readTxtFileToArr("./input.txt");
// console.log(instructionArr[instructionArr.length - 1]);

const grid: boolean[][] = [];
for (let i = 0; i < 1000; i += 1) {
  grid.push(new Array(1000).fill(false));
}
// false: the lioghts off, true: the lights on
// console.log(grid);

// starting point; all the lights are off

// TYPES
interface Instruction {
  do: number;
  gridSize: number[];
}

// some methods/functions
const interpretInstruction = (instruction: string): Instruction => {
  // 0 : turn off, 1 : tturn on, 2 : toggle
  const result: Instruction = {
    do: /turn on/.test(instruction) ? 1 : /toggle/.test(instruction) ? 2 : 0,
    gridSize: [],
  };
  // use Regex or mayber not, or just loop it
  let tempIdx: number = 0;
  while (tempIdx < instruction.length) {
    //
    if (
      instruction[tempIdx] !== "" &&
      instruction[tempIdx] !== " " &&
      Number(instruction[tempIdx]) >= 0
    ) {
      //
      //   console.log(instruction[tempIdx], tempIdx);
      let tempNum: string = ``;
      while (
        Number(instruction[tempIdx]) ||
        Number(instruction[tempIdx]) === 0
      ) {
        tempNum = `${tempNum}${instruction[tempIdx]}`;
        tempIdx += 1;
      }
      result.gridSize.push(Number(tempNum));
      continue;
    }
    tempIdx += 1;
  }
  return result;
};
const checkOnLights: (grid: boolean[][]) => number = (
  grid: boolean[][]
): number => {
  let countOnLights: number = 0;
  for (let i = 0; i < grid.length; i += 1) {
    for (let j = 0; j < grid[0].length; j += 1) {
      if (grid[i][j]) countOnLights += 1;
    }
  }
  return countOnLights;
};
const traverseGrid = (grid: boolean[][], instruction: Instruction): void => {
  //
  const startRow: number = instruction.gridSize[0];
  const endRow: number = instruction.gridSize[2];
  const startCol: number = instruction.gridSize[1];
  const endCol: number = instruction.gridSize[3];
  const command: number = instruction.do;
  if (command === 1) {
    for (let row = startRow; row <= endRow; row += 1) {
      for (let col = startCol; col <= endCol; col += 1) {
        grid[row][col] = true;
      }
    }
  } else if (command === 0) {
    for (let row = startRow; row <= endRow; row += 1) {
      for (let col = startCol; col <= endCol; col += 1) {
        grid[row][col] = false;
      }
    }
  } else {
    for (let row = startRow; row <= endRow; row += 1) {
      for (let col = startCol; col <= endCol; col += 1) {
        grid[row][col] = !grid[row][col];
      }
    }
  }
};
const doInstructionReturnOn = (
  grid: boolean[][],
  instructionArr: string[]
): number => {
  //
  for (let i = 0; i < instructionArr.length; i += 1) {
    traverseGrid(grid, interpretInstruction(instructionArr[i]));
  }
  return checkOnLights(grid);
};
console.log(instructionArr[0]);
console.log(interpretInstruction(instructionArr[0]));
console.log(instructionArr[10]);
console.log(interpretInstruction(instructionArr[10]));
console.log(instructionArr[10]);
console.log("turn on 0,0 through 999,999");
console.log(interpretInstruction("turn on 0,0 through 999,999"));
console.log(instructionArr[100]);
console.log(interpretInstruction(instructionArr[100]));
console.log(instructionArr[200]);
console.log(interpretInstruction(instructionArr[200]));
// doInstruction(grid, instructionArr);
// traverseGrid(grid, interpretInstruction("turn on 0,0 through 999,999"));
// traverseGrid(grid, interpretInstruction("toggle 0,0 through 999,0"));
// traverseGrid(grid, interpretInstruction("turn off 499,499 through 500,500"));

// metada
console.log("how many lights are on: ", checkOnLights(grid));
console.log("instructions: ", instructionArr.length);
// traverseGrid(grid, interpretInstruction("turn on 499,499 through 500,500"));
// console.log("how many lights are on: ", checkOnLights(grid));
// // console.log(interpretInstruction("toggle 0,0 through 999,0"));
// traverseGrid(grid, interpretInstruction("turn off 499,499 through 500,500"));
// console.log("how many lights are on: ", checkOnLights(grid));
// traverseGrid(grid, interpretInstruction("toggle 0,0 through 99,0"));
// console.log("how many lights are on: ", checkOnLights(grid));
console.log(doInstructionReturnOn(grid, instructionArr));

// part two
// santa suck ass he fecking suck at writing instruction
const grid2: number[][] = [];
for (let i = 0; i < 1000; i += 1) {
  grid2.push(new Array(1000).fill(0));
}
const checkBrightness: (grid: number[][]) => number = (
  grid: number[][]
): number => {
  let countOnLights: number = 0;
  for (let i = 0; i < grid.length; i += 1) {
    for (let j = 0; j < grid[0].length; j += 1) {
      if (grid[i][j]) countOnLights += grid[i][j];
    }
  }
  return countOnLights;
};
const traverseGrid2 = (grid: number[][], instruction: Instruction): void => {
  //
  const startRow: number = instruction.gridSize[0];
  const endRow: number = instruction.gridSize[2];
  const startCol: number = instruction.gridSize[1];
  const endCol: number = instruction.gridSize[3];
  const command: number = instruction.do;
  if (command === 1) {
    for (let row = startRow; row <= endRow; row += 1) {
      for (let col = startCol; col <= endCol; col += 1) {
        grid[row][col] += 1;
      }
    }
  } else if (command === 0) {
    for (let row = startRow; row <= endRow; row += 1) {
      for (let col = startCol; col <= endCol; col += 1) {
        if (grid[row][col] > 0) grid[row][col] -= 1;
      }
    }
  } else {
    for (let row = startRow; row <= endRow; row += 1) {
      for (let col = startCol; col <= endCol; col += 1) {
        grid[row][col] += 2;
      }
    }
  }
};
const doInstructionReturnBrightness = (
  grid: number[][],
  instructionArr: string[]
): number => {
  //
  for (let i = 0; i < instructionArr.length; i += 1) {
    traverseGrid2(grid, interpretInstruction(instructionArr[i]));
  }
  return checkBrightness(grid);
};
// traverseGrid2(grid2, interpretInstruction("toggle 0,0 through 999,999"));
// console.log(checkBrightness(grid2));
console.log(doInstructionReturnBrightness(grid2, instructionArr));
