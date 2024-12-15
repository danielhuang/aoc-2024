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

    let center = b.bottom_left_cell() + (b.top_right_point() - b.bottom_left_point()) / 2;

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
        let bots2 = bots.cii().map(|(pos, vel)| b.wrap(pos + vel * i)).cset();
        let mut sets = DisjointSet::using(bots2.cii());
        for bot in bots2.cii() {
            for adj in bot.adj().ii().filter(|x| bots2.contains(x)) {
                sets.join(bot, adj);
            }
        }
        let sets = sets.sets();
        let mut amounts = sets.ii().map(|x| x.len()).cv();
        amounts.sort();
        amounts.reverse();
        if amounts[0] > 20 && amounts[1] > 20 {
            cp(i);
            break;
        }
    }
}
