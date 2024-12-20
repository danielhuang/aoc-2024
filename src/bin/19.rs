use aoc_2024::*;

fn strip_prefix<'a>(all: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.len() > all.len() {
        return None;
    }

    if prefix.is_empty() {
        return Some(all);
    }

    for i in 0..prefix.len() {
        if all.as_bytes()[i] != prefix.as_bytes()[i] {
            return None;
        }
    }

    Some(&all[prefix.len()..])
}

fn calc<'a>(parts: &[String], line: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(&ans) = cache.get(&line) {
        return ans;
    }

    let ans = if line.is_empty() {
        1
    } else {
        parts
            .iter()
            .map(|prefix| {
                strip_prefix(line, prefix)
                    .map(|rest| calc(parts, rest, cache))
                    .unwrap_or(0)
            })
            .sumi()
    };

    cache.insert(line, ans);

    ans
}

fn main() {
    let input = load_input();

    let [head, body] = input.paras().ca();

    let parts = head.alphanumeric_words();

    let mut count = 0;
    let mut total = 0;

    let mut cache = HashMap::new();

    for line in body.lines() {
        let x = calc(&parts, line, &mut cache);
        if x > 0 {
            count += 1;
        }
        total += x;
    }

    cp(count);
    cp(total);
}
