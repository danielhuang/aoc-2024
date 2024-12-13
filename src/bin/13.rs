use aoc_2024::*;

fn main() {
    let input = load_input();

    for offset in [0, 10000000000000] {
        let mut cost = 0;

        for para in input.paras() {
            let [ax, ay, bx, by, x, y] = para.ints().ii().map(|x| Q::new(x, 1)).ca();
            let x = x + offset;
            let y = y + offset;
            let a_count = (bx * y - x * by) / (bx * ay - ax * by);
            let b_count = (x * ay - ax * y) / (bx * ay - ax * by);
            if a_count.is_integer() && b_count.is_integer() {
                cost += a_count.int() * 3 + b_count.int();
            }
        }

        cp(cost);
    }
}
