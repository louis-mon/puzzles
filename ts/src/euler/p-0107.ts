import * as fs from "fs";

type Edge = { to: number; weight: number; from: number };
type Graph = Map<number, Edge[]>;

/**
 * Reads a file representing an adjacency matrix where each line `i` contains
 * comma-separated weights from vertex `i` to every other vertex `j`.
 * Constructs and returns a graph as an adjacency list.
 *
 * @param filePath - The path to the input file
 * @returns A promise that resolves to the graph represented as an adjacency list
 */
function readGraphFromMatrixFile(filePath: string): Graph {
  const graph: Graph = new Map();

  /*    const fileStream = fs.createReadStream(filePath);
    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity,
    });*/

  const rl = fs.readFileSync(filePath).toString().split("\n");

  let i = 0; // Vertex index for each line
  for (const line of rl) {
    const weights = line.split(",").map(Number);

    // Ensure vertex i is in the graph
    if (!graph.has(i)) {
      graph.set(i, []);
    }

    // Iterate through weights and add edges
    for (let j = 0; j < weights.length; j++) {
      const weight = weights[j];
      if (!Number.isNaN(weight)) {
        // Assuming weight 0 means no edge
        graph.get(i)!.push({ to: j, weight, from: i });
      }
    }

    i++; // Move to the next vertex
  }

  return graph;
}

/**
 * Union-Find (Disjoint Set) data structure to manage connected components.
 */
class UnionFind {
  private parent: number[];
  private rank: number[];

  constructor(size: number) {
    this.parent = Array.from({ length: size }, (_, i) => i);
    this.rank = Array(size).fill(0);
  }

  find(x: number): number {
    if (this.parent[x] !== x) {
      this.parent[x] = this.find(this.parent[x]);
    }
    return this.parent[x];
  }

  union(x: number, y: number): boolean {
    const rootX = this.find(x);
    const rootY = this.find(y);

    if (rootX === rootY) {
      return false; // x and y are already in the same set
    }

    // Union by rank
    if (this.rank[rootX] > this.rank[rootY]) {
      this.parent[rootY] = rootX;
    } else if (this.rank[rootX] < this.rank[rootY]) {
      this.parent[rootX] = rootY;
    } else {
      this.parent[rootY] = rootX;
      this.rank[rootX]++;
    }

    return true;
  }
}

/**
 * Constructs a Minimum Spanning Tree (MST) from graph G using Kruskal's algorithm.
 *
 * @param G - The input graph represented as an adjacency list
 * @returns A new graph E, which is the MST of G
 */
function constructMST(G: Graph): Graph {
  const E: Graph = new Map(); // Initialize empty graph E

  // Create a list of all edges in G
  const edges: Edge[] = [];
  for (const [from, edgeList] of G) {
    for (const edge of edgeList) {
      edges.push({ from, to: edge.to, weight: edge.weight });
    }
  }

  // Sort edges by weight
  edges.sort((a, b) => a.weight - b.weight);

  // Initialize Union-Find structure
  const uf = new UnionFind(G.size);

  for (const edge of edges) {
    if (uf.union(edge.from, edge.to)) {
      // If edge connects two disjoint sets, add it to E
      if (!E.has(edge.from)) {
        E.set(edge.from, []);
      }
      if (!E.has(edge.to)) {
        E.set(edge.to, []);
      }
      E.get(edge.from)!.push({
        from: edge.from,
        to: edge.to,
        weight: edge.weight,
      });
      E.get(edge.to)!.push({
        from: edge.to,
        to: edge.from,
        weight: edge.weight,
      });

      // Stop if all vertices are connected (i.e., the graph is fully connected)
      if (E.size === G.size && edges.length === E.size - 1) {
        break;
      }
    }
  }

  return E;
}

function sumOfAllWeights(graph: Graph): number {
  const visited = new Set<string>();
  let totalWeight = 0;

  for (const [from, edges] of graph) {
    for (const edge of edges) {
      const edgeKey = `${Math.min(from, edge.to)}-${Math.max(from, edge.to)}`;
      if (!visited.has(edgeKey)) {
        totalWeight += edge.weight;
        visited.add(edgeKey);
      }
    }
  }

  return totalWeight;
}

const org = readGraphFromMatrixFile("src/euler/p-0107.txt");
const mst = constructMST(org);
console.log(sumOfAllWeights(org) - sumOfAllWeights(mst));
