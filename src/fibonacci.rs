pub fn fibonacci_calc(n: u32) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_calc(n - 1) + fibonacci_calc(n - 2)
    }
}

