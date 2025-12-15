#[derive(Debug, Clone)]
struct Powerbank(Vec<u32>);

const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

const INPUT: &str = include_str!("../inputs/day3");

impl TryFrom<&str> for Powerbank {
  type Error = String;

  fn try_from(input: &str) -> Result<Self, Self::Error> {
    let items = input
      .trim()
      .chars()
      .map(|c| {
        c.to_digit(10)
          .ok_or_else(|| format!("Invalid character in powerbank input: {}", c))
      })
      .collect::<Result<Vec<_>, String>>()?;
    Ok(Self(items))
  }
}

impl Powerbank {
  fn iter(&self) -> impl Iterator<Item = &u32> {
    self.0.iter()
  }

  fn max_power(&self) -> usize {
    if self.0.is_empty() || self.0.len() == 1 {
      panic!("Power bank is not big enough");
    }

    let last = (self.0.len() - 1) as usize;

    // find the largest digit, preferring earlier if there are multiple of the same
    let (left_index, left_digit) = self.0[0..last]
      .iter()
      .enumerate()
      .max_by_key(|&(index, &digit)| (digit, -(index as isize)))
      .unwrap();

    let (_, right_digit) = self
      .0
      .iter()
      .enumerate()
      .skip(left_index + 1)
      .max_by_key(|&(_, &digit)| digit)
      .unwrap();

    (left_digit * 10 + right_digit) as usize
  }

  fn max_power_v2(&self, digits: usize) -> usize {
    let mut to_skip = 0;

    (0..digits).fold(0, |accum, elem| {
      let reserve = digits - 1 - elem;
      let end = self.0.len() - reserve;
      let (index, &max) = self.0[0..end]
        .iter()
        .enumerate()
        .skip(to_skip)
        .max_by_key(|&(index, &digit)| (digit, -(index as isize)))
        .unwrap();

      to_skip = index + 1;

      (accum * 10) + max as usize
    })
  }

  fn read_list(input: &str) -> Result<Vec<Powerbank>, String> {
    input.lines().map(|line| line.trim().try_into()).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn p1_example() {
    let banks = Powerbank::read_list(EXAMPLE).unwrap();
    let value = banks.iter().map(|b| b.max_power()).sum::<usize>();
    assert_eq!(value, 357);
  }

  #[test]
  fn p1_solution() {
    let banks = Powerbank::read_list(INPUT).unwrap();
    let value = banks.iter().map(|b| b.max_power()).sum::<usize>();
    assert_eq!(value, 17694);
  }

  #[test]
  fn p2_example() {
    let banks = Powerbank::read_list(EXAMPLE).unwrap();
    let value = banks.iter().map(|b| b.max_power_v2(12)).sum::<usize>();
    assert_eq!(value, 3121910778619);
  }

  #[test]
  fn p2_solution() {
    let banks = Powerbank::read_list(INPUT).unwrap();
    let value = banks.iter().map(|b| b.max_power_v2(12)).sum::<usize>();
    assert_eq!(value, 175659236361660);
  }
}
