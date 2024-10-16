fn invert_the_case(input: String) -> String {
    input
        .chars() // Перетворення рядка у ітератор символів
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>().chars().next().unwrap() // Змінення верхніх регістрів на нижні
            } else {
                c.to_uppercase().collect::<String>().chars().next().unwrap() // Змінення нижніх регістрів на верхні
            }
        })
        .collect() // Збираємо символи назад у рядок
}

#[test]
fn test1() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}

#[test]
fn test2() {
    let original = "Hello World!".to_string();
    let inverted = invert_the_case(original.clone());

    println!("Original: {}", original); // Виводимо оригінальний текст
    println!("Inverted: {}", inverted);   // Виводимо змінений текст
}
