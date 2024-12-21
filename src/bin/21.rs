use aoc_2024::*;

fn map1(a: char) -> Cell2 {
    // dbg!(a);
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
        _ => todo!(),
    }
}

fn map2(a: char) -> Cell2 {
    match a {
        '^' => c2(1, 1),
        'v' => c2(1, 0),
        '<' => c2(0, 0),
        '>' => c2(2, 0),
        'A' => c2(2, 1),
        _ => {
            dbg!(a);
            todo!()
        }
    }
}

#[cached]
fn search2(layers: usize, last_pressed_location: Cell2, to_press: char) -> Z {
    let map2_reverse: HashMap<_, _> = "<>v^A".chars().map(|x| (map2(x), x)).collect();

    if layers == 0 {
        1
    } else {
        let path = dijkstra(
            [(last_pressed_location, 'A', false)],
            |&(prev, finger, done), _| {
                if done {
                    return vec![(
                        (map2('A'), 'A', true),
                        search2(layers - 1, map2(finger), 'A'),
                    )];
                }
                let results = prev
                    .adj()
                    .ii()
                    .chain([prev])
                    .filter(|x| map2_reverse.contains_key(x))
                    .map(|next| {
                        if next == prev.up(1) {
                            (
                                (next, '^', next == map2(to_press)),
                                search2(layers - 1, map2(finger), '^'),
                            )
                        } else if next == prev.down(1) {
                            (
                                (next, 'v', next == map2(to_press)),
                                search2(layers - 1, map2(finger), 'v'),
                            )
                        } else if next == prev.left(1) {
                            (
                                (next, '<', next == map2(to_press)),
                                search2(layers - 1, map2(finger), '<'),
                            )
                        } else if next == prev.right(1) {
                            (
                                (next, '>', next == map2(to_press)),
                                search2(layers - 1, map2(finger), '>'),
                            )
                        } else if next == prev {
                            ((next, finger, next == map2(to_press)), 1)
                        } else {
                            todo!()
                        }
                    })
                    .cv();
                // dbg!(layers, &prev, &results);
                results
            },
            |x| x.1 == 'A' && x.2,
        )
        .unwrap();
        // dbg!(
        //     layers,
        //     map2_reverse[&last_pressed_location],
        //     to_press,
        //     &path
        // );
        path.1
    }
}

#[cached]
fn search1(layers: usize, last_pressed_location: Cell2, to_press: char) -> Z {
    let map1_reverse: HashMap<_, _> = "0123456789A".chars().map(|x| (map1(x), x)).collect();

    let map2_reverse: HashMap<_, _> = "<>v^A".chars().map(|x| (map2(x), x)).collect();

    let path = dijkstra(
        [(last_pressed_location, 'A', false)],
        |&(prev, finger, done), _| {
            if done {
                return vec![(
                    (map2('A'), 'A', true),
                    search2(layers - 1, map2(finger), 'A'),
                )];
            }
            let results = prev
                .adj()
                .ii()
                .chain([prev])
                .filter(|x| map1_reverse.contains_key(x))
                .map(|next| {
                    if next == prev.up(1) {
                        (
                            (next, '^', next == map1(to_press)),
                            search2(layers - 1, map2(finger), '^'),
                        )
                    } else if next == prev.down(1) {
                        (
                            (next, 'v', next == map1(to_press)),
                            search2(layers - 1, map2(finger), 'v'),
                        )
                    } else if next == prev.left(1) {
                        (
                            (next, '<', next == map1(to_press)),
                            search2(layers - 1, map2(finger), '<'),
                        )
                    } else if next == prev.right(1) {
                        (
                            (next, '>', next == map1(to_press)),
                            search2(layers - 1, map2(finger), '>'),
                        )
                    } else if next == prev {
                        ((next, finger, next == map1(to_press)), 1)
                    } else {
                        todo!()
                    }
                })
                .cv();
            // dbg!(layers, &prev, &results);
            results
        },
        |x| x.1 == 'A' && x.2,
    )
    .unwrap();
    // dbg!(
    //     layers,
    //     map2_reverse[&last_pressed_location],
    //     to_press,
    //     &path
    // );
    path.1
}

fn main() {
    let input = load_input();

    let mut all = 0;

    // dbg!(&map2_reverse);

    let test = "<A";

    dbg!(search2(1, map2('A'), '>'));
    dbg!(search2(1, map2('>'), 'v'));
    dbg!(search2(1, map2('v'), '<'));
    dbg!(search2(1, map2('A'), '<'));
    dbg!(search2(2, map2('A'), '<'));

    // <
    // v<<A

    // v<A<AA^>>A
    // v<<A
    // dbg!(search2(3, map2('A'), '<'));

    let mut cost = 0;
    cost += search2(2, map2('A'), '<');
    for (a, b) in "<A^A>^^AvvvA".chars().tuple_windows() {
        let start = map2(a);
        cost += search2(2, start, b);
    }
    dbg!(&cost);

    // todo!();

    for line in input.lines() {
        let mut cost = 0;

        cost += search1(26, map1('A'), line.chars().next().unwrap());

        for (a, b) in line.chars().tuple_windows() {
            let start = map1(a);
            cost += search1(26, start, b);
        }

        dbg!(&cost);

        let num = line.ints().one();
        // dbg!(&path);
        all += num * cost;
        // todo!();
    }

    cp(all);
}
