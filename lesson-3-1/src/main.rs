fn calculate(a: i32, b: i32, op: char) -> i32 {
    match op {
        '+' => a.saturating_add(b),
        '-' => a.saturating_sub(b),
        '*' => a.saturating_mul(b),
        _ => 0,
    }
}

fn main() {
    assert_eq!(calculate(10, 20, '+'), 30);
    assert_eq!(calculate(100, 200, '+'), 300);
    assert_eq!(calculate(i32::MAX, 1, '+'), i32::MAX);

    assert_eq!(calculate(50, 30, '-'), 20);
    assert_eq!(calculate(i32::MIN, 1, '-'), i32::MIN);

    assert_eq!(calculate(5, 6, '*'), 30);
    assert_eq!(calculate(i32::MAX, 2, '*'), i32::MAX);

    assert_eq!(calculate(10, 20, '/'), 0);

    println!("Все тесты прошли успешно!");
}
