const SIZE: usize = 6; // Висота ромба (розмір центральної частини)

#[test]
fn test() {
    let mut result = String::new();

    // Верхня частина ромба
    for i in 0..SIZE {
        result.push_str(&" ".repeat(SIZE - i - 1));
        result.push_str(&"*".repeat(2 * i + 1));
        result.push('\n');
    }

    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        result.push_str(&" ".repeat(SIZE - i - 1));
        result.push_str(&"*".repeat(2 * i + 1));
        result.push('\n');
    }

    print!("{}", result);
}

