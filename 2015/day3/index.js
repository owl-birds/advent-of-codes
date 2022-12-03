"use strict";
exports.__esModule = true;
var modules_1 = require("../../modules");
var fileName = "input.txt";
var movesTxt = (0, modules_1.readTxtFileToString)(fileName);
var findAllHousePositionPresents = function (moves) {
    var housesPosition = {};
    var currPosition = [0, 0];
    housesPosition[currPosition.join("")] = 1;
    for (var i = 0; i < moves.length; i += 1) {
        // ^V<> : 4 type of moves
        if (moves[i] === "^") {
            // north
            currPosition[1] += 1;
        }
        else if (moves[i] === "v") {
            // south
            currPosition[1] -= 1;
        }
        else if (moves[i] === ">") {
            // east
            currPosition[0] += 1;
        }
        else {
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
var findHouses = function (moves) {
    var positionSet = new Set();
    var currPosition = [0, 0];
    positionSet.add(currPosition.join(""));
    for (var i = 0; i < moves.length; i += 1) {
        // ^V<> : 4 type of moves
        if (moves[i] === "^") {
            // north
            currPosition[1] += 1;
        }
        else if (moves[i] === "v") {
            // south
            currPosition[1] -= 1;
        }
        else if (moves[i] === ">") {
            // east
            currPosition[0] += 1;
        }
        else {
            // west
            currPosition[0] -= 1;
        }
        positionSet.add(currPosition.join(""));
    }
    return positionSet;
};
// clue even odd moves, robot and santa
var findAllHousePositionPresents2 = function (moves) {
    var santaPosition = [0, 0];
    var robotPosition = [0, 0];
    var housesPosition = {};
    housesPosition[santaPosition.join("")] = 2;
    var isSanta = true;
    for (var i = 0; i < moves.length; i += 1) {
        // ^V<> : 4 type of moves
        if (moves[i] === "^") {
            // north
            if (isSanta) {
                santaPosition[1] += 1;
            }
            else {
                robotPosition[1] += 1;
            }
        }
        else if (moves[i] === "v") {
            // south
            if (isSanta) {
                santaPosition[1] -= 1;
            }
            else {
                robotPosition[1] -= 1;
            }
        }
        else if (moves[i] === ">") {
            // east
            if (isSanta) {
                santaPosition[0] += 1;
            }
            else {
                robotPosition[0] += 1;
            }
        }
        else {
            // west
            if (isSanta) {
                santaPosition[0] -= 1;
            }
            else {
                robotPosition[0] -= 1;
            }
        }
        if (isSanta) {
            if (!housesPosition[santaPosition.join("")]) {
                housesPosition[santaPosition.join("")] = 1;
            }
            else {
                housesPosition[santaPosition.join("")] += 1;
            }
        }
        else {
            if (!housesPosition[robotPosition.join("")]) {
                housesPosition[robotPosition.join("")] = 1;
            }
            else {
                housesPosition[robotPosition.join("")] += 1;
            }
        }
        isSanta = !isSanta;
    }
    return housesPosition;
};
// IDK WTF IS WRONG WITH THIS CODE (BELOW)
var findAllHousePositionPresents3 = function (moves) {
    var housesPosition = {};
    var santaMoves = moves.split("").filter(function (_, i) { return i % 2 === 0; });
    var robotMoves = moves.split("").filter(function (_, i) { return i % 2 !== 0; });
    var santa = findAllHousePositionPresents(santaMoves.join(""));
    var robot = findAllHousePositionPresents(robotMoves.join(""));
    for (var _i = 0, _a = Object.keys(santa); _i < _a.length; _i++) {
        var key = _a[_i];
        if (!housesPosition[key]) {
            housesPosition[key] = 1;
            continue;
        }
        housesPosition[key] += 1;
    }
    for (var _b = 0, _c = Object.keys(robot); _b < _c.length; _b++) {
        var key = _c[_b];
        if (!housesPosition[key]) {
            housesPosition[key] = 1;
            continue;
        }
        housesPosition[key] += 1;
    }
    return housesPosition;
};
// fourth time the charms
var findAllHoueses = function (moves) {
    var santaMoves = findHouses(moves
        .split("")
        .filter(function (_, i) { return i % 2 === 0; })
        .join(""));
    var robotMoves = findHouses(moves
        .split("")
        .filter(function (_, i) { return i % 2 !== 0; })
        .join(""));
    robotMoves.forEach(function (position) { return santaMoves.add(position); });
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
var test = findAllHousePositionPresents("^v^v^v^v^v");
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
