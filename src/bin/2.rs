use aoc_2024::*;

fn main() {
    let input = load_input();

    for part2 in [false, true] {
        let mut count = 0;

        for line in input.lines() {
            let list = line.ints();

            for i in 0..(list.len() + 1) {
                let mut list = list.clone();

                if part2 {
                    if i < list.len() {
                        list.remove(i);
                    } else {
                        continue;
                    }
                }

                if list.iter().rev().is_sorted() {
                    list.reverse();
                }

                if list.is_sorted()
                    && list
                        .iter()
                        .tuple_windows()
                        .all(|(&a, &b)| (b - a).abs() >= 1 && (b - a).abs() <= 3)
                {
                    count += 1;
                    break;
                }
            }
        }

        cp(count);
    }
}
