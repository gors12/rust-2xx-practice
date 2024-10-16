use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, usize, usize)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = i32::MAX;
    let mut indices = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            indices = (i, i + 1);
        }
    }

    Some((min_sum, indices.0, indices.1))
}

fn print_data(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{}.", i)).collect::<String>());
    println!("data:  {:?}", data);

    if let Some((min_sum, idx1, idx2)) = min_adjacent_sum(data) {
        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            data[idx1], data[idx2], min_sum, idx1, idx2
        );
    }
}

#[test]
fn main() {
    let vector = gen_random_vector(20);
    print_data(&vector);
}
