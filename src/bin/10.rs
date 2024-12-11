use aoc_2024::*;

fn main() {
    let input = load_input();

    let grid = parse_grid(&input, |x| x.int(), -1);

    let mut part1 = 0;

    for head in grid.findv(0) {
        for (pos, _) in bfs_reach([head], |x| {
            x.adj().ii().filter(|y| grid[y] - grid[x] == 1).cv()
        }) {
            if grid[pos] == 9 {
                part1 += 1;
            }
        }
    }

    cp(part1);

    let part2 = count_paths(
        grid.findv(0),
        |x| x.adj().ii().filter(|y| grid[y] - grid[x] == 1).cv(),
        |x| grid[x] == 9,
    );

    cp(part2);
}
