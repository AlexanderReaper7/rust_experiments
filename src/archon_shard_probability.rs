use rand::Rng;
pub fn main() {
    let report_numbers: Vec<usize> = vec![1,2,3,4,5,6,7,8,9,10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000,];
    let mut rand = rand::thread_rng();
    let mut current_probability = 0.2;
    let mut true_count: usize = 0;
    for i in 1.. {
        if rand.gen_bool(current_probability) {
            true_count += 1;
            current_probability = 0.2;
        } else {
            current_probability += 0.2;
        }
        // on every report number, report the true probability, and every 100 million.
        if report_numbers.contains(&i) || (|x| x % 100_000_000 == 0)(i){
            println!("{}: {}", i, true_count as f64 / i as f64);
        }
    }
}