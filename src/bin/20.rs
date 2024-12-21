use aoc_2024::*;

fn main() {
    let input = load_input();

    let (mut grid, _) = input.grid(|x| x, '#');

    let start = grid.findv('S').one();
    let end = grid.findv('E').one();

    grid[start] = '.';
    grid[end] = '.';

    let no_cheats_cost = bfs(
        [start],
        |x| x.adj().ii().filter(|to| grid[to] == '.'),
        |x| *x == end,
    )
    .unwrap()
    .len()
    .int()
        - 1;

    let reachable1 = bfs_reach([start], |x| x.adj().ii().filter(|to| grid[to] == '.')).cv();

    let reachable2: FxHashMap<_, _> =
        bfs_reach([end], |x| x.adj().ii().filter(|to| grid[to] == '.')).collect();

    for cheat_time_limit in [2, 20] {
        let mut count = 0;

        for (p1, cost1) in reachable1.iter().copied() {
            for dx in -cheat_time_limit..=cheat_time_limit {
                for dy in -(cheat_time_limit - dx.abs())..=(cheat_time_limit - dx.abs()) {
                    let p2 = p1 + v2(dx, dy);
                    if let Some(cost2) = reachable2.get(&p2).copied() {
                        if (p2 - p1).manhat() + cost1 as Z + cost2 as Z + 100 <= no_cheats_cost {
                            count += 1;
                        }
                    }
                }
            }
        }

        cp(count);
    }
}
