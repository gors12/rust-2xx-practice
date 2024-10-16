struct Point {
    x: i32,
    y: i32,
}

// прямокутник
struct Rectangle {
    a: Point, // ліва верхня точка
    b: Point, // права нижня точка
}

// Функція для обчислення фактичної зайнятої площі
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    // Створення матриці для позначення зайнятих квадратів
    let mut grid = vec![vec![false; 100]; 100];

    // Заповнення матриці зайнятої площі
    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                grid[y as usize][x as usize] = true;
            }
        }
    }

    // Обчислюємо фактично зайняту площу
    let mut occupied_area = 0;
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell {
                occupied_area += 1;
            }
        }
    }

    occupied_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
