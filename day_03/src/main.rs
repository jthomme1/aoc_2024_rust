use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
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
struct MulInstructionParseErr;

impl FromStr for MulInstruction {
    type Err = MulInstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix("mul(")
            .and_then(|s| s.strip_suffix(")"))
            .and_then(|s| s.split_once(","))
            .ok_or(MulInstructionParseErr)?;
        let multiplicand = x.parse::<usize>().map_err(|_| MulInstructionParseErr)?;
        let multiplier = y.parse::<usize>().map_err(|_| MulInstructionParseErr)?;
        Ok(MulInstruction {
            multiplicand,
            multiplier,
        })
    }
}

fn extract_instructions(str: &String) -> Vec<MulInstruction> {
    str.match_indices("mul")
        .map(|(start_index, _)| {
            str[start_index..]
                .find(")")
                .and_then(|end_index| Some(&str[start_index..start_index + end_index + 1]))
        })
        .filter_map(|x| x)
        .filter_map(|str| str.parse::<MulInstruction>().ok())
        .collect()
}

fn read_input() -> String {
    let input_file_name = "input";
    read_to_string(input_file_name).unwrap()
}

fn main() {
    let input = read_input();
    let sum = extract_instructions(&input)
        .into_iter()
        .map(|c| c.result())
        .sum::<usize>();
    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_should_succeed_if_string_is_valid() {
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
    fn test_parsing_should_fail_for_invalid_strings() {
        let invalid_mul_strings = vec!["mul(123,45", "", "ul(1,1)", "mul (1, 2)"];
        for invalid_mul_string in invalid_mul_strings {
            assert_eq!(
                invalid_mul_string.parse::<MulInstruction>(),
                Err(MulInstructionParseErr)
            );
        }
    }
}
