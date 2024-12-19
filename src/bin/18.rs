use aoc_2024::*;

fn search(corrupted: &HashMap<Cell2, usize>, b: Cuboid<2>, limit: usize) -> Option<Vec<Cell2>> {
    let start = b.top_left_cell();
    let end = b.bottom_right_cell();

    bfs(
        [start],
        |x| {
            x.adj()
                .ii()
                .filter(|&y| b.contains(y) && corrupted.get(&y).is_none_or(|&z| z > limit))
        },
        |x| *x == end,
    )
}

fn main() {
    let input = load_input();

    let mut corrupted = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let [a, b] = line.ints().ca();

        corrupted.insert(c2(a, -b), i);
    }

    let b = bounds(corrupted.keys().cloned());

    let path = search(&corrupted, b, 1024);
    cp(path.unwrap().len() - 1);

    let mut good = 0;
    let mut bad = corrupted.len();

    while bad - good > 1 {
        let mid = (bad + good) / 2;

        let path = search(&corrupted, b, mid);

        if path.is_some() {
            good = mid;
        } else {
            bad = mid;
        }
    }

    let (&bad_cell, _) = corrupted.iter().find(|&(_, &i)| i == bad).unwrap();

    cp(format!("{},{}", bad_cell[0], -bad_cell[1]));
}
