use aoc_2024::*;

fn main() {
    let input = load_input();

    let mut total = 0;

    let mut prices_list = vec![];

    for mut n in input.ints() {
        let mut prices = vec![n];
        for _ in 0..2000 {
            n ^= n * 64;
            n %= 16777216;
            n ^= n / 32;
            n %= 16777216;
            n ^= n * 2048;
            n %= 16777216;
            prices.push(n % 10);
        }

        prices_list.push(prices);
        total += n;
    }

    cp(total);

    let mut options = HashSet::new();
    let mut found_in = defaultdict(set([]));
    let mut indexes = vec![];

    for (j, prices) in prices_list.cii().enumerate() {
        let price_changes = derivative_excl_first(&prices);
        let mut option_indexes = FxHashMap::default();

        for (i, option) in price_changes.windows(4).ii().enumerate().rev() {
            options.insert(option.to_vec());
            if i > 0 {
                option_indexes.insert(option.to_vec(), i);
                found_in[option.to_vec()].insert(j);
            }
        }

        indexes.push(option_indexes);
    }

    let ans = options
        .iter()
        .map(|o| {
            let mut total = 0;
            for j in found_in[o].cii() {
                total += prices_list[j][indexes[j][o]];
            }
            total
        })
        .max()
        .unwrap();

    cp(ans);
}
