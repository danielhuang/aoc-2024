use aoc_2024::*;

fn main() {
    let input = load_input();

    let grid = parse_grid(&input, |x| x, '.');

    let b = bounds(grid.keys().cloned());

    for part2 in [false, true] {
        let mut count = 0;

        for blocker in b.cells() {
            let mut pos = grid.findv('^').one();
            let mut vel = v2(0, 1);

            let mut positions = set([]);
            let mut states = set([]);

            positions.insert(pos);

            let mut blocked = false;

            while b.contains(pos) {
                pos += vel;
                while (part2 && pos + vel == blocker) || grid[pos + vel] == '#' {
                    vel = vel.turn_right();
                }
                positions.insert(pos);
                if !states.insert((pos, vel)) {
                    blocked = true;
                    break;
                }
            }

            if !part2 {
                cp(positions.len() - 1);
                break;
            }

            if blocked {
                count += 1;
            }
        }

        if part2 {
            cp(count);
        }
    }
}
