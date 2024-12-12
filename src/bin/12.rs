use aoc_2024::*;

fn main() {
    let input = load_input();

    let mut grid = parse_grid(&input, |x| x, ' ');

    let mut groups = vec![];

    while let Some(unused) = grid.find(|x| x != ' ').first().cloned() {
        let region = bfs_reach([unused], |&x| {
            x.adj().cii().filter(|y| grid[y] == grid[x]).cv()
        })
        .cv();

        let mut group = vec![];

        for c in region {
            grid[c.0] = ' ';
            group.push(c.0);
        }

        groups.push(group);
    }

    let mut price = 0;
    let mut price2 = 0;

    for group in groups {
        let group = group.cset();
        let area = group.len().int();
        let mut perimeter = 0;
        for c in group.cii() {
            for adj in c.adj() {
                if !group.contains(&adj) {
                    perimeter += 1;
                }
            }
        }
        let sides = group
            .cii()
            .flat_map(|x| x.corners())
            .unique()
            .map(|x| {
                let mut count = 0;
                for c in x.corner_of() {
                    if group.contains(&c) {
                        count += 1;
                    }
                }
                let mut count2 = 0;
                for c in x.corner_minmax_of() {
                    if group.contains(&c) {
                        count2 += 1;
                    }
                }
                if count == 1 || count == 3 {
                    1
                } else if count == 2 && (count2 == 0 || count2 == 2) {
                    2
                } else {
                    0
                }
            })
            .sumi();
        price += area * perimeter;
        price2 += area * sides;
    }

    cp(price);
    cp(price2);
}
