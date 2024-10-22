type Token = string;

type Command = {
  c: number;
  v: Token;
};

type State = {
  c: Array<Set<Token>>;
  h: Command[];
  links: Array<Set<number>>;
  slots: Array<Set<number>>;
  allChoices: Set<number>;
  board: string[][];
  done: boolean;
};

function range(start: number, end: number): number[] {
  const res: number[] = [];
  for (let i = start; i < end; ++i) {
    res.push(i);
  }
  return res;
}

const allTokens: Token[] = range(1, 10).map((i) => `${i}`);

function coordToI(r: number, c: number): number {
  return r * 9 + c;
}

function coordFromI(i: number): [number, number] {
  return [Math.floor(i / 9), i % 9];
}

function coordToSq(r: number, c: number): number {
  const rr = Math.floor(r / 3);
  const cc = Math.floor(c / 3);
  return rr * 3 + cc;
}

function initState(board: string[][]): State {
  const state: State = {
    c: range(0, 9 * 9).map((i) => new Set()),
    h: [],
    links: range(0, 9 * 9).map((i) => new Set()),
    board,
    done: false,
    allChoices: new Set(),
    slots: range(0, 9).map((i) => new Set()),
  };
  const freeByRow: number[][] = range(0, 9).map((i) => []);
  const freeByCol: number[][] = range(0, 9).map((i) => []);
  const freeBySq: number[][] = range(0, 9).map((i) => []);
  board.forEach((row, r) => {
    row.forEach((cell, c) => {
      if (cell === ".") {
        const i = coordToI(r, c);
        state.c[i] = new Set(allTokens);
        state.allChoices.add(i);
        freeByRow[r].push(i);
        freeByCol[c].push(i);
        freeBySq[coordToSq(r, c)].push(i);
      }
    });
  });
  board.forEach((row, r) => {
    row.forEach((cell, c) => {
      const i = coordToI(r, c);
      const links = state.links[i];
      freeByRow[r]
        .concat(freeByCol[c])
        .concat(freeBySq[coordToSq(r, c)])
        .forEach((v) => links.add(v));
      if (cell !== ".") {
        links.forEach((l) => {
          state.c[l].delete(cell);
        });
      }
    });
  });
  state.c.forEach((c, i) => {
    state.slots[c.size].add(i);
  });
  return state;
}

function getChoices(state: State): Array<Command> {
  const slot = state.slots.find((s, i) => s.size && i)!;
  const c = slot.values().next().value as number;
  return [...state.c[c]].map((v) => ({ c, v }));
}

type Application = {
  failed: boolean;
  modifs: Set<number>;
};

function setBoard(state: State, i: number, v: Token) {
  const [r, c] = coordFromI(i);
  state.board[r][c] = v;
}

function applyChoice(state: State, c: Command): Application {
  const res: Application = {
    failed: false,
    modifs: new Set(),
  };
  for (let l of state.links[c.c]) {
    const poss = state.c[l];
    if (l === c.c) {
      setBoard(state, l, c.v);
      state.slots[poss.size].delete(l);
      continue;
    }
    if (!poss.has(c.v) || !state.allChoices.has(l)) {
      continue;
    }
    res.modifs.add(l);
    poss.delete(c.v);
    state.slots[poss.size].add(l);
    state.slots[poss.size + 1].delete(l);
    if (state.c[l].size === 0) {
      res.failed = true;
    }
  }
  state.allChoices.delete(c.c);
  return res;
}

function rollback(state: State, c: Command, a: Application) {
  state.slots[state.c[c.c].size].add(c.c);
  setBoard(state, c.c, ".");
  for (let l of a.modifs) {
    const poss = state.c[l];
    poss.add(c.v);
    state.slots[poss.size].add(l);
    state.slots[poss.size - 1].delete(l);
  }
  state.allChoices.add(c.c);
}

function pPrint(state: State, p?: number) {
  state.board.forEach((row, i) => {
    if (i % 3 === 0 && i) {
      console.log("");
    }
    const r = row.map((c, x) =>
      state.allChoices.has(coordToI(i, x))
        ? coordToI(i, x) === p
          ? "*"
          : "."
        : c,
    );
    console.log(
      range(0, 3)
        .map((i) => r.slice(i * 3, (i + 1) * 3).join(""))
        .join(" "),
    );
  });
  console.log("---");
}

function solveRec(state: State): void {
  if (state.allChoices.size === 0) {
    state.done = true;
    return;
  }
  for (let c of getChoices(state)) {
    const a = applyChoice(state, c);
    if (!a.failed) {
      solveRec(state);
    }
    if (state.done) {
      break;
    }
    rollback(state, c, a);
  }
}

/**
 Do not return anything, modify board in-place instead.
 */
function solveSudoku(board: string[][]): void {
  const state = initState(board);
  solveRec(state);
}

const board = [
  [".", ".", ".", ".", ".", "7", ".", ".", "9"],
  [".", "4", ".", ".", "8", "1", "2", ".", "."],
  [".", ".", ".", "9", ".", ".", ".", "1", "."],
  [".", ".", "5", "3", ".", ".", ".", "7", "2"],
  ["2", "9", "3", ".", ".", ".", ".", "5", "."],
  [".", ".", ".", ".", ".", "5", "3", ".", "."],
  ["8", ".", ".", ".", "2", "3", ".", ".", "."],
  ["7", ".", ".", ".", "5", ".", ".", "4", "."],
  ["5", "3", "1", ".", "7", ".", ".", ".", "."],
];
solveSudoku(board);
console.log(board);
