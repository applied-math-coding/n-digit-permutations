use super::utils::Statistic;

pub struct DigitPermutations {
  n: i32,
  k: i32,
  stat: Statistic,
  next_iter: Option<Box<DigitPermutations>>,
  cur_digit: i32,
}

impl DigitPermutations {
  pub fn new(n: i32, k: i32) -> DigitPermutations {
    return DigitPermutations {
      n,
      k,
      stat: [0; 10],
      next_iter: None,
      cur_digit: 1,
    };
  }

  fn ensure_next_iter(&mut self) {
    match self.next_iter.as_mut() {
      None => self.init_next_iter(),
      _ => (),
    };
  }

  fn init_next_iter(&mut self) {
    let mut next_stat = self.stat;
    next_stat[self.cur_digit as usize] += 1;
    self.next_iter = Some(Box::new(DigitPermutations {
      n: self.n - 1,
      k: self.k,
      stat: next_stat,
      next_iter: None,
      cur_digit: 0,
    }));
  }

  fn get_next_digit(&self) -> i32 {
    for d in (self.cur_digit + 1)..10 {
      if self.stat[d as usize] < self.k {
        return d;
      }
    }
    return 10;
  }
}

impl Iterator for DigitPermutations {
  type Item = Vec<i32>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.n == 0 {
      return None;
    } else if self.n == 1 {
      let res = if self.cur_digit > 9 {
        None
      } else {
        Some(vec![self.cur_digit])
      };
      self.cur_digit = self.get_next_digit();
      return res;
    } else {
      self.ensure_next_iter();
      loop {
        if let Some(v) = self.next_iter.as_mut()?.next() {
          let mut v_new = v.clone();
          v_new.insert(0, self.cur_digit);
          return Some(v_new);
        } else {
          self.cur_digit = self.get_next_digit();
          if self.cur_digit > 9 {
            return None;
          }
          self.init_next_iter();
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn iterator_test_1() {
    assert_eq!(DigitPermutations::new(3, 2).count(), 891);
  }

  #[test]
  fn enumerate_test_2() {
    assert_eq!(DigitPermutations::new(2, 2).count(), 90);
  }
}
