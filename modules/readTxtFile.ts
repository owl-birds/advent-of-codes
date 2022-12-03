import * as fs from "fs";

const readTxtFileToArr = (fileName: string): string[] => {
  return fs
    .readFileSync(fileName)
    .toString("utf8")
    .split("\n")
    .filter((currString) => currString.length > 2);
};

const readTxtFileToString = (fileName: string): string => {
  return fs.readFileSync(fileName).toString("utf8");
};

export { readTxtFileToArr, readTxtFileToString };
