"use strict";
exports.__esModule = true;
var fs = require("fs");
var fileName = "input-1.txt";
var fileString = fs.readFileSync(fileName).toString("utf8");
var currFloor = 0;
for (var i = 0; i < fileString.length; i += 1) {
    if (fileString[i] === "(")
        currFloor += 1;
    else
        currFloor -= 1;
}
console.log(currFloor);
