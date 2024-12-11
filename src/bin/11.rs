use aoc_2024::*;

fn main() {
    let input = load_input();

    for blinks in [25, 75] {
        let mut stones = freqs(input.ints());

        for _ in 0..blinks {
            let mut new_stones = DefaultHashMap::new(0);
            for (stone, count) in stones {
                if stone == 0 {
                    new_stones[1] += count;
                } else if stone.to_string().len() % 2 == 0 {
                    let stone = stone.to_string();
                    let half_len = stone.len() / 2;
                    let left = (&stone[..half_len]).int();
                    let right = (&stone[half_len..]).int();
                    new_stones[left] += count;
                    new_stones[right] += count;
                } else {
                    new_stones[stone * 2024] += count;
                }
            }
            stones = new_stones;
        }

        cp(stones.values().copied().sumi());
    }
}
