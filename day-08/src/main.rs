use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(isize),
    Jump(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let instruction_str = parts[0];
        let val: isize = parts[1].parse().unwrap();
        match instruction_str {
            "acc" => Ok(Self::Acc(val)),
            "jmp" => Ok(Self::Jump(val)),
            "nop" => Ok(Self::Nop(val)),
            _ => Err(()),
        }
    }
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .split('\n')
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn execute_program(instructions: Vec<Instruction>) -> (isize, bool) {
    let mut idx: isize = 0;
    let mut executed_idx: Vec<isize> = vec![];
    let mut acc: isize = 0;

    // Get value either before infinite loop starts or on program completion
    while !executed_idx.contains(&idx) && idx < instructions.len() as isize {
        executed_idx.push(idx);
        match instructions[idx as usize] {
            Instruction::Acc(v) => {
                acc += v;
                idx += 1;
            }
            Instruction::Jump(v) => {
                idx += v;
            }
            Instruction::Nop(_) => {
                idx += 1;
            }
        };
    }

    (acc, !executed_idx.contains(&idx))
}

fn find_changed_instruction_acc(instructions: Vec<Instruction>) -> isize {
    instructions
        .iter()
        .enumerate()
        .filter(
            |(_idx, instruction)| matches!(instruction, Instruction::Jump(_) | Instruction::Nop(_)),
        )
        .map(|(idx, _)| {
            let mut modified_instructions = instructions.clone();
            let instruction_to_change = &modified_instructions[idx];
            modified_instructions[idx] = match instruction_to_change {
                Instruction::Jump(v) => Instruction::Nop(*v),
                Instruction::Nop(v) => Instruction::Jump(*v),
                _ => unreachable!(""),
            };
            execute_program(modified_instructions)
        })
        .filter(|(_acc, completed)| *completed)
        .map(|(acc, _)| acc)
        .take(1)
        .next()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("./day-08/input.txt").unwrap();

    let instructions = parse_instructions(&input);
    let (acc, _) = execute_program(instructions.clone());
    println!("Part 1: {}", acc);

    println!("Part 2: {}", find_changed_instruction_acc(instructions));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let instructions = parse_instructions(sample);
        let (acc, _) = execute_program(instructions);
        assert_eq!(acc, 5);
    }

    #[test]
    fn test_sample_2() {
        let sample = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let instructions = parse_instructions(sample);
        assert_eq!(find_changed_instruction_acc(instructions), 8);
    }
}
