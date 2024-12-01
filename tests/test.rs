use aoc_2024::*;

#[test]
fn intervals() {
    let mut intervals = Intervals::default();
    intervals.add(0, 10);
    intervals.add(10, 20);
    assert!(intervals.iter().cv() == (0..20).cv());
}

#[test]
fn intervals2() {
    let mut intervals = Intervals::default();
    intervals.add(0, 20);
    intervals.remove(5, 10);
    assert!(intervals.iter().cv() == (0..5).chain(10..20).cv());
}

#[test]
fn intervals3() {
    let mut intervals = Intervals::default();
    intervals.add(0, 20);
    intervals.remove(0, 20);
    assert!(intervals.iter().cv().is_empty());
}

#[test]
fn intervals4() {
    let mut intervals = Intervals::default();
    intervals.add(0, 20);
    intervals.remove(-1, 21);
    assert!(intervals.iter().cv().is_empty());
}

#[test]
fn intervals5() {
    let mut intervals = Intervals::default();
    intervals.add(0, 20);
    intervals.remove(5, 20);
    assert!(intervals.iter().cv() == (0..5).cv());
}

#[test]
fn intervals6() {
    let mut intervals = Intervals::default();
    intervals.add(0, 5);
    intervals.add(6, 20);
    assert!(intervals.iter().cv() == (0..5).chain(6..20).cv());
}

#[test]
fn intervals7() {
    let mut a = Intervals::default();
    a.add(0, 5);
    let mut b = Intervals::default();
    b.add(5, 10);
    let intervals = Intervals::union(a, b);
    assert!(intervals.iter().cv() == (0..10).cv());
}

#[test]
fn intervals_iter() {
    let mut a = Intervals::default();
    a.add(1, 4);
    a.add(5, 10);
    a.add(15, 20);
    assert!(a.iter().cv() == (1..4).chain(5..10).chain(15..20).cv());
    assert!(a.iter().rev().cv() == (1..4).chain(5..10).chain(15..20).rev().cv());
}

#[test]
fn area() {
    assert_eq!(area_points([p2(0, 0), p2(0, 2), p2(2, 2), p2(2, 0)]), 4);
    assert_eq!(
        area_with_border([c2(0, 0), c2(0, 2), c2(2, 2), c2(2, 0)]),
        9
    );
    assert_eq!(
        area_without_border([c2(0, 0), c2(0, 2), c2(2, 2), c2(2, 0)]),
        1
    );

    let input = "R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)";

    let mut corners = vec![];
    let mut cur = c2(0, 0);
    for line in input.lines() {
        let [dir, len, _] = line.words().ca();
        let dir = charvel(dir);
        let len = len.int();
        corners.push(cur);
        cur += dir * len;
    }

    assert_eq!(area_with_border(corners), 62);
}

#[test]
fn y23d17() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let grid = parse_grid(input, |x| x.int(), 99999);
    let b = bounds(grid.keys().cloned());
    let end = b.bottom_right_cell();

    dbg!(&end);

    let result = dijkstra(
        [
            (b.top_left_cell(), v2(0, -1)),
            (b.top_left_cell(), v2(1, 0)),
        ],
        |&(pos, vel), _| {
            let mut results = vec![];
            for distance in 1..=3 {
                for vel in [vel.turn_left(), vel.turn_right()] {
                    let dest = vel * distance + pos;
                    let cost = pos.goto(dest).cii().skip(1).map(|x| grid[x]).sumi();
                    results.push(((dest, vel), cost));
                }
            }
            results
        },
        |x| x.0 == end,
    )
    .unwrap();

    assert_eq!(result.1, 102);
}

#[test]
fn cuboid_corners() {
    let b = bounds([c2(0, 0), c2(2, 2)]);
    assert_eq!((b.top_right_point() - b.bottom_left_point()).manhat(), 6);
    assert_eq!((b.top_left_point() - b.bottom_right_point()).manhat(), 6);

    assert_eq!((b.top_right_cell() - b.bottom_left_cell()).manhat(), 4);
    assert_eq!((b.top_left_cell() - b.bottom_right_cell()).manhat(), 4);
}
