fn main() {
    println!("{}", solve(13195));
    println!("{}", solve(600851475143));
}

fn solve(n: u64) -> u64 {
    let n_sqrt = (n as f64).sqrt().ceil() as usize;
    let mut ans: u64 = 0;

    for i in 2..(n_sqrt + 1) {
        let x = i as u64;

        if n % x == 0 { 
            ans = n / x;
            break;
        }
    }

    if ans == 0 {
        n
    } else {
        solve(ans)
    }
}

