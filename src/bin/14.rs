use aoc_2024::*;

fn main() {
    let input = load_input();

    let mut bots = vec![];

    for line in input.lines() {
        let [px, py, vx, vy] = line.ints().ca();
        let pos = c2(px, py);
        let vel = v2(vx, vy);
        bots.push((pos, vel));
    }

    let b = bounds(bots.cii().map(|x| x.0));

    let center = b.bottom_left_cell() + b.top_right_cell().vector() / 2;

    let mut map = DefaultHashMap::new(0);

    for (pos, vel) in bots.cii() {
        let new_pos = pos + vel * 100;
        let new_pos = b.wrap(new_pos);
        if new_pos[0] != center[0] && new_pos[1] != center[1] {
            map[(new_pos[0] < center[0], new_pos[1] < center[1])] += 1;
        }
    }

    cp(map.values().copied().product::<Z>());

    for i in 0.. {
        let bots2 = bots.cii().map(|(pos, vel)| b.wrap(pos + vel * i)).cv();
        if bounds(bots2.cii()).volume() < b.volume()
            && bfs_reach([center], |x| x.adj().ii().filter(|y| bots2.contains(y))).count() > 30
        {
            cp(i);
            break;
        }
    }
}
