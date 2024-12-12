use aoc_2024::*;

fn split(x: Z) -> (Z, Z) {
    let digits = x.ilog10() + 1;
    let d = (10).pow(digits / 2);
    (x / d, x % d)
}

fn main() {
    let input = load_input();

    for blinks in [25, 75] {
        let mut stones = freqs(input.ints());

        for _ in 0..blinks {
            let mut new_stones = DefaultHashMap::new(0);
            for (stone, count) in stones {
                if stone == 0 {
                    new_stones[1] += count;
                } else if stone.ilog10() % 2 == 1 {
                    let (left, right) = split(stone);
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
