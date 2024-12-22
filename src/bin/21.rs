use aoc_2024::*;

fn map1(a: char) -> Cell2 {
    match a {
        '0' => c2(1, 0),
        '1' => c2(0, 1),
        '2' => c2(1, 1),
        '3' => c2(2, 1),
        '4' => c2(0, 2),
        '5' => c2(1, 2),
        '6' => c2(2, 2),
        '7' => c2(0, 3),
        '8' => c2(1, 3),
        '9' => c2(2, 3),
        'A' => c2(2, 0),
        _ => unreachable!(),
    }
}

fn map2(a: char) -> Cell2 {
    match a {
        '^' => c2(1, 1),
        'v' => c2(1, 0),
        '<' => c2(0, 0),
        '>' => c2(2, 0),
        'A' => c2(2, 1),
        _ => unreachable!(),
    }
}

fn is_valid1(x: Cell2) -> bool {
    matches!(
        x,
        Cell([1, 0])
            | Cell([0, 1])
            | Cell([1, 1])
            | Cell([2, 1])
            | Cell([0, 2])
            | Cell([1, 2])
            | Cell([2, 2])
            | Cell([0, 3])
            | Cell([1, 3])
            | Cell([2, 3])
            | Cell([2, 0])
    )
}

fn is_valid2(x: Cell2) -> bool {
    matches!(
        x,
        Cell([1, 1]) | Cell([1, 0]) | Cell([0, 0]) | Cell([2, 0]) | Cell([2, 1])
    )
}

fn search(
    layers: usize,
    last_pressed_location: Cell2,
    to_press: char,
    keypad: bool,
    cache: &mut FxHashMap<(usize, Cell2, char, bool), Z>,
) -> Z {
    if layers == 0 {
        1
    } else {
        if let Some(&ans) = cache.get(&(layers, last_pressed_location, to_press, keypad)) {
            return ans;
        }

        let map = |x| {
            if keypad { map1(x) } else { map2(x) }
        };

        let is_valid = |x| {
            if keypad { is_valid1(x) } else { is_valid2(x) }
        };

        let path = dijkstra(
            [(last_pressed_location, 'A', false)],
            |&(prev, finger, done), _| {
                if done {
                    return vec![(
                        (map2('A'), 'A', true),
                        search(layers - 1, map2(finger), 'A', false, cache),
                    )];
                }

                prev.adj()
                    .ii()
                    .chain([prev])
                    .filter(|x| is_valid(*x))
                    .map(|next| {
                        if next == prev.up(1) {
                            (
                                (next, '^', next == map(to_press)),
                                search(layers - 1, map2(finger), '^', false, cache),
                            )
                        } else if next == prev.down(1) {
                            (
                                (next, 'v', next == map(to_press)),
                                search(layers - 1, map2(finger), 'v', false, cache),
                            )
                        } else if next == prev.left(1) {
                            (
                                (next, '<', next == map(to_press)),
                                search(layers - 1, map2(finger), '<', false, cache),
                            )
                        } else if next == prev.right(1) {
                            (
                                (next, '>', next == map(to_press)),
                                search(layers - 1, map2(finger), '>', false, cache),
                            )
                        } else if next == prev {
                            ((next, finger, next == map(to_press)), 1)
                        } else {
                            unreachable!()
                        }
                    })
                    .cv()
            },
            |x| x.1 == 'A' && x.2,
        )
        .unwrap();

        cache.insert((layers, last_pressed_location, to_press, keypad), path.1);
        path.1
    }
}

fn main() {
    let input = load_input();

    let mut cache = FxHashMap::default();

    for layers in [3, 26] {
        let mut ans = 0;

        for line in input.lines() {
            let mut cost = 0;

            cost += search(
                layers,
                map1('A'),
                line.chars().next().unwrap(),
                true,
                &mut cache,
            );

            for (a, b) in line.chars().tuple_windows() {
                let start = map1(a);
                cost += search(layers, start, b, true, &mut cache);
            }

            let num = line.ints().one();
            ans += num * cost;
        }

        cp(ans);
    }
}
