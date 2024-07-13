import { isEqual, range, sortBy } from "lodash";
import { benchmark } from "../utils/benchmark";

function isPandigital(s: string) {
  return isEqual(sortBy(s.split("").map((x) => +x)), range(1, 10));
}

const phiLog = Math.log10((1 + Math.sqrt(5)) / 2);
const sqrt5log = Math.log10(Math.sqrt(5));

function firstDigitsOfFibonacci(n: number): number {
  const flog = n * phiLog - sqrt5log;
  const nDigits = Math.floor(flog);
  const mantissa = flog - nDigits;
  return Math.floor(Math.pow(10, 8 + mantissa));
}

function solution() {
  let f0 = 1;
  let f = 1;
  let k = 2;
  let p = 1_000_000_000;
  while (true) {
    ++k;
    const f1 = f0 + f;
    f0 = f % p;
    f = f1 % p;
    if (isPandigital(f.toString())) {
      if (isPandigital(firstDigitsOfFibonacci(k).toString())) {
        return k;
      }
    }
  }
}

console.log(benchmark(() => solution()));
