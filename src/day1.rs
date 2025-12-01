const INPUT: &'static str = include_str!("../inputs/day1");
const EXAMPLE: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[derive(Debug, Clone, Copy)]
enum Direction {
  Left,
  Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
  dir: Direction,
  count: usize,
}

impl TryFrom<&str> for Instruction {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let (dir_char, count_str) = value.split_at(1);

    let dir = match dir_char {
      "L" => Direction::Left,
      "R" => Direction::Right,
      ch => return Err(format!("Invalid direction: {}", ch)),
    };

    let count = count_str
      .parse::<usize>()
      .map_err(|e| format!("Invalid count: {}", e))?;

    Ok(Instruction { dir, count })
  }
}

impl Instruction {
  fn parse_list(input: &str) -> Result<Vec<Instruction>, String> {
    input
      .lines()
      .map(|line| Instruction::try_from(line.trim()))
      .collect()
  }

  fn execute(list: &[Instruction]) -> usize {
    let mut zeroes = 0;
    let mut pos = 50;

    for instr in list {
      let c = instr.count % 100;
      pos = match instr.dir {
        Direction::Left => (pos + 100 - c) % 100,
        Direction::Right => (pos + c) % 100,
      };

      if pos == 0 {
        zeroes += 1;
      }
    }
    return zeroes;
  }

  fn execute_p2(list: &[Instruction]) -> usize {
    let mut zeroes = 0;
    let mut pos = 50;

    for instr in list {
      let c = instr.count % 100;
      zeroes += instr.count / 100;

      let new_pos = match instr.dir {
        Direction::Left => (pos + 100 - c) % 100,
        Direction::Right => (pos + c) % 100,
      };

      zeroes += match instr.dir {
        Direction::Left if new_pos == 0 || (new_pos > pos && pos > 0) => 1,
        Direction::Right if new_pos == 0 || (new_pos < pos && pos > 0) => 1,
        _ => 0,
      };

      pos = new_pos;
    }

    return zeroes;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn p1_example() {
    let instructions = Instruction::parse_list(EXAMPLE).unwrap();
    let result = Instruction::execute(&instructions);
    assert_eq!(result, 3);
  }

  #[test]
  fn p1_solution() {
    let instructions = Instruction::parse_list(INPUT).unwrap();
    let result = Instruction::execute(&instructions);
    assert_eq!(result, 969);
  }

  #[test]
  fn p2_example() {
    let instructions = Instruction::parse_list(EXAMPLE).unwrap();
    let result = Instruction::execute_p2(&instructions);
    assert_eq!(result, 6);
  }

  #[test]
  fn p2_solution() {
    let instructions = Instruction::parse_list(INPUT).unwrap();
    let result = Instruction::execute_p2(&instructions);
    assert_eq!(result, 5887);
  }
}
