use aoc_2024::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = load_input();

    let mut total = 0;

    let mut hs = vec![];

    for n in input.ints() {
        let mut h = vec![n];
        let mut n = n;
        for _ in 0..2000 {
            n ^= n * 64;
            n %= 16777216;
            n ^= n / 32;
            n %= 16777216;
            n ^= n * 2048;
            n %= 16777216;
            h.push(n % 10);
        }

        hs.push(h);
        total += n;
    }

    cp(total);

    let mut options = HashSet::new();

    let mut indexes = vec![];

    for h in hs.cii() {
        let d = derivative_excl_first(&h);
        let mut option_indexes = FxHashMap::default();

        for (i, w) in d.windows(4).ii().enumerate().rev() {
            options.insert(w.to_vec());
            if i > 0 {
                option_indexes.insert(w.to_vec(), i);
            }
        }

        indexes.push(option_indexes);
    }

    let ans = options
        .iter()
        .map(|o| {
            if sometimes() {
                dbg!(options.len());
            }
            let mut total = 0;
            for (j, h) in hs.iter().enumerate() {
                if let Some(&i) = indexes[j].get(o) {
                    total += h[i];
                }
            }
            total
        })
        .max()
        .unwrap();

    cp(ans);
}
