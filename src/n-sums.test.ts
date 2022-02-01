import { nSums } from './n-sums';

test('nSums (3,2,2,2)', () => {
  expect([...nSums(3, 2, 2, 2)].length).toBe(2);
});

test('nSums (5, 3, 3, 3)', () => {
  expect([...nSums(5, 3, 3, 3)].length).toBe(12);
});

test('nSums (5, 3, 3, 2)', () => {
  expect([...nSums(5, 3, 3, 2)].length).toBe(9);
});