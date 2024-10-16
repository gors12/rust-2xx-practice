fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize; // Довжина рядка

    // Якщо рядок порожній, повертаємо його без змін
    if len == 0 {
        return s;
    }

    // Нормалізуємо значення n, щоб бути в межах довжини рядка
    let n = ((n % len) + len) % len;

    // Розділяємо рядок на дві частини і об'єднуємо їх у зворотному порядку
    let split_index = (len - n) as usize; // Індекс для розділення
    let (left, right) = s.split_at(split_index);

    format!("{}{}", right, left) // Повертаємо обернений рядок
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(
            rotate(s.to_string(), *n),
            exp.to_string()
        );
    });
}
