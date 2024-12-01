use aoc_2024::*;

#[test]
fn simple() {
    let matrix = Matrix::new([v2(1, 0), v2(0, 1), v2(0, 0)]);
    assert_eq!(matrix * v3(1, 2, 3), v2(1, 2));
    assert_eq!(matrix * v3(0, 0, 3), v2(0, 0));
}

#[test]
fn simple2() {
    let duplicate = Matrix::new([v3(1, 1, 1)]);
    let collapse = Matrix::new([v1(1), v1(1), v1(-1)]);

    assert_eq!(collapse * duplicate, Matrix::new([v1(1)]));
}
