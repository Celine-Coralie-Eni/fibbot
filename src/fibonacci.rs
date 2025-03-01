pub fn fibonacci_calc(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_calc(n - 1) + fibonacci_calc(n - 2)
    }
}

