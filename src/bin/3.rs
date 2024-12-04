use aoc_2024::*;

fn main() {
    let input = load_input();

    for part2 in [false, true] {
        let mut sum = 0;

        for section in input.split("do()") {
            for section2 in section.split("don't()") {
                for matched in section2.regex_capture_groups(r#"mul\(([0-9]+),([0-9]+)\)"#) {
                    let [a, b] = matched.ca();
                    sum += a.int() * b.int();
                }
                if part2 {
                    break;
                }
            }
        }

        cp(sum);
    }
}
