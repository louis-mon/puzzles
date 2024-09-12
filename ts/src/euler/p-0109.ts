import _ from "lodash";
import { benchmark } from "../utils/benchmark";

const possibleMoves: Array<[string, number, number]> = _.range(1, 21)
  .flatMap(
    (i: number): Array<[string, number, number]> => [
      [`S${i}`, i, 1],
      [`D${i}`, i, 2],
      [`T${i}`, i, 3],
    ],
  )
  .concat([
    ["S25", 25, 1],
    ["D25", 25, 2],
  ]);

function generateCombi() {
  const visited = new Set<string>();

  function generateDartsCombi({
    prefix,
    tot,
  }: {
    prefix: string[];
    tot: number;
  }) {
    possibleMoves.forEach(([s, p, n]) => {
      if (prefix.length === 0 && n !== 2) {
        return;
      }
      const newTot = tot + p * n;
      const newP = prefix.concat(s);
      const [fst, ...rP] = newP;
      const key = [fst].concat(rP.sort()).join("");
      if (visited.has(key) || newTot >= 100) {
        return;
      }
      visited.add(key);
      if (newP.length < 3) {
        generateDartsCombi({ prefix: newP, tot: newTot });
      }
    });
  }

  generateDartsCombi({ prefix: [], tot: 0 });

  console.log(visited.size);
}

benchmark(() => generateCombi());
