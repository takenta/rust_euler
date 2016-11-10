fn main() {
    println!("{:?}", fibonacci(32).iter().filter(|&n| n < &4000000 && n % 2 == 0).collect::<Vec<&u64>>());
    println!("{}", fibonacci(32).iter().filter(|&n| n < &4000000 && n % &2 == 0).fold(0, |acc, &n| acc + n));
}

fn fibonacci(limit: usize) -> Vec<u64> {
    let mut acc: Vec<u64> = vec![1, 2];
    _fibonacci(&mut acc, 0, limit);

    acc
}


fn _fibonacci(acc: &mut Vec<u64>, index: usize, limit: usize) {
    if acc.len() < limit {
        let (a, b) = (acc[index], acc[index+1]);
        acc.push(a + b);
        _fibonacci(acc, index+1, limit);
    }
}
