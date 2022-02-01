import { digitPermutations } from './digit-permutations';

test('digitPermutations (1,2)', () => {
  expect([...digitPermutations(1, 2)].length).toBe(9);
});

test('digitPermutations (2,2)', () => {
  expect([...digitPermutations(2, 2)].length).toBe(90);
});

test('digitPermutations (3,2)', () => {
  expect([...digitPermutations(3, 2)].length).toBe(891);
});