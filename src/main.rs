use get_pr::get_pr_body;
use fibonacci::fibonacci_calc;
use std::env;
use post_comments::post_comment;

#[tokio::main]
async fn main() {
    // Read environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "100".to_string())
        .parse::<u64>()
        .unwrap_or(100);

    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    let pr_number: u128 = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u128>()
        .expect("Invalid PR_NUMBER");
    // Extract numbers from the pull request content
    let numbers = get_pr_body(pr_number).await;
    println!("{:?}", numbers);

    // Calculate Fibonacci values for the extracted numbers
    let mut fibonacci_values = String::from("the fibonacci are\n");
    for number in numbers {
        let fib = fibonacci_calc(50);
        fibonacci_values.push_str(&format!("- Fibonacci({}) = {}\n", number, fib));
    }
    println!("values: {}", fibonacci_values);

    if let Err(e) = post_comment(&fibonacci_values).await {
        eprint!("Error posting comment: {}", e);
    }
}


mod extract_num;
mod get_pr;
mod fibonacci;
mod post_comments;