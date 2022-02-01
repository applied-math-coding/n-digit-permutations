export function* digitPermutations(n: number, k: number, stat?: number[]): Generator<number[]> {
  let isRoot = false;
  if (!stat) {
    stat = Array(10).fill(0);
    isRoot = true;
  }
  if (n === 0) {
    return;
  } else if (n === 1) {
    for (const digit of getNextDigit(stat, isRoot, k)) {
      yield [digit];
    }
  } else {
    for (const digit of getNextDigit(stat, isRoot, k)) {
      const nextStat = [...stat];
      nextStat[digit] += 1;
      for (const v of digitPermutations(n - 1, k, nextStat)) {
        v.unshift(digit);
        yield v;
      }
    }
  }
}

function getNextDigit(stat: number[], isRoot: boolean, k: number): number[] {
  return Object
    .entries(stat)
    .filter(([d, v]) => v < k && (!isRoot || +d > 0))
    .map(([d,]) => +d);
}
