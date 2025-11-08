fn fuzz_buzz(value: i32) {
    let divisible_by_3 = value % 3 == 0;
    let divisible_by_5 = value % 5 == 0;

    if divisible_by_3 && divisible_by_5 {
        println!("FizzBuzz");

        return;
    }

    if divisible_by_3 {
        println!("Fizz");

        return;
    }

    if divisible_by_5 {
        println!("Buzz");

        return;
    }
}

fn format_data() {
    let year = 2024;
    let month = 1;
    let day = 15;

    println!("📅 {}-{:02}-{:02}", year, month, day)
}

fn format_money() {
    let amount = 142.1211;

    println!("💵 {:.2} ₽", amount)
}

fn format_rgb() {
    let r = 255;
    let g = 128;
    let b = 0;

    println!("🎨 #{:02X}{:02X}{:02X}", r, g, b)
}

fn format_table() {
    let name = "Alice";
    let age = 25;
    let scope = 95.543;

    println!("| {:<8} | {:^8} | {:>8.1} | ", name, age, scope)
}

fn main() {
    let divider = "-------------";

    // 1. FuzzBuzz
    fuzz_buzz(5);
    println!("{}", divider);

    // 2. Format data
    format_data();
    println!("{}", divider);

    // 2. Format money
    format_money();
    println!("{}", divider);

    // 2. Format rgb
    format_rgb();
    println!("{}", divider);

    // 2. Format table
    format_table();
    println!("{}", divider);
}
