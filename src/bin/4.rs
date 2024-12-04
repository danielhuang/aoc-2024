use aoc_2024::*;
use stringtools::Stringtools;

fn main() {
    let input = load_input();

    let grid = parse_grid(&input, |x| x, '.');

    let mut count = 0;

    for start in grid.find(|x| "XMAS".contains(x)) {
        for dir in Vector2::adj_with_diag() {
            let path = start.goto(start + dir * 3);
            let word = path.cii().map(|x| grid[x]).cstr();
            if word == "XMAS" || word == "SAMX" {
                count += 1;
            }
        }
    }

    assert!(count % 2 == 0);
    cp(count / 2);

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
