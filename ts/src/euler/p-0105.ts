import { fromPairs, range, Dictionary, sum, last, ceil, sortBy } from "lodash";
import fs from "fs";
import { benchmark } from "../utils/benchmark";

function isSpecialSubSet(rawValues: number[]): boolean {
  const values = sortBy(rawValues);
  const nPack = Math.floor(values.length / 2);
  const sumsBySize = range(nPack).reduce<Record<number, Set<number>>>(
    (acc, i) => ({
      ...acc,
      [i + 1]: new Set<number>(),
    }),
    {},
  );

  function testSum(indices: number[]): boolean {
    if (indices.length) {
      const s = sum(indices.map((i) => values[i]));
      if (sumsBySize[indices.length].has(s)) {
        return false;
      }
      sumsBySize[indices.length].add(s);
    }
    if (indices.length === nPack) {
      return true;
    }
    const lastI = last(indices) ?? -1;
    return range(lastI + 1, values.length).every((i) =>
      testSum([...indices, i]),
    );
  }

  function testIneq(n: number) {
    return (
      sum(range(n + 2).map((i) => values[i])) >
      sum(range(values.length - n - 1, values.length).map((i) => values[i]))
    );
  }

  return range(nPack + 1).every((i) => testIneq(i)) && testSum([]);
}

let listOfInput = fs
  .readFileSync("src/euler/p-0105.txt")
  .toString()
  .split("\n")
  .map((s) => s.split(",").map((i) => +i));
const res = benchmark(() =>
  sum(listOfInput.filter(isSpecialSubSet).map((s) => sum(s))),
);

console.log(res);
