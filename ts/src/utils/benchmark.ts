export function benchmark<T>(code: () => T) {
  const before = Date.now();
  const res = code();
  console.log(`Elapsed time: ${Date.now() - before} ms`);
  return res;
}
