// import * as fs from "fs"; // nodejs

const readTxtFileToArr = (fileName: string): string[] => {
  const text: string = Deno.readTextFileSync(fileName);
  // return fs
  //   .readFileSync(fileName)
  //   .toString("utf8")
  //   .split("\n")
  //   .filter((currString: string) => currString.length > 2);
  return text.split("\n").filter((currString: string) => currString.length > 2);
};

const readTxtFileToString = (fileName: string): string => {
  // return fs.readFileSync(fileName).toString("utf8");
  const text: string = Deno.readTextFileSync(fileName);
  return text;
};

export { readTxtFileToArr, readTxtFileToString };
