use aoc_2024::*;

#[test]
fn a() {
    let mut tree = KTree::default();
    for x in 0..100 {
        for y in 0..100 {
            tree.add(c2(x, y));
        }
    }
    assert_eq!(tree.count(bounds([p2(0, 0), p2(10, 10)])), 100);
    assert_eq!(tree.count(bounds([p2(25, 75), p2(50, 60)])), 25 * 15);
    assert_eq!(tree.count(bounds([p2(-100, -100), p2(20, 20)])), 20 * 20);
}
