use clap::Parser;

mod algorithms;

#[derive(Parser)]
struct Args {
    #[arg(short = 'n', long = "limit")]
    limit: u64,
}

fn main() {
    let args = Args::parse();

    let primes: u64 = algorithms::eratosthenes::eratosthenes(args.limit)
        .iter()
        .map(|&val| val as u64)
        .sum();
    println!("Prime Count: {}", primes);
}
