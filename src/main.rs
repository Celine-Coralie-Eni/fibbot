use get_pr::get_pr_body;
use fibonacci::fibonacci_calc;
use std::env;
use post_comments::post_comment;

#[tokio::main]
async fn main() {
    // Read environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
    let pr_number = env::var("PR_NUMBER").unwrap_or_else(|_| "2".to_string());

    // Parse the PR number
    let pr_number: u32 = match pr_number.parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Invalid PR_NUMBER: {}", e);
            std::process::exit(1);
        }
    };

    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Extract numbers from the pull request content
    let numbers:Vec<u32> = get_pr_body(pr_number.into()).await;
    println!("{:?}", numbers);

    // Calculate Fibonacci values for the extracted numbers
    let mut fibonacci_values = String::from("the fibonacci are\n");
    for number in numbers {
        let fib = fibonacci_calc(number);
        fibonacci_values.push_str(&format!("- Fibonacci({}) = {}\n", number, fib));
    }
    println!("values: {}", fibonacci_values);

    if let Err(e) = post_comment(&fibonacci_values).await {
        eprintln!("Error posting comment: {}", e);
    }
}

mod extract_num;
mod get_pr;
mod fibonacci;
mod post_comments;
