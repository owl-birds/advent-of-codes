"use strict";
exports.__esModule = true;
exports.readTxtFileToString = exports.readTxtFileToArr = void 0;
var fs = require("fs");
var readTxtFileToArr = function (fileName) {
    return fs
        .readFileSync(fileName)
        .toString("utf8")
        .split("\n")
        .filter(function (currString) { return currString.length > 2; });
};
exports.readTxtFileToArr = readTxtFileToArr;
var readTxtFileToString = function (fileName) {
    return fs.readFileSync(fileName).toString("utf8");
};
exports.readTxtFileToString = readTxtFileToString;
