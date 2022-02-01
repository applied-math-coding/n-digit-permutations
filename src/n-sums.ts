export function* nSums(n: number, k: number, p: number, k_first: number): Generator<number[]> {
  if (p === 0 || (p - 1) * k + k_first < n || n < 0) {
    return;
  } else if (p === 1) {
    yield [n];
  } else {
    for (let cur_v = 0; cur_v <= k_first; cur_v++) {
      for (let v of nSums(n - cur_v, k, p - 1, k)) {
        v.unshift(cur_v);
        yield v;
      }
    }
  }
}
