use reqwest::Client;
use serde_json::Value;
use std::env;


fn main() {
    // Get the inputs from environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string()) == "true";
    let max_threshold: i32 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100); // Default to 100 if parsing fails

    // Log the values
    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Validate the parameters
    if max_threshold <= 0 {
        eprintln!("Error: Max threshold must be greater than 0.");
        std::process::exit(1);
    }

    // Use the inputs in your logic
    if enable_fib {
        println!("Fibonacci calculation is enabled.");
        // Implement Fibonacci logic here
    }
}

    






// Process PR content, compute Fibonacci numbers, and post a comment
pub async fn process_pr(pr_number: u128) -> Result<(), Box<dyn std::error::Error>> {
    let pr_body = get_pr_body(pr_number).await?;
    let numbers = extract_numbers(&pr_body);

    let mut results = String::new();

    for num in numbers {
        let fib = fibonacci(num as u128);
        results.push_str(&format!("Fibonacci of {} is {}\n", num, fib));
    }

    // Post a comment with the results
    let repo = env::var("GITHUB_REPOSITORY")?;
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!("https://api.github.com/repos/{}/issues/{}/comments", repo, pr_number);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .bearer_auth(token)
        .json(&serde_json::json!({ "body": results }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment: {}", response.status());
    }

    Ok(())
}

async fn process_pull_request_body(body: &str, pr_number: u128) -> Result<(), Box<dyn std::error::Error>> {
    let numbers = extract_numbers(body);
    for num in numbers {
        process_pr(pr_number).await?;
    }
    Ok(())
}

#[tokio::main]
async fn num() {
    let pr_body = "This is a sample pull request body with numbers: 10, 20, 30.";
    let pr_number = 123;
    process_pull_request_body(pr_body, pr_number).await.unwrap();
}

#[test]
fn test_input_parsing() {
    // Set the environment variables for the input parameters
    std::env::set_var("INPUT_ENABLE_FIB", "true");
    std::env::set_var("INPUT_MAX_THRESHOLD", "100");

    // Call the main function to parse the inputs
    main();

    // Verify the parsed values
    assert_eq!(
        true,
        env::var("INPUT_ENABLE_FIB")
            .unwrap()
            .parse::<bool>()
            .unwrap()
    );
    assert_eq!(
        200,
        env::var("INPUT_MAX_THRESHOLD")
            .unwrap()
            .parse::<i32>()
            .unwrap()
    );

    let sample_pr_content = "The PR contains 5 changes and 3 bug fixes. The max threshold is 100.";
    let extracted_numbers = extract_numbers(sample_pr_content);

    println!("Extracted numbers: {:?}", extracted_numbers);

    for num in extracted_numbers {
        println!("Fibonacci of {} is {}", num, fibonacci(num as u128));
    }
}



#[test]
fn test_fibonacci_edge_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(10), 55);

    // Test for negative input
    assert!(std::panic::catch_unwind(|| fibonacci(u128::MAX)).is_err());
}

#[test]
fn test_fibonacci_efficiency() {
    // Test the function's efficiency for large inputs
    assert_eq!(fibonacci(93), 12200160415121876738);
    assert_eq!(fibonacci(94), 19740274219868223167);
}

mod extract_num;
mod get_pr;
mod fibonacci;