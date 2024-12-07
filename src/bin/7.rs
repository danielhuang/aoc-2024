use aoc_2024::*;

fn concat(a: Z, b: Z) -> Z {
    a * (10).pow(b.ilog10() + 1) + b
}

fn is_possible(goal: Z, start: Z, rest: &[i128], part2: bool) -> bool {
    if rest.is_empty() {
        goal == start
    } else {
        if start > goal {
            return false;
        }

        (part2 && is_possible(goal, concat(start, rest[0]), &rest[1..], part2))
            || is_possible(goal, start * rest[0], &rest[1..], part2)
            || is_possible(goal, start + rest[0], &rest[1..], part2)
    }
}

fn main() {
    let input = load_input();

    for part2 in [false, true] {
        let mut count = 0;

        for line in input.lines() {
            let ints = line.ints();
            let goal = ints[0];
            let others = &ints[1..];
            if is_possible(goal, others[0], &others[1..], part2) {
                count += goal;
            }
        }

        cp(count);
    }
}
