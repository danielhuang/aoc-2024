use aoc_2024::*;

#[cached]
fn calc(head: Vec<String>, line: String) -> usize {
    if line.is_empty() {
        1
    } else {
        head.cii()
            .map(|prefix| {
                let mut c = 0;
                if let Some(rest) = line.strip_prefix(&prefix) {
                    c += calc(head.to_vec(), rest.tos());
                }
                c
            })
            .sumi()
    }
}

fn main() {
    let input = load_input();

    let [head, body] = input.paras().ca();

    let head = head.alphanumeric_words();

    let mut count = 0;
    let mut total = 0;

    for line in body.lines() {
        let x = calc(head.clone(), line.tos());
        if x > 0 {
            count += 1;
        }
        total += x;
    }

    cp(count);
    cp(total);
}
