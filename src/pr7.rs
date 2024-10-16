fn draw_tree(num_triangles: usize) {
    // Генерація трикутників та визначення висоти останнього трикутника
    let max_height = 2 + (num_triangles - 1); // Висота останнього трикутника
    let max_width = 2 * max_height - 1; // Ширина останнього трикутника

    // Вирівнювання
    let offset = (max_width - 3) / 2; // Відстань для вирівнювання

    // (верхівка ялинки)
    for j in 0..3 {
        // Визначаємо кількість пробілів
        let spaces = (max_height - j - 1) + offset + if j > 0 { 1 } else { 0 };
        // Визначаємо кількість зірочок
        let stars = if j == 2 { 3 } else { 1 };

        // Виводимо рядок зірочок, вирівняний по центру
        println!("{:>width$}", "*".repeat(stars), width = spaces + stars);
    }

    // Генерація трикутників
    for i in 1..num_triangles {
        // Визначаємо кількість рядків для кожного трикутника
        let height = 2 + i;

        for j in 0..height {
            // Визначаємо кількість пробілів
            let spaces = (max_height - j - 1) + offset;
            // Визначаємо кількість зірочок
            let stars = 2 * j + 1;

            println!("{:>width$}", "*".repeat(stars), width = spaces + stars);
        }
    }
}

#[test]
fn main() {
    let num_triangles = 10;
    draw_tree(num_triangles);
}
