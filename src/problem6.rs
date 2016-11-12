fn main() {
    let max: u64 = 100;
    let sum_squares = (1..max+1).map(|x| x.pow(2)).fold(0, |acc, x| acc + x);
    let square_sum = (1..max+1).fold(0, |acc, x| acc + x).pow(2);

    println!("{}", square_sum - sum_squares);
}
