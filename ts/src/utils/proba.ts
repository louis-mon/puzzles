/**
 * Calculate the factorial of a number.
 * @param n - The number to calculate the factorial for.
 * @returns The factorial of the given number.
 */
export function factorial(n: number): number {
  if (n < 0) {
    throw new Error("Factorial is not defined for negative numbers.");
  }
  let result = 1;
  for (let i = 2; i <= n; i++) {
    result *= i;
  }
  return result;
}

/**
 * Calculate the binomial coefficient (n choose k).
 * @param n - The total number of items.
 * @param k - The number of items to choose.
 * @returns The binomial coefficient.
 */
export function binomial(n: number, k: number): number {
  if (k < 0 || k > n) {
    return 0;
  }
  return factorial(n) / (factorial(k) * factorial(n - k));
}
