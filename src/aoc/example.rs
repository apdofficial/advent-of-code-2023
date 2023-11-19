use itertools::Itertools;

pub fn top_1_elf_calories(input: &str) -> usize {
    calculate_calories_sorted(input)[0]
}

pub fn top_3_elf_calories(input: &str) -> usize {
    calculate_calories_sorted(input)[0..3].iter().sum()
}

fn calculate_calories_sorted(input: &str) -> Vec<usize> {
    input.split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<usize>().unwrap()).sum::<usize>())
        .sorted()
        .rev()
        .collect::<Vec<_>>()
}