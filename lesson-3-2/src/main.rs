fn format_message(name: &str, score: u32, level: u32) -> String {
    format!("Привет, {}! Ваш счёт: {}, уровень: {}.", name, score, level)
}

fn build_greeting(name: &str, suffix: &str) -> String {
    let mut result = name.to_string();

    result.push(' ');
    result.push_str(suffix);

    return result;
}

fn main() {
    let msg1 = format_message("Дмитрий", 150, 5);

    println!("{}", msg1);

    let msg2 = build_greeting("Иван", ",добро пожаловать!");

    println!("{}", msg2);
}
