import { difference, last, max, min, range } from "lodash";
import { benchmark } from "../utils/benchmark";

function solution(n: number) {
  let count = 0;
  const nPack = Math.floor(n / 2);

  function testFromSet(
    toMatch: number[],
    choices: number[],
    isInf: boolean,
  ): void {
    if (toMatch.length === 0 && isInf) {
      ++count;
      return;
    }
    choices.forEach((ci, i) => {
      const isCiInf = ci < toMatch[0];
      testFromSet(toMatch.slice(1), choices.slice(i + 1), isCiInf || isInf);
    });
  }

  function testSum(indices: number[]): void {
    if (indices.length >= 2) {
      const choices: number[] = difference(
        range(min(indices) as number, n),
        indices,
      );
      testFromSet(indices, choices, false);
    }
    if (indices.length === nPack) {
      return;
    }
    const lastI = last(indices) ?? -1;
    range(lastI + 1, n).forEach((i) => testSum([...indices, i]));
  }

  testSum([]);
  return count;
}

console.log(benchmark(() => solution(12)));
