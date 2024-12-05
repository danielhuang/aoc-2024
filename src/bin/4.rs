use aoc_2024::*;

fn main() {
    let input = load_input();

    let grid = parse_grid(&input, |x| x, '.');

    let mut count = 0;

    for start in grid.findv('X') {
        for dir in Vector2::adj_with_diag() {
            let path = start.goto(start + dir * 3);
            let word = path.iter().map(|&x| grid[x]);
            if word.eq("XMAS".chars()) {
                count += 1;
            }
        }
    }

    cp(count);

    let mut count = 0;

    for start in grid.findv('A') {
        let corners = [v2(1, 1), v2(-1, -1), v2(-1, 1), v2(1, -1)];
        let corners = corners.map(|x| x + start);
        let corners = corners.map(|x| grid[x]);

        if (corners[0..2] == ['M', 'S'] || corners[0..2] == ['S', 'M'])
            && (corners[2..4] == ['M', 'S'] || corners[2..4] == ['S', 'M'])
        {
            count += 1;
        }
    }

    cp(count);
}
