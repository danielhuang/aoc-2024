use aoc_2024::*;

fn search(
    grid: &DefaultHashMap<Cell2, char>,
    start: impl IntoIterator<Item = (Cell2, Vector2)>,
) -> impl Iterator<Item = DijkstraReachableItem<(Cell2, Vector2), Z>> {
    dijkstra_reach(start, |&(pos, vel), _| {
        let mut results = vec![];
        results.push(((pos, vel.turn_left()), 1000));
        results.push(((pos, vel.turn_right()), 1000));
        if grid[pos + vel] == '.' {
            results.push(((pos + vel, vel), 1));
        }
        results
    })
}

fn main() {
    let input = load_input();

    let (mut grid, _) = input.grid(|x| x, '#');

    let start = grid.findv('S').one();
    let end = grid.findv('E').one();

    grid[start] = '.';
    grid[end] = '.';

    let search_end = search(&grid, [(start, v2(1, 0))])
        .find(|x| x.node.0 == end)
        .unwrap();

    let min_cost = search_end.total_cost;
    let end = search_end.node;

    cp(min_cost);

    let from_start: HashMap<_, _> = search(&grid, [(start, v2(1, 0))])
        .take_while(|x| x.total_cost <= min_cost)
        .map(|x| (x.node, x.total_cost))
        .collect();

    let from_end = search(&grid, [(end.0, -end.1)]).take_while(|x| x.total_cost <= min_cost);

    let mut good_nodes = set([]);

    for mut node in from_end {
        node.node.1 = -node.node.1;
        if let Some(cost_from_start) = from_start.get(&node.node) {
            let total_cost = node.total_cost + cost_from_start;
            if total_cost <= min_cost {
                good_nodes.insert(node.node.0);
            }
        }
    }

    cp(good_nodes.len());
}
