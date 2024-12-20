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

    let reachable2 = bfs_reach([end], |x| x.adj().ii().filter(|to| grid[to] == '.')).cv();

    for cheat_time_limit in [2, 20] {
        let mut count = 0;

        for (p1, cost1) in reachable1.clone() {
            for (p2, cost2) in reachable2.clone() {
                if (p2 - p1).manhat() <= cheat_time_limit
                    && (p2 - p1).manhat() + cost1.int() + cost2.int() + 100 <= no_cheats_cost.int()
                {
                    count += 1;
                }
            }
        }

        cp(count);
    }
}
