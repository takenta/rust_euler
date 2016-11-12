fn main() {
    println!("{}", solve(20));
}

fn solve(limit: u64) -> u64 {
    (2..limit+1).fold(1, |acc, x| acc * x / gcd(acc, x))
}

fn gcd(a: u64, b: u64) -> u64 {
    let (a, b) = if a > b { 
        (a, b) 
    } else { 
        (b, a) 
    };

    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

