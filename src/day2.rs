use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../inputs/day2");
const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

fn id_pair_to_range(input: &str) -> Result<RangeInclusive<usize>, String> {
  let (left, right) = input
    .split_once("-")
    .ok_or_else(|| format!("Invalid ID pair format: {}", input))?;

  let start = left
    .parse::<usize>()
    .map_err(|e| format!("Failed to parse start ID: {}", e))?;

  let end = right
    .parse::<usize>()
    .map_err(|e| format!("Failed to parse end ID: {}", e))?;

  Ok(RangeInclusive::new(start, end))
}

fn is_invalid(id: usize) -> bool {
  let digits = (id as f64).log10().floor() as usize + 1;
  let divisor = 10_usize.pow((digits / 2) as u32);
  let top = id / divisor;
  let bottom = id % divisor;

  top == bottom
}

fn is_invalid_p2(id: usize) -> bool {
  // check for any number of sets of repeated digits, not just two.
  // e.g. 123123, 454545, 111111, 22, 2424
  let digits = (id as f64).log10().floor() as usize + 1;
  let half_divisor = 10_usize.pow((digits / 2) as u32);
  let mut divisor = 10;

  while divisor <= half_divisor {
    // make sure it splits evenly into chunks
    let divisor_digits = (divisor as f64).log10().floor() as usize;
    if digits % divisor_digits != 0 {
      divisor *= 10;
      continue;
    }

    let mut value = id;
    let mut success = true;
    let expected = value % divisor;

    while value != 0 {
      let chunk = value % divisor;
      if chunk != expected {
        success = false;
        break;
      }

      value /= divisor;
    }

    if success {
      return true;
    }

    divisor *= 10;
  }

  return false;
}

fn select_invalid(range: &RangeInclusive<usize>) -> Vec<usize> {
  range.clone().filter(|id| is_invalid(*id)).collect()
}

fn select_invalid_p2(range: &RangeInclusive<usize>) -> Vec<usize> {
  range.clone().filter(|id| is_invalid_p2(*id)).collect()
}

#[derive(Debug, Clone)]
struct RangeSet {
  ranges: Vec<RangeInclusive<usize>>,
}

impl RangeSet {
  fn from_list(input: &str) -> Result<Self, String> {
    input
      .split(",")
      .map(str::trim)
      .map(id_pair_to_range)
      .collect::<Result<Vec<_>, String>>()
      .map(|ranges| Self { ranges })
  }

  fn iter(&self) -> impl Iterator<Item = &RangeInclusive<usize>> {
    self.ranges.iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn p2_detection() {
    //assert!(!is_invalid_p2(2121212124));
    assert!(!is_invalid_p2(10101));
  }

  #[test]
  fn p1_example() {
    let ranges = RangeSet::from_list(EXAMPLE).expect("Failed to parse ranges");
    let ans = ranges
      .iter()
      .flat_map(|range| select_invalid(range))
      .sum::<usize>();

    assert_eq!(ans, 1227775554);
  }

  #[test]
  fn p1_solution() {
    let ranges = RangeSet::from_list(INPUT).expect("Failed to parse ranges");
    let ans = ranges
      .iter()
      .flat_map(|range| select_invalid(range))
      .sum::<usize>();

    assert_eq!(ans, 17077011375);
  }

  #[test]
  fn p2_example() {
    let ranges = RangeSet::from_list(EXAMPLE).expect("Failed to parse ranges");
    let ans = ranges
      .iter()
      .flat_map(|range| select_invalid_p2(range))
      .sum::<usize>();

    assert_eq!(ans, 4174379265);
  }

  #[test]
  fn p2_solution() {
    let ranges = RangeSet::from_list(INPUT).expect("Failed to parse ranges");
    let ans = ranges
      .iter()
      .flat_map(|range| select_invalid_p2(range))
      .sum::<usize>();

    assert_eq!(ans, 36037497037);
  }
}
