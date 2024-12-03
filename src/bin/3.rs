use aoc_2024::*;

fn main() {
    let input = load_input();

    let input = format!("do(){input}");

    for skip_count in [0, 1] {
        let mut sum = 0;

        for section in input.split("don't()") {
            for section2 in section.split("do()").skip(skip_count) {
                for matched in section2.regex_multi("mul\\(([0-9]+),([0-9]+)\\)") {
                    let [a, b] = matched.ca();
                    sum += a.int() * b.int();
                }
            }
        }

        cp(sum);
    }
}
