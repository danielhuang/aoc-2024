use aoc_2024::*;

fn main() {
    let input = load_input();

    let mut id = 0;

    let mut sizes = vec![];

    let mut free = false;

    for len in input.trim().chars() {
        if free {
            for _ in 0..len.int() {
                sizes.push(None);
            }
        } else {
            for _ in 0..len.int() {
                sizes.push(Some(id));
            }
            id += 1;
        }
        free = !free;
    }

    {
        let mut sizes = sizes.clone();
        let mut i = 0;

        'outer: loop {
            while sizes.last().unwrap().is_none() {
                sizes.pop().unwrap();
            }
            let to_move = sizes.pop().unwrap();
            while {
                if i >= sizes.len() {
                    sizes.push(to_move);
                    break 'outer;
                }
                sizes[i].is_some()
            } {
                i += 1;
            }
            sizes[i] = to_move;
        }

        cp(sizes
            .cii()
            .enumerate()
            .map(|(i, size)| i * size.unwrap_or(0).uint())
            .sumi());
    }

    for move_id in (0..=id).rev() {
        let len = sizes.cii().filter(|x| *x == Some(move_id)).count();
        for i in 0..sizes.len() {
            if sizes[i] == Some(move_id) {
                break;
            }
            if (i..(i + len)).all(|x| sizes[x].is_none()) {
                for x in sizes.iter_mut() {
                    if *x == Some(move_id) {
                        *x = None;
                    }
                }
                sizes[i..(i + len)].fill(Some(move_id));
                break;
            }
        }
    }

    cp(sizes
        .cii()
        .enumerate()
        .map(|(i, size)| i * size.unwrap_or(0).uint())
        .sumi());
}
