pub fn eratosthenes(limit: u64) -> Vec<bool> {
    let n: usize = limit as usize;
    let mut prime_list: Vec<bool> = vec![true; n + 1];

    prime_list[0] = false;
    prime_list[1] = false;

    for index in 2..=n.isqrt() {
        if prime_list[index] {
            for multiple in (index * index..=n).step_by(index) {
                prime_list[multiple] = false;
            }
        }
    }

    prime_list
}
