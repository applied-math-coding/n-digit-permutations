pub struct Summands {
  n: i128,
  k: i128,
  p: i128,
  cur_v: i128,
  next_iter: Option<Box<Summands>>,
  k_first: i128,
}

impl Summands {
  pub fn new(n: i128, k: i128, p: i128, k_first: i128) -> Summands {
    return Summands {
      n,
      k,
      p,
      cur_v: 0,
      next_iter: None,
      k_first,
    };
  }

  fn init_next_iter(&mut self) {
    self.next_iter = Some(Box::new(Summands::new(
      self.n - self.cur_v,
      self.k,
      self.p - 1,
      self.k,
    )));
  }
}

impl Iterator for Summands {
  type Item = Vec<i128>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.p == 0 || (self.p - 1) * self.k + self.k_first < self.n || self.n < 0 {
      return None;
    } else if self.p == 1 {
      self.p = 0;
      return Some(vec![self.n]);
    } else {
      loop {
        match self.next_iter {
          None => {
            self.init_next_iter();
          }
          _ => (),
        }
        if let Some(mut v) = self.next_iter.as_mut()?.next() {
          v.insert(0, self.cur_v);
          return Some(v);
        } else if self.cur_v + 1 <= self.k_first {
          self.cur_v += 1;
          self.init_next_iter();
        } else {
          return None;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn summands_test_1() {
    assert_eq!(Summands::new(3, 2, 2, 2).count(), 2);
  }

  #[test]
  fn summands_test_2() {
    assert_eq!(Summands::new(5, 3, 3, 3).count(), 12);
  }

  #[test]
  fn summands_test_3() {
    assert_eq!(Summands::new(5, 3, 3, 2).count(), 9);
  }
}
