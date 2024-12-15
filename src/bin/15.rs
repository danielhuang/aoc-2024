use aoc_2024::*;

fn main() {
    let input = load_input();

    for part2 in [false, true] {
        let [mut head, body] = input.paras().ca();

        if part2 {
            head = head
                .trim()
                .chars()
                .flat_map(|x| {
                    match x {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        '\n' => "\n",
                        _ => unreachable!(),
                    }
                    .chars()
                })
                .cstr();
        }

        let mut grid = parse_grid(&head, |x| x, '#');
        let b = bounds(grid.keys().cloned());

        let mut pos = grid.findv('@').one();
        grid[pos] = '.';

        'outer: for m in body.chars() {
            if m.is_whitespace() {
                continue;
            }

            let vel = charvel(m);
            let new_pos = pos + vel;
            if grid[new_pos] == '#' {
                continue;
            }
            if grid[new_pos] == '[' || grid[new_pos] == ']' || grid[new_pos] == 'O' {
                let to_move = bfs_reach([new_pos], |prev| {
                    let mut results = vec![];
                    if grid[prev] == '[' {
                        results.push(prev.right(1));
                    }
                    if grid[prev] == ']' {
                        results.push(prev.left(1));
                    }
                    if m == 'v' && grid[prev.down(1)].is_in("[]O".chars()) {
                        results.push(prev.down(1));
                    }
                    if m == '^' && grid[prev.up(1)].is_in("[]O".chars()) {
                        results.push(prev.up(1));
                    }
                    if m == '>' && grid[prev.right(1)].is_in("[]O".chars()) {
                        results.push(prev.right(1));
                    }
                    if m == '<' && grid[prev.left(1)].is_in("[]O".chars()) {
                        results.push(prev.left(1));
                    }
                    results
                })
                .map(|x| x.0)
                .cv();
                let mut grid2 = grid.clone();
                for to_remove in to_move.cii() {
                    grid2[to_remove] = '.';
                    if grid2[to_remove + vel] == '#' {
                        continue 'outer;
                    }
                }
                for to_remove in to_move.cii() {
                    grid2[to_remove + vel] = grid[to_remove];
                }
                grid = grid2;
            }

            pos = new_pos;
        }

        let mut total = 0;
        for boxpos in grid.findv(if part2 { '[' } else { 'O' }) {
            let d = boxpos - b.top_left_cell();
            total += (d[0].abs()) + 100 * (d[1].abs());
        }

        cp(total);
    }
}
