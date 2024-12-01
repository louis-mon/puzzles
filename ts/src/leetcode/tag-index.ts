const stream = ["a,b,c", "b,c,d", "c,d,e"];

function findInTags(input: string[], tags: string[]): string[] {
  const inputList = input.map((t) => t.split(","));
  const indexes = new Map<string, Set<number>>();
  inputList.forEach((list, index) => {
    list.forEach((tag) => {
      if (!indexes.has(tag)) {
        indexes.set(tag, new Set<number>());
      }
      indexes.get(tag)!.add(index);
    });
  });
  const initIndexes = new Set(inputList.map((l, i) => i));
  const getIndexSize = (tag: string) => indexes.get(tag)?.size || 0;
  // this allows for reduce quickly the size of indexes to recopy after
  const mostSelective = tags
    .slice()
    .sort((a, b) => getIndexSize(a) - getIndexSize(b));
  const intersection = mostSelective.reduce((acc, tag) => {
    const tagIndexes = indexes.get(tag);
    if (!tagIndexes) {
      return acc;
    }
    const newSet = new Set<number>();
    acc.forEach((t) => {
      if (tagIndexes.has(t)) {
        newSet.add(t);
      }
    });
    return newSet;
  }, initIndexes);
  const resultSet = new Set<string>();
  const inputSet = new Set(tags);
  intersection.forEach((i) => {
    inputList[i].forEach((t) => {
      if (inputSet.has(t)) {
        return;
      }
      resultSet.add(t);
    });
  });
  return [...resultSet];
}

console.log(findInTags(stream, ["a", "b"]));
