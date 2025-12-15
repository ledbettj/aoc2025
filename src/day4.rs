use std::collections::HashSet;

const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
const INPUT: &str = include_str!("../inputs/day4");

type Position = (isize, isize);

#[derive(Debug, Clone)]
struct Grid {
  cells: HashSet<Position>,
  width: usize,
  height: usize,
}

impl TryFrom<&str> for Grid {
  type Error = String;

  fn try_from(input: &str) -> Result<Self, Self::Error> {
    let mut cells = HashSet::new();
    let lines: Vec<&str> = input.trim().lines().collect();
    let height = lines.len();
    let width = lines
      .get(0)
      .ok_or_else(|| "Input grid is empty".to_string())?
      .len();

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        match c {
          '@' => {
            cells.insert((x as isize, y as isize));
          }
          '.' => {}
          _ => {
            return Err(format!(
              "Invalid character in grid input: {} at {},{}",
              c, x, y
            ));
          }
        };
      }
    }

    Ok(Self {
      cells,
      width,
      height,
    })
  }
}

impl Grid {
  fn neighbors(&self, &(x, y): &Position) -> usize {
    let mut count = 0;
    for xoff in -1..=1 {
      for yoff in -1..=1 {
        if xoff == 0 && yoff == 0 {
          continue;
        }

        let pos = (x + xoff, y + yoff);
        if self.cells.contains(&pos) {
          count += 1;
        }
      }
    }

    count
  }

  fn count_moveable(&self) -> usize {
    self
      .cells
      .iter()
      .filter(|&pos| self.neighbors(&pos) < 4)
      .count()
  }

  fn remove_moveable(&mut self) -> usize {
    let to_remove: Vec<Position> = self
      .cells
      .iter()
      .filter(|&pos| self.neighbors(&pos) < 4)
      .cloned()
      .collect();

    let total = to_remove.len();

    for pos in to_remove {
      self.cells.remove(&pos);
    }

    total
  }

  fn cleanup(&mut self) -> usize {
    let mut total_removed = 0;
    loop {
      let removed = self.remove_moveable();
      if removed == 0 {
        break;
      }
      total_removed += removed;
    }
    total_removed
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn p1_example() {
    let grid: Grid = EXAMPLE.try_into().unwrap();
    assert_eq!(grid.count_moveable(), 13);
  }

  #[test]
  pub fn p1_solution() {
    let grid: Grid = INPUT.try_into().unwrap();
    assert_eq!(grid.count_moveable(), 1433);
  }

  #[test]
  fn p2_example() {
    let mut grid: Grid = EXAMPLE.try_into().unwrap();
    assert_eq!(grid.cleanup(), 43);
  }

  #[test]
  fn p2_solution() {
    let mut grid: Grid = INPUT.try_into().unwrap();
    assert_eq!(grid.cleanup(), 8616);
  }
}
