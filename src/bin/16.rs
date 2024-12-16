use aoc_2024::*;

fn main() {
    let input = load_input();

    let (mut grid, b) = input.grid(|x| x, '#');

    let start = grid.findv('S').one();
    let end = grid.findv('E').one();

    grid[start] = '.';
    grid[end] = '.';

    let mut seen = set([start, end]);

    let mut best_cost = INF;

    loop {
        for avoid_count in 0.. {
            dbg!(&avoid_count);
            for avoid in seen.cii().combinations(avoid_count) {
                let res = dijkstra(
                    [(start, v2(1, 0))],
                    |(pos, vel), _| {
                        let mut results = vec![];
                        for new_pos in pos.adj().cii().filter(|y| grid[y] == '.') {
                            if avoid.contains(&new_pos) {
                                continue;
                            }
                            let diff = new_pos - *pos;
                            if diff == *vel || diff == -*vel {
                                results.push(((new_pos, diff), 1));
                            } else {
                                results.push(((new_pos, diff), 1001));
                            }
                        }
                        results
                    },
                    |x| x.0 == end,
                );

                if let Some(res) = res {
                    if res.1 < best_cost {
                        best_cost = res.1;
                    }

                    // dbg!(res.1, best_cost);

                    if best_cost != INF && res.1 == best_cost {
                        for pos in res.0 {
                            seen.insert(pos.0);
                        }
                    }

                    dbg!(seen.len());
                }
            }
        }
    }

    // cp(res.1);
}
