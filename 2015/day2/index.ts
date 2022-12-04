// import * as fs from "fs";

// DENO
import { readTxtFileToArr } from "../../modules/index.ts";

// const readTxtFile = (fileName: string): string[] => {
//   return fs
//     .readFileSync(fileName)
//     .toString("utf8")
//     .split("\n")
//     .filter((currString) => currString.length > 2);
// };
interface Present {
  l: number;
  w: number;
  h: number;
}
const getTheMeasure = (sizeString: string): Present => {
  const size: string[] = sizeString.split("x");
  return { l: Number(size[0]), w: Number(size[1]), h: Number(size[2]) };
};
const totalWrappingPaperEachPresent = (measurements: Present): number => {
  const { l, w, h } = measurements;
  const min: number = Math.min(l * w, l * h, w * h);
  const area: number = 2 * l * w + 2 * w * h + 2 * l * h;
  return area + min;
};
const totalRibbonEachPresent = (measurements: Present): number => {
  const { l, w, h } = measurements;
  const cubic: number = l * w * h;
  const tempArr: number[] = [l, w, h].sort((a, b) => a - b);
  const wrap: number = 2 * tempArr[0] + 2 * tempArr[1];
  return cubic + wrap;
};

const fileName: string = "input.txt";
const fileTxt: string[] = readTxtFileToArr(fileName);
const presents: Present[] = fileTxt.map((present) => getTheMeasure(present));

let totalWrappingPaper: number = 0;
let totalRibbon: number = 0;

for (let i = 0; i < presents.length; i += 1) {
  totalWrappingPaper += totalWrappingPaperEachPresent(presents[i]);
  totalRibbon += totalRibbonEachPresent(presents[i]);
}
//console.log(totalWrappingPaperEachPresent({ l: 2, w: 3, h: 4 }));
console.log(totalWrappingPaper);
//console.log(totalRibbonEachPresent({ l: 2, w: 3, h: 4 }));
//console.log(totalRibbonEachPresent({ l: 1, w: 1, h: 10 }));
console.log(totalRibbon);
