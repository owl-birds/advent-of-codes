import * as fs from "fs";

const fileName: string = "input-1.txt";
const fileString: string = fs.readFileSync(fileName).toString("utf8");

let currFloor: number = 0;

for (let i = 0; i < fileString.length; i += 1) {
  if (fileString[i] === "(") currFloor += 1;
  else currFloor -= 1;
}
console.log(currFloor);
