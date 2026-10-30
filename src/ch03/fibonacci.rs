// N번째 피보나치 수열
fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
fn main() {
    let num = fibonacci(7);
    println!("The fibonacci number is {num}");
}