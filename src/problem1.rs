fn main() {
    println!("{}", run(10));
    println!("{}", run(1000));
}

fn run(limit: u32) -> u32 {
    let mut acc = 0;

    for n in 1..limit {
        if n % 3 == 0 || n % 5 == 0 {
            acc = acc + n;
        }
    }

    acc
}
