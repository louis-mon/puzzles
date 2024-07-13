import { range } from "lodash";
import { matrixPower } from "./matrix";

// Function to compute the nth Fibonacci number using matrix exponentiation
function fibonacci(n: number): bigint {
  const baseMatrix: bigint[][] = [
    [BigInt(1), BigInt(1)],
    [BigInt(1), BigInt(0)],
  ];

  if (n === 0) {
    return BigInt(0);
  }

  const resultMatrix = matrixPower(baseMatrix, n);
  return resultMatrix[0][1]; // F(n) is in the position (0, 1)
}
