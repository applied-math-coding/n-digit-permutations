use super::utils::Statistic;

pub fn enumerate(n: i32, k: i32) -> Vec<Vec<i32>> {
  return rec_enumerate(n, k, &[0; 10], true);
}

fn rec_enumerate(n: i32, k: i32, stat: &Statistic, is_root: bool) -> Vec<Vec<i32>> {
  let min_digit: i32 = if is_root { 1 } else { 0 };
  let mut result = vec![];
  if n == 0 {
    return result;
  }
  if n == 1 {
    for (idx, v) in stat.iter().enumerate() {
      if *v < k && idx >= min_digit as usize {
        result.push(vec![idx as i32]);
      }
    }
    return result;
  }
  for d in min_digit..10 {
    if stat[d as usize] >= k {
      continue;
    }
    let mut next_stat = *stat;
    next_stat[d as usize] += 1;
    for p in rec_enumerate(n - 1, k, &next_stat, false).iter() {
      let mut p_c = p.clone();
      p_c.insert(0, d);
      result.push(p_c);
    }
  }
  return result;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn enumerate_test1() {
    assert_eq!(enumerate(3, 2).len(), 891);
  }

  #[test]
  fn enumerate_test2() {
    assert_eq!(enumerate(2, 2).len(), 90);
  }
}
