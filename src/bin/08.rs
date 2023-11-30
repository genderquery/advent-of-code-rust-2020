use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

struct Cpu<'a> {
    instructions: &'a [Instruction],
    pc: i16,
    acc: i16,
}

impl Cpu<'_> {
    pub fn new(instructions: &[Instruction]) -> Cpu {
        Cpu {
            instructions,
            pc: 0,
            acc: 0,
        }
    }

    pub fn tick(&mut self) {
        match self.instructions[self.pc as usize] {
            Instruction::Acc(operand) => {
                self.acc += operand;
                self.pc += 1;
            }
            Instruction::Jmp(dest) => {
                self.pc += dest;
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (instr, operand) = line.split_once(' ').unwrap();
            let operand = operand.parse().unwrap();
            match instr {
                "nop" => Instruction::Nop(operand),
                "acc" => Instruction::Acc(operand),
                "jmp" => Instruction::Jmp(operand),
                _ => panic!("invalid instruction"),
            }
        })
        .collect()
}

/// Determines if a given set of `instructions` will terminate and if so,
/// returns the value of the accumulator when it does.
fn does_terminate(instructions: &[Instruction]) -> Option<i16> {
    let mut cpu = Cpu::new(instructions);
    let mut visited = HashSet::new();
    while (cpu.pc as usize) < instructions.len() && !visited.contains(&cpu.pc) {
        visited.insert(cpu.pc);
        cpu.tick();
    }
    if (cpu.pc as usize) >= instructions.len() {
        Some(cpu.acc)
    } else {
        None
    }
}

/// Flip a JMP to a NOP and vice versa.
fn flip_instruction(instruction: Instruction) -> Instruction {
    match instruction {
        Instruction::Jmp(op) => Instruction::Nop(op),
        Instruction::Nop(op) => Instruction::Jmp(op),
        _ => unreachable!(),
    }
}

/// Find a corrupted instruction and return the value of the accumulator after
/// successfully terminating.
fn find_corrupted_instruction(instructions: &[Instruction]) -> Option<i16> {
    for (pc, _) in instructions
        .iter()
        .enumerate()
        .filter(|(_, instr)| !matches!(instr, Instruction::Acc(_)))
    {
        let mut instructions = instructions.to_owned();
        instructions[pc] = flip_instruction(instructions[pc]);
        if let Some(acc) = does_terminate(&instructions) {
            return Some(acc);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let instructions = parse(input);
    let mut cpu = Cpu::new(&instructions);
    let mut visited = HashSet::new();
    while !visited.contains(&cpu.pc) {
        visited.insert(cpu.pc);
        cpu.tick();
    }
    cpu.acc.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = parse(input);
    find_corrupted_instruction(&instructions)
        .unwrap()
        .to_string()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "8");
    }
}
