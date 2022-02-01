use super::summands::*;

pub fn count(n: i128, k: i128, d: i128) -> i128 {
  return Summands::new(n, k, d, k)
    .map(|p| multinomial(n, &p))
    .sum::<i128>()
    - Summands::new(n - 1, k, d, k - 1)
      .map(|p| multinomial(n - 1, &p))
      .sum::<i128>();
}

fn multinomial(n: i128, p: &Vec<i128>) -> i128 {
  let mut res = 1.0;
  let mut cur_idx = 0;
  let mut cur_p = p[cur_idx];
  let mut cur_n = n;
  while cur_n > 0 {
    if cur_p < 1 {
      cur_idx += 1;
      cur_p = p[cur_idx];
      continue;
    }
    res *= cur_n as f64 / cur_p as f64;
    cur_n -= 1;
    cur_p -= 1;
  }
  return res as i128;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn multinomial_test_1() {
    assert_eq!(multinomial(3, &vec![2, 0, 1]), 3);
  }

  #[test]
  fn multinomial_test_2() {
    assert_eq!(multinomial(3, &vec![1, 1, 1]), 6);
  }

  #[test]
  fn multinomial_test_3() {
    assert_eq!(multinomial(11, &vec![1, 4, 4, 2]), 34650);
  }

  #[test]
  fn count_test_1() {
    assert_eq!(count(3, 2, 10), 891);
  }

  #[test]
  fn count_test_2() {
    assert_eq!(count(2, 2, 10), 90);
  }
}
