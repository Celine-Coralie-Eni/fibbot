use get_pr::get_pr_body;
use fibonacci::fibonacci_calc;
use std::env;
use post_comments::post_comment;

#[tokio::main]
async fn main() {
    // Read environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD");
    let pr_number = env::var("PR_NUMBER").unwrap_or_else(|_| "1".to_string());
    match pr_number.parse::<u32>() {
        Ok(_num) => {
            // Use the parsed PR number
        },
        Err(e) => {
            eprintln!("Invalid PR_NUMBER: {}", e);
            std::process::exit(1);
        }
    }
    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {:?}", max_threshold);

    
        let pr_number = std::env::var("PR_NUMBER").unwrap_or_else(|_| "0".to_string());
        let pr_number: i32 = pr_number.parse().expect("Invalid PR_NUMBER");
    
    // Extract numbers from the pull request content
    let numbers = get_pr_body(pr_number.try_into().unwrap()).await;
    println!("{:?}", numbers);

    // Calculate Fibonacci values for the extracted numbers
    let mut fibonacci_values = String::from("the fibonacci are\n");
    for number in numbers {
        let fib = fibonacci_calc(number.try_into().unwrap());
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

