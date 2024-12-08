use aoc_2024::*;

fn unconcat_prefix(total: Z, suffix: Z) -> Option<Z> {
    let suffix_digits = suffix.ilog10() + 1;
    let remaining_digits = total / (10).pow(suffix_digits);
    let expected_suffix = total % (10).pow(suffix_digits);
    if suffix == expected_suffix {
        Some(remaining_digits)
    } else {
        None
    }
}

fn is_possible(goal: Z, base: Z, rest: &[Z], part2: bool) -> bool {
    if rest.is_empty() {
        goal == base
    } else {
        (part2
            && unconcat_prefix(goal, rest[rest.len() - 1])
                .is_some_and(|x| is_possible(x, base, &rest[..rest.len() - 1], part2)))
            || (goal % rest[rest.len() - 1] == 0
                && is_possible(
                    goal / rest[rest.len() - 1],
                    base,
                    &rest[..rest.len() - 1],
                    part2,
                ))
            || is_possible(
                goal - rest[rest.len() - 1],
                base,
                &rest[..rest.len() - 1],
                part2,
            )
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
