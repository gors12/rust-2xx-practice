fn is_palindrome(x: u32) -> bool {
    let s = x.to_string(); // Перетворюємо число в рядок
    s == s.chars().rev().collect::<String>() // Порівнюємо рядок з його реверсованою версією
}

#[test]
fn test1() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];


    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}

#[test]
fn test2() {
    let number = 121;
    if is_palindrome(number) {
        println!("{} є паліндромом.", number);
    } else {
        println!("{} не є паліндромом.", number);
    }
}
