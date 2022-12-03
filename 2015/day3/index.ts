import { readTxtFileToString } from "../../modules";

const fileName: string = "input.txt";
const movesTxt: string = readTxtFileToString(fileName);

interface HousesPosition {
  [housePosition: string]: number; // position: how much presents
}

const findAllHousePositionPresents = (moves: string): HousesPosition => {
  const housesPosition: HousesPosition = {};
  const currPosition: number[] = [0, 0];
  housesPosition[currPosition.join("")] = 1;
  for (let i = 0; i < moves.length; i += 1) {
    // ^V<> : 4 type of moves
    if (moves[i] === "^") {
      // north
      currPosition[1] += 1;
    } else if (moves[i] === "v") {
      // south
      currPosition[1] -= 1;
    } else if (moves[i] === ">") {
      // east
      currPosition[0] += 1;
    } else {
      // west
      currPosition[0] -= 1;
    }
    if (!housesPosition[currPosition.join("")]) {
      housesPosition[currPosition.join("")] = 1;
      continue;
    }
    housesPosition[currPosition.join("")] += 1;
  }
  return housesPosition;
};
const findHouses = (moves: string): Set<string> => {
  const positionSet: Set<string> = new Set<string>();
  const currPosition: number[] = [0, 0];
  positionSet.add(currPosition.join(""));
  for (let i = 0; i < moves.length; i += 1) {
    // ^V<> : 4 type of moves
    if (moves[i] === "^") {
      // north
      currPosition[1] += 1;
    } else if (moves[i] === "v") {
      // south
      currPosition[1] -= 1;
    } else if (moves[i] === ">") {
      // east
      currPosition[0] += 1;
    } else {
      // west
      currPosition[0] -= 1;
    }
    positionSet.add(currPosition.join(""));
  }
  return positionSet;
};
// clue even odd moves, robot and santa
const findAllHousePositionPresents2 = (moves: string): HousesPosition => {
  const santaPosition: number[] = [0, 0];
  const robotPosition: number[] = [0, 0];
  const housesPosition: HousesPosition = {};
  housesPosition[santaPosition.join("")] = 2;
  let isSanta: boolean = true;
  for (let i = 0; i < moves.length; i += 1) {
    // ^V<> : 4 type of moves
    if (moves[i] === "^") {
      // north
      if (isSanta) {
        santaPosition[1] += 1;
      } else {
        robotPosition[1] += 1;
      }
    } else if (moves[i] === "v") {
      // south
      if (isSanta) {
        santaPosition[1] -= 1;
      } else {
        robotPosition[1] -= 1;
      }
    } else if (moves[i] === ">") {
      // east
      if (isSanta) {
        santaPosition[0] += 1;
      } else {
        robotPosition[0] += 1;
      }
    } else {
      // west
      if (isSanta) {
        santaPosition[0] -= 1;
      } else {
        robotPosition[0] -= 1;
      }
    }
    if (isSanta) {
      if (!housesPosition[santaPosition.join("")]) {
        housesPosition[santaPosition.join("")] = 1;
      } else {
        housesPosition[santaPosition.join("")] += 1;
      }
    } else {
      if (!housesPosition[robotPosition.join("")]) {
        housesPosition[robotPosition.join("")] = 1;
      } else {
        housesPosition[robotPosition.join("")] += 1;
      }
    }
    isSanta = !isSanta;
  }
  return housesPosition;
};
// IDK WTF IS WRONG WITH THIS CODE (BELOW)
const findAllHousePositionPresents3 = (moves: string): HousesPosition => {
  const housesPosition: HousesPosition = {};
  const santaMoves: string[] = moves.split("").filter((_, i) => i % 2 === 0);
  const robotMoves: string[] = moves.split("").filter((_, i) => i % 2 !== 0);
  const santa: HousesPosition = findAllHousePositionPresents(
    santaMoves.join("")
  );
  const robot: HousesPosition = findAllHousePositionPresents(
    robotMoves.join("")
  );
  for (let key of Object.keys(santa)) {
    if (!housesPosition[key]) {
      housesPosition[key] = 1;
      continue;
    }
    housesPosition[key] += 1;
  }
  for (let key of Object.keys(robot)) {
    if (!housesPosition[key]) {
      housesPosition[key] = 1;
      continue;
    }
    housesPosition[key] += 1;
  }
  return housesPosition;
};
// fourth time the charms
const findAllHoueses = (moves: string): number => {
  const santaMoves: Set<string> = findHouses(
    moves
      .split("")
      .filter((_, i) => i % 2 === 0)
      .join("")
  );
  const robotMoves: Set<string> = findHouses(
    moves
      .split("")
      .filter((_, i) => i % 2 !== 0)
      .join("")
  );
  robotMoves.forEach((position) => santaMoves.add(position));
  return santaMoves.size;
};
// const housePosition: HousesPosition = {};

// const currPosition: number[] = [0, 0];
// // add a present to the starting house
// housePosition[currPosition.join("")] = 1;

// for (let i = 0; i < "^v^v^v^v^v".length; i += 1) {
//   // ^V<> : 4 type of moves
//   if ("^v^v^v^v^v"[i] === "^") {
//     // north
//     currPosition[1] += 1;
//   } else if ("^v^v^v^v^v"[i] === "v") {
//     // south
//     currPosition[1] -= 1;
//   } else if ("^v^v^v^v^v"[i] === ">") {
//     // east
//     currPosition[0] += 1;
//   } else {
//     // west
//     currPosition[0] -= 1;
//   }
//   if (!housePosition[currPosition.join("")]) {
//     housePosition[currPosition.join("")] = 1;
//     continue;
//   }
//   housePosition[currPosition.join("")] += 1;
// }

const test: HousesPosition = findAllHousePositionPresents("^v^v^v^v^v");
console.log(test);
// console.log(housePosition);
console.log(findAllHousePositionPresents("^>v<"));
console.log(Object.keys(findAllHousePositionPresents("^>v<")).length);
console.log(Object.keys(findAllHousePositionPresents(movesTxt)).length);
console.log(Object.keys(findAllHousePositionPresents2("^v^v^v^v^v")).length);
console.log(Object.keys(findAllHousePositionPresents2("^>v<")).length);
console.log(Object.keys(findAllHousePositionPresents2("^v")).length);
console.log(Object.keys(findAllHousePositionPresents2("^v^v^v^v^v")).length);
console.log(Object.keys(findAllHousePositionPresents3(movesTxt)).length);
console.log(findHouses(movesTxt).size);
console.log(findAllHoueses(movesTxt));
