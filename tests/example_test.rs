#[cfg(test)]
mod example_test {
    use std::fs;
    use advent_of_code_2023::aoc::example;

    fn get_test_data() -> &'static str {
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n"
    }

    #[test]
    fn day_01_part1() {
        let data = get_test_data();
        assert_eq!(24000, example::top_1_elf_calories(&data));
    }

    #[test]
    fn day_01_part2() {
        let data = get_test_data();
        assert_eq!(45000, example::top_3_elf_calories(&data));
    }

    #[test]
    fn day_01_file() {
        let data = fs::read_to_string("data/example.txt")
            .expect("Could not to open data/example.txt");
        assert_eq!(69693, example::top_1_elf_calories(&data));
        assert_eq!(200945, example::top_3_elf_calories(&data));
    }
}