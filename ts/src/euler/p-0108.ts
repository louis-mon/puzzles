import * as _ from "lodash";

function checkN(n: number) {
  let count = 0;
  for (let x = n + 1; x <= n * 2; ++x) {
    if ((n * x) % (x - n) === 0) {
      ++count;
    }
  }
  return count;
}

function checkN2(n: number) {
  const solutionsX = _.range(n + 1, n * (n + 1) + 1).filter(
    (x) => (n * x) % (x - n) === 0,
  );
  const solutions = solutionsX
    .map((x) => [x, (n * x) / (x - n)])
    .filter(([x, y]) => x >= y);
  console.log(`for ${n}`, solutions.length);
  return solutions.length;
}

const N = 2 * 2 * 3 * 3 * 5 * 7 * 11 * 13;
//console.log(N, checkN(N));
/*for (let n  = 1; n < 1000000; ++n) {
    if (checkN(n) > 300) {
        console.log(n);
        break;
    }
}*/

const primes = [
  2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
];

function computeNbSols(ai: number[]): number {
  return (ai.reduce((a, b) => (2 * b + 1) * a, 1) + 1) / 2;
}

function testCombi(ai: number[]) {
  const nbSolsTh = computeNbSols(ai);
  const n = ai.reduce((a, b, i) => a * primes[i] ** b, 1);
  const nbSolsReal = checkN(n);
  if (nbSolsReal !== nbSolsTh) {
    console.log(n, ai, nbSolsTh, nbSolsReal);
  }
}

function multiTestCombi(ai: number[]) {
  const permute = (arr: number[], m: number[] = []) => {
    if (arr.length === 0) {
      testCombi(m);
    } else {
      const seen = new Set<number>();
      for (let i = 0; i < arr.length; i++) {
        if (!seen.has(arr[i])) {
          seen.add(arr[i]);
          let curr = arr.slice();
          let next = curr.splice(i, 1);
          permute(curr.slice(), m.concat(next));
        }
      }
    }
  };

  permute(ai);
}

type Node = number[]; // Node is now an array of numbers

function sign(n: bigint) {
  if (n === 0n) {
    return 0;
  } else if (n < 0n) {
    return -1;
  } else {
    return 1;
  }
}

class PriorityQueue {
  private heap: { node: Node; priority: bigint }[] = [];

  enqueue(node: Node, priority: bigint) {
    this.heap.push({ node, priority });
    this.heap.sort((a, b) => sign(a.priority - b.priority));
  }

  dequeue(): Node | undefined {
    return this.heap.shift()?.node;
  }

  isEmpty(): boolean {
    return this.heap.length === 0;
  }
}

function generateNodesByIncreasingCost(
  startNode: Node,
  getNeighbors: (node: Node) => Iterable<Node>,
  getCost: (node: Node) => bigint,
  stopAfter: (node: Node) => boolean,
): void {
  const visited: Set<string> = new Set(); // Using a set of strings to represent visited nodes
  const priorityQueue = new PriorityQueue();

  // Start with the initial node
  priorityQueue.enqueue(startNode, getCost(startNode));
  visited.add(JSON.stringify(startNode)); // Convert the array to a string for proper comparison

  while (!priorityQueue.isEmpty()) {
    const currentNode = priorityQueue.dequeue()!;

    if (stopAfter(currentNode)) {
      break;
    }

    // Add unvisited neighbors to the priority queue
    for (const neighbor of getNeighbors(currentNode)) {
      const neighborKey = JSON.stringify(neighbor);
      if (!visited.has(neighborKey)) {
        visited.add(neighborKey);
        priorityQueue.enqueue(neighbor, getCost(neighbor));
      }
    }
  }
}

type SolForN = { node: Node; nbSols: number; n: bigint; nDigits: number };

function bestSolForNDigits(nDigits: number): SolForN {
  const start = _.range(nDigits).map((i) => 1);

  let res: SolForN | null = null;

  generateNodesByIncreasingCost(
    start,
    (node) =>
      _.range(node.length)
        .map((i) => node.map((v, j) => (j === i ? v + 1 : v)))
        .map((node) => _.sortBy(node, (x) => -x) as number[]),
    nFor,
    (node) => {
      if (computeNbSols(node) > 4000000000) {
        res = { node, nbSols: computeNbSols(node), n: nFor(node), nDigits };
        return true;
      }
      return false;
    },
  );

  return res!;
}

function nFor(ai: number[]): bigint {
  return ai.reduce((a, b, i) => a * BigInt(primes[i] ** b), 1n);
}

console.log(_.sortBy(_.range(13, 21).map(bestSolForNDigits), (s) => s.n));
