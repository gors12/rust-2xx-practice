const WIDTH: usize = 30;
const HEIGHT: usize = 15;

#[test]
fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 {
                // Верхня та нижня лінії рамки
                output.push('*');
            } else if x == 0 || x == WIDTH - 1 {
                // Ліва та права сторони рамки
                output.push('*');
            } else if x == y * 2 || x == WIDTH.checked_sub(y * 2 + 1).unwrap_or(0) {
                // Діагоналі
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
