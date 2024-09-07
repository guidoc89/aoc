const fs = require("fs");
// const lines = fs.readFileSync("../test1.txt", "utf8").trim().split("\n");
// const lines = fs.readFileSync("../test2.txt", "utf8").trim().split("\n");
const lines = fs.readFileSync("../prod2.txt", "utf8").trim().split("\n");

// const inputString = "1abc2";

// // NOTE: Part 1)
// const parseLines = (line) => {
//   const digits = [];
//   [...line].map((letter, index) => {
//     if (!isNaN(Number(letter))) {
//       digits.push(letter);
//     }
//   });
//   return Number(digits[0] + digits[digits.length - 1]);
// };
//
// let totalSum = 0;
// lines.map((line) => {
//   totalSum += parseLines(line);
// });

// console.log(totalSum)

// lines.forEach(line => {
//
//     for (let i=0; i < line.length; i++) {
//         let digits = [];
//         try {
//             digits.push(Number(line[i]))
//         } catch {}
//
//         const currentResult = String(digits[0]) + String(digits[-1])
//         totalSum += Number(currentResult)
//     }
// })
// console.log(totalSum)
//
//
//
// NOTE: Part 2)
//
const inputString = "eightwothree";
const mappings = {
  "one": 1,
  "two": 2,
  "three": 3,
  "four": 4,
  "five": 5,
  "six": 6,
  "seven": 7,
  "eight": 8,
  "nine": 9,
};

let digits = [];
// console.log(
//   [...inputString].map((letter, index) => {
//     if (!isNaN(Number(letter))) {
//       digits.push(letter);
//     }
//     for (const [key, value] of Object.entries(mappings)) {
//       if (inputString.slice(index).startsWith(key)) {
//         digits.push(String(value));
//       }
//     }
//   }),
// );

// console.log(digits)
// const guido = Object.keys(mappings)
// for (const [key, value] of Object.entries(mappings)) {
//     console.log(key)

// }
//
// NOTE: Part 1)
const parseLinesPartTwo = (line) => {
    let digitsPartTwo = [];
  [...line].map((letter, index) => {
    if (!isNaN(Number(letter))) {
      digitsPartTwo.push(letter);
    } else {
      for (const [stringNumber, number] of Object.entries(mappings)) {
        if (line.slice(index).startsWith(stringNumber)) {
          digitsPartTwo.push(String(number));
        }
      }
    }
  });

  console.log(
    Number(digitsPartTwo[0] + digitsPartTwo[digitsPartTwo.length - 1]),
  );
  return Number(digitsPartTwo[0] + digitsPartTwo[digitsPartTwo.length - 1]);
};

let totalSumPartTwo = 0;
lines.map((line) => {
  totalSumPartTwo += parseLinesPartTwo(line);
});
console.log(totalSumPartTwo);
