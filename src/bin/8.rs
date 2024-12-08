use aoc_2024::*;

fn main() {
    let input = load_input();

    let grid = parse_grid(&input, |x| x, '.');

    let b = bounds(grid.keys().cloned());

    let frequencies = grid.values().copied().filter(|x| *x != '.').cset();

    for part2 in [false, true] {
        let mut antinodes = vec![];

        for letter in frequencies.cii() {
            let places = grid.findv(letter);
            for x in places.cii() {
                for y in places.cii() {
                    if x != y {
                        if part2 {
                            antinodes.push(y);
                        }
                        for dist in 1.. {
                            let antinode = (y - x) * dist + y;
                            if !b.contains(antinode) {
                                break;
                            }
                            antinodes.push(antinode);
                            if !part2 {
                                break;
                            }
                        }
                    }
                }
            }
        }

        antinodes.retain(|x| b.contains(*x));

        cp(antinodes.cset().len());
    }
}
