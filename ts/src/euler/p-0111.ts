import { countBy, last, range } from "lodash";
import { benchmark } from "../utils/benchmark";

function test() {
  for (let i = 0; i < 1e10; ++i) {}
}

const problemInput = 1e9;

const countByDigits: Array<Array<bigint>> = range(10).map((i) =>
  range(11).map((j) => BigInt(0)),
);

function computeForN(n: number) {
  if (n < problemInput) {
    return BigInt(0);
  }
  const s = n.toString();
  const c = countBy(s.split(""));
  for (let i = 0; i < 10; i++) {
    countByDigits[i][c[i]] = (countByDigits[i][c[i]] ?? BigInt(0)) + BigInt(n);
  }
}

function segmentedSieve(limit: number) {
  const segmentSize = Math.floor(Math.sqrt(limit)) + 1;

  // Step 1: Use simple sieve to find primes up to √limit
  const primes = simpleSieve(segmentSize);

  // Step 2: Segmented Sieve for range [√limit, limit]
  const lowLimit = segmentSize;
  const highLimit = limit;

  // Initialize the segment array to keep track of primes in the range
  let sieve = new Array(segmentSize).fill(true);

  let low = lowLimit;
  let high = Math.min(low + segmentSize - 1, highLimit);

  primes.map(computeForN);

  // Process segments
  while (low <= highLimit) {
    sieve.fill(true); // Reset the sieve for each segment

    // For each prime, mark its multiples in the current segment
    for (let prime of primes) {
      let start = Math.max(prime * prime, Math.ceil(low / prime) * prime);

      for (let j = start; j <= high; j += prime) {
        sieve[j - low] = false;
      }
    }

    // Collect primes from the current segment
    for (let i = low; i <= high; i++) {
      if (sieve[i - low]) {
        computeForN(i);
      }
    }

    // Move to the next segment
    low += segmentSize;
    high = Math.min(low + segmentSize - 1, highLimit);
  }
  return countByDigits
    .map(
      (counts) =>
        counts
          .slice()
          .reverse()
          .find((v) => v > 0)!,
    )
    .reduce((acc, v) => acc + v, BigInt(0));
}

// Simple sieve to find all primes up to √limit
function simpleSieve(limit: number) {
  const sieve = new Array(limit + 1).fill(true);
  sieve[0] = sieve[1] = false;

  for (let i = 2; i * i <= limit; i++) {
    if (sieve[i]) {
      for (let j = i * i; j <= limit; j += i) {
        sieve[j] = false;
      }
    }
  }

  const primes = [];
  for (let i = 2; i <= limit; i++) {
    if (sieve[i]) {
      primes.push(i);
    }
  }
  return primes;
}

benchmark(() => console.log(segmentedSieve(problemInput * 10))); // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, ...]
