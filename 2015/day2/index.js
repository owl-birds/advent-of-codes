"use strict";
exports.__esModule = true;
var fs = require("fs");
var readTxtFile = function (fileName) {
    return fs
        .readFileSync(fileName)
        .toString("utf8")
        .split("\n")
        .filter(function (currString) { return currString.length > 2; });
};
var getTheMeasure = function (sizeString) {
    var size = sizeString.split("x");
    return { l: Number(size[0]), w: Number(size[1]), h: Number(size[2]) };
};
var totalWrappingPaperEachPresent = function (measurements) {
    var l = measurements.l, w = measurements.w, h = measurements.h;
    var min = Math.min(l * w, l * h, w * h);
    var area = 2 * l * w + 2 * w * h + 2 * l * h;
    return area + min;
};
var totalRibbonEachPresent = function (measurements) {
    var l = measurements.l, w = measurements.w, h = measurements.h;
    var cubic = l * w * h;
    var tempArr = [l, w, h].sort(function (a, b) { return a - b; });
    var wrap = 2 * tempArr[0] + 2 * tempArr[1];
    return cubic + wrap;
};
var fileName = "input.txt";
var fileTxt = readTxtFile(fileName);
var presents = fileTxt.map(function (present) { return getTheMeasure(present); });
var totalWrappingPaper = 0;
var totalRibbon = 0;
for (var i = 0; i < presents.length; i += 1) {
    totalWrappingPaper += totalWrappingPaperEachPresent(presents[i]);
    totalRibbon += totalRibbonEachPresent(presents[i]);
}
//console.log(totalWrappingPaperEachPresent({ l: 2, w: 3, h: 4 }));
console.log(totalWrappingPaper);
//console.log(totalRibbonEachPresent({ l: 2, w: 3, h: 4 }));
//console.log(totalRibbonEachPresent({ l: 1, w: 1, h: 10 }));
console.log(totalRibbon);
