use aoc_2024::*;

fn main() {
    let input = load_input();

    let mut lefts = vec![];
    let mut rights = vec![];

    for line in input.lines() {
        let [a, b] = line.ints().ca();
        lefts.push(a);
        rights.push(b);
    }

    lefts.sort();
    rights.sort();

    let mut distance = 0;

    for (l, r) in lefts.cii().zip(rights.cii()) {
        distance += (l - r).abs();
    }

    cp(distance);

    let mut similarity = 0;

    let right_freqs = freqs(rights);

    for l in lefts.cii() {
        similarity += l * right_freqs[l].int();
    }

    cp(similarity);
}
