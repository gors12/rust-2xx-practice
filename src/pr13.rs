use rand::Rng;

fn gen_balanced_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![4; n]; // базове значення для генерації
    let mut rng = rand::thread_rng();

    for i in 0..n {
        shipments[i] += rng.gen_range(0..=5); // випадкове число
    }

    let total: u32 = shipments.iter().sum();
    let remainder = (total % n as u32) as usize;
    shipments[remainder] += n as u32 - (total % n as u32);

    shipments
}

fn count_permutation(shipments: &[u32]) -> Option<(usize, Vec<String>)> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    if total % n != 0 {
        return None; // Якщо не можна рівномірно розподілити
    }
    let average = total / n;
    let mut moves = 0;
    let mut balance = 0;
    let mut steps = Vec::new();

    for &load in shipments {
        let diff = average as i32 - load as i32;
        balance += diff;
        // Додаємо значення до steps як строки
        steps.push(diff.to_string());
        moves += balance.abs() as usize;
    }

    Some((moves, steps))
}

fn print_distribution(shipments: &[u32]) {
    println!("{:?}", shipments);

    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    println!(
        "total   = {}",
        shipments.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" + ")
    );
    println!("        = {}", total);
    println!("average = {} / {} = {}", total, n, total / n);

    if let Some((moves, steps)) = count_permutation(shipments) {
        println!("\n[");
        for &load in shipments {
            print!("    {:<2},", load);
        }
        println!("\n]");

        // Форматування зсувів
        for step in steps.iter() {
            if step.parse::<i32>().unwrap() >= 0 {
                print!("{:>4} ", format!("+{}", step));
            } else {
                print!("{:>4} ", step);
            }
        }
        println!("\n\nanswer = {}", moves);
    } else {
        println!("Cannot evenly distribute cargo across all ships.");
    }
}

#[test]
fn main() {
    let shipments = gen_balanced_shipments(5);
    print_distribution(&shipments);
}
