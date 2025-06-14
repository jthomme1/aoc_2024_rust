use std::fs::read_to_string;
use std::str::FromStr;
use std::usize;

#[derive(Debug, PartialEq, Eq, Clone)]
struct MulInstruction {
    multiplicand: usize,
    multiplier: usize,
}

impl MulInstruction {
    fn result(&self) -> usize {
        self.multiplicand * self.multiplier
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InstructionParseErr;

impl FromStr for MulInstruction {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix("mul(")
            .and_then(|s| s.strip_suffix(")"))
            .and_then(|s| s.split_once(","))
            .ok_or(InstructionParseErr)?;
        let multiplicand = x.parse::<usize>().map_err(|_| InstructionParseErr)?;
        let multiplier = y.parse::<usize>().map_err(|_| InstructionParseErr)?;
        Ok(MulInstruction {
            multiplicand,
            multiplier,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DoInstruction;

impl FromStr for DoInstruction {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "do()" {
            Ok(DoInstruction)
        } else {
            Err(InstructionParseErr)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DontInstruction;

impl FromStr for DontInstruction {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "don't()" {
            Ok(DontInstruction)
        } else {
            Err(InstructionParseErr)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    DoInstruction(),
    DontInstruction(),
    MulInstruction(MulInstruction),
}

#[derive(Debug, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq)]
struct ProgramParseErr;

impl FromStr for Program {
    type Err = ProgramParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indices_with_mul_instructions =
            Program::get_indices_with_instructions(s, "mul(", |candidate| {
                candidate
                    .parse::<MulInstruction>()
                    .map(Instruction::MulInstruction)
            });
        let indices_with_do_instructions =
            Program::get_indices_with_instructions(s, "do(", |candidate| {
                candidate
                    .parse::<DoInstruction>()
                    .map(|_instr| Instruction::DoInstruction())
            });
        let indices_with_dont_instructions =
            Program::get_indices_with_instructions(s, "don't(", |candidate| {
                candidate
                    .parse::<DontInstruction>()
                    .map(|_instr| Instruction::DontInstruction())
            });
        let mut indices_with_instructions = vec![
            indices_with_mul_instructions,
            indices_with_do_instructions,
            indices_with_dont_instructions,
        ]
        .concat();

        indices_with_instructions.sort_by_key(|(index, _)| *index);

        let sorted_instructions = indices_with_instructions
            .into_iter()
            .map(|(_, instr)| instr)
            .collect();

        Ok(Program {
            instructions: sorted_instructions,
        })
    }
}

impl Program {
    fn evaluate(&self) -> usize {
        // FIXME: implement
        0
    }

    fn get_indices_with_instructions<'a, T, Err>(
        s: &'a str,
        candidate_start: &str,
        parse: fn(&'a str) -> Result<T, Err>,
    ) -> Vec<(usize, T)> {
        Program::find_candidates_with(s, candidate_start)
            .into_iter()
            .map(|(index, candidate_str)| (index, parse(candidate_str)))
            .filter_map(|(index, maybe_instruction)| {
                maybe_instruction.ok().map(|instr| (index, instr))
            })
            .collect()
    }

    fn find_candidates_with<'a>(s: &'a str, candidate_start: &str) -> Vec<(usize, &'a str)> {
        s.match_indices(candidate_start)
            .map(|(start_index, _)| {
                s[start_index..].find(")").and_then(|end_index| {
                    Some((start_index, &s[start_index..start_index + end_index + 1]))
                })
            })
            .filter_map(|x| x)
            .collect()
    }
}

fn read_input() -> String {
    let input_file_name = "input";
    read_to_string(input_file_name).unwrap()
}

fn main() {
    let input = read_input();
    let program = input.parse::<Program>().unwrap();
    println!("{}", program.evaluate())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_parsing_should_succeed_if_string_is_valid() {
        let valid_do_string = "do()";
        assert_eq!(
            valid_do_string.parse::<DoInstruction>(),
            Ok(DoInstruction {})
        );
    }

    #[test]
    fn test_dont_parsing_should_succeed_if_string_is_valid() {
        let valid_dont_string = "don't()";
        assert_eq!(
            valid_dont_string.parse::<DontInstruction>(),
            Ok(DontInstruction {})
        );
    }

    #[test]
    fn test_mul_parsing_should_succeed_if_string_is_valid() {
        let valid_mul_string = "mul(123,45)";
        assert_eq!(
            valid_mul_string.parse::<MulInstruction>(),
            Ok(MulInstruction {
                multiplicand: 123,
                multiplier: 45
            })
        );
    }

    #[test]
    fn test_mul_parsing_should_fail_for_invalid_strings() {
        let invalid_mul_strings = vec!["mul(123,45", "", "ul(1,1)", "mul (1, 2)"];
        for invalid_mul_string in invalid_mul_strings {
            assert_eq!(
                invalid_mul_string.parse::<MulInstruction>(),
                Err(InstructionParseErr)
            );
        }
    }

    #[test]
    fn test_program_parsing_works_as_expected() {
        let program_string = "don't() mul(1,1) do() mul(2,2)";
        assert_eq!(
            program_string.parse::<Program>(),
            Ok(Program {
                instructions: vec![
                    Instruction::DontInstruction(),
                    Instruction::MulInstruction(MulInstruction {
                        multiplicand: 1,
                        multiplier: 1
                    }),
                    Instruction::DoInstruction(),
                    Instruction::MulInstruction(MulInstruction {
                        multiplicand: 2,
                        multiplier: 2
                    })
                ]
            })
        );
    }
}
