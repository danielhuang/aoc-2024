use aoc_2024::*;

fn main() {
    let input = load_input();

    let (head, body) = input.split_once("\n\n").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut before_after = set([]);

    for line in head.lines() {
        let [before, after] = line.ints().ca();
        before_after.insert((before, after));
    }

    for row in body.lines() {
        let row = row.ints();
        let mut sorted_row = row.clone();

        sorted_row.sort_by(|&a, &b| {
            if a == b {
                Ordering::Equal
            } else if before_after.contains(&(a, b)) {
                Ordering::Less
            } else if before_after.contains(&(b, a)) {
                Ordering::Greater
            } else {
                unreachable!()
            }
        });

        if row == sorted_row {
            part1 += row[row.len() / 2];
        } else {
            part2 += sorted_row[sorted_row.len() / 2];
        }
    }

    cp(part1);
    cp(part2);
}
