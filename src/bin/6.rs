use aoc_2024::*;

fn main() {
    let input = load_input();

    let grid = parse_grid(&input, |x| x, '.');

    let b = bounds(grid.keys().cloned());

    let start = grid.findv('^').one();

    let mut positions = set([start]);

    for part2 in [false, true] {
        let mut count = 0;

        for blocker in positions.clone() {
            let mut pos = start;
            let mut vel = v2(0, 1);

            let mut states = set([]);

            positions.insert(pos);

            while b.contains(pos) {
                pos += vel;
                while (part2 && pos + vel == blocker) || grid[pos + vel] == '#' {
                    vel = vel.turn_right();
                }
                if !part2 {
                    positions.insert(pos);
                }
                if !states.insert((pos, vel)) {
                    count += 1;
                    break;
                }
            }

            if !part2 {
                cp(positions.len() - 1);
                break;
            }
        }

        if part2 {
            cp(count - 1);
        }
    }
}
