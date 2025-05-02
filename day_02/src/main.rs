use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Level {
    value: i32,
}

impl Level {
    fn is_close_enough_to_but_not_equal(&self, other: &Level) -> bool {
        return (self.value - other.value).abs() <= 3 && self != other;
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Level) -> bool {
        self.value == other.value
    }
}

impl Eq for Level {}
impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Level) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Level) -> Ordering {
        return self.value.cmp(&other.value);
    }
}

struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn has_sorted_levels(&self) -> bool {
        self.levels.is_sorted() || self.levels.is_sorted_by(|a, b| a > b)
    }

    fn all_levels_are_close_enough_but_not_equal(&self) -> bool {
        self.levels
            .windows(2)
            .all(|w| w[0].is_close_enough_to_but_not_equal(&w[1]))
    }

    fn is_safe(&self) -> bool {
        self.has_sorted_levels() && self.all_levels_are_close_enough_but_not_equal()
    }
}

struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    fn amount_safe(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }
}

fn main() {
    let reports = read_input();
    println!("{}", reports.amount_safe())
}

fn parse_into_report(line: String) -> Report {
    Report {
        levels: line
            .split_whitespace()
            .map(|level| Level {
                value: level.parse::<i32>().unwrap(),
            })
            .collect(),
    }
}

fn read_input() -> Reports {
    let input_file_name = "input";
    let input_file = File::open(input_file_name).unwrap();

    let reader = BufReader::new(input_file);

    Reports {
        reports: reader
            .lines()
            .map(|line| parse_into_report(line.unwrap()))
            .collect(),
    }
}
