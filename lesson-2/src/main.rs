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

fn main() {
    for n in 1..=20 {
        fuzz_buzz(n);
    }
}
