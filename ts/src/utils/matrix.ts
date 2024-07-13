import { range } from "lodash";

export function multiplyMatrix(a: bigint[][], b: bigint[][]): bigint[][] {
  return a.map((aRow, row) =>
    range(b[0].length).map((bColIdx) =>
      aRow.reduce(
        (acc, cell, rowIdx) => acc + cell * b[rowIdx][bColIdx],
        BigInt(0),
      ),
    ),
  );
}

// Helper function to perform matrix exponentiation
export function matrixPower(matrix: bigint[][], n: number): bigint[][] {
  if (n === 1) {
    return matrix;
  }

  const rem = n % 2;
  const base = (n - rem) / 2;
  const halfPower = matrixPower(matrix, base);
  const halfSquared = multiplyMatrix(halfPower, halfPower);
  return rem === 0 ? halfSquared : multiplyMatrix(halfSquared, matrix);
}
