use std::{ops::RangeInclusive, str::FromStr};

const INPUT: &str = include_str!("../inputs/day5");
const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[derive(Debug, Clone)]
struct Inventory {
  ranges: Vec<RangeInclusive<usize>>,
  items: Vec<usize>,
}

impl Inventory {
  fn fresh_count(&self) -> usize {
    self
      .items
      .iter()
      .filter(|&&item| self.ranges.iter().any(|range| range.contains(&item)))
      .count()
  }

  fn possible_fresh_count(&mut self) -> usize {
    self.max_compact();
    self.ranges.iter().cloned().map(|r| r.count()).sum()
  }

  fn max_compact(&mut self) {
    println!("Starting compaction with {} ranges", self.ranges.len());
    while self.compact() {}
    println!("Compacted ranges: {:?}", self.ranges.len());
  }

  fn compact(&mut self) -> bool {
    for (i1, r1) in self.ranges.clone().iter().enumerate() {
      for (i2, r2) in self.ranges.clone().iter().enumerate() {
        if i1 == i2 {
          continue;
        }
        // check if ranges overlap at all
        if r1.contains(r2.start()) || r1.contains(r2.end()) {
          let new_start = std::cmp::min(*r1.start(), *r2.start());
          let new_end = std::cmp::max(*r1.end(), *r2.end());
          let new = RangeInclusive::new(new_start, new_end);
          self.ranges.retain(|r| r != r1 && r != r2);
          self.ranges.push(new);
          return true;
        }
      }
    }
    false
  }
}

impl FromStr for Inventory {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut ranges = Vec::new();

    let (first, last) = input
      .split_once("\n\n")
      .ok_or_else(|| "Invalid inventory input".to_string())?;

    for line in first.lines() {
      let (start_str, end_str) = line
        .split_once("-")
        .ok_or_else(|| format!("Invalid range line: {}", line))?;
      let start = start_str
        .trim()
        .parse::<usize>()
        .map_err(|e| format!("Invalid start of range '{}': {}", start_str, e))?;
      let end = end_str
        .trim()
        .parse::<usize>()
        .map_err(|e| format!("Invalid end of range '{}': {}", end_str, e))?;
      ranges.push(RangeInclusive::new(start, end));
    }

    let items = last
      .lines()
      .map(|line| {
        line
          .trim()
          .parse::<usize>()
          .map_err(|e| format!("Invalid item '{}': {}", line, e))
      })
      .collect::<Result<Vec<_>, String>>()?;

    Ok(Self { ranges, items })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn p1_example() {
    let inventory: Inventory = EXAMPLE.parse().unwrap();
    assert_eq!(inventory.fresh_count(), 3);
  }

  #[test]
  fn p1_solution() {
    let inventory: Inventory = INPUT.parse().unwrap();
    assert_eq!(inventory.fresh_count(), 798);
  }

  #[test]
  fn p2_example() {
    let mut inventory: Inventory = EXAMPLE.parse().unwrap();
    assert_eq!(inventory.possible_fresh_count(), 14);
  }

  #[test]
  fn p2_solution() {
    let mut inventory: Inventory = INPUT.parse().unwrap();
    assert_eq!(inventory.possible_fresh_count(), 366181852921027);
  }
}
