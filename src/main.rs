 use std::env;
// use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
// use serde_json::{json, Value};
   fn main() {
    println!("Hello World!");
       // Parse the input parameters
       let enable_fib = env::var("INPUT_ENABLE_FIB")
           .unwrap_or_else(|_| "false".to_string())
           .parse::<bool>()
           .unwrap_or(false);

       let max_threshold = env::var("INPUT_MAX_THRESHOLD")
           .unwrap_or_else(|_| "100".to_string())
           .parse::<i32>()
           .unwrap_or(100);

       // Demonstrate the usage of the parameters
       if enable_fib {
           println!("Calculating Fibonacci sequence up to {}", max_threshold);
           // Implement Fibonacci sequence calculation here
       } else {
           println!("Fibonacci sequence calculation is impossible. Max threshold is {}", max_threshold);
       }
   }


#[test]
fn test_input_parsing() {
    // Set the environment variables for the input parameters
    std::env::set_var("INPUT_ENABLE_FIB", "true");
    std::env::set_var("INPUT_MAX_THRESHOLD", "200");

    // Call the main function to parse the inputs
    main();

    // Verify the parsed values
    assert_eq!(true, env::var("INPUT_ENABLE_FIB").unwrap().parse::<bool>().unwrap());
    assert_eq!(200, env::var("INPUT_MAX_THRESHOLD").unwrap().parse::<i32>().unwrap());


  let sample_pr_content = "The PR contains 5 changes and 3 bug fixes. The max threshold is 100.";
    let extracted_numbers = extract_numbers(sample_pr_content);

    println!("Extracted numbers: {:?}", extracted_numbers);

    for num in extracted_numbers {
        println!("Fibonacci of {} is {}", num, fibonacci(num as u64));
    }

fn extract_numbers(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            numbers.push(num);
        }
    }
    numbers
}


 pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    b
}


#[test]
fn test_fibonacci_edge_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(10), 55);

    // Test for negative input
    assert!(std::panic::catch_unwind(|| fibonacci(-1)).is_err());
}

#[test]
fn test_fibonacci_efficiency() {
    // Test the function's efficiency for large inputs
    assert_eq!(fibonacci(93), 12200160415121876738);
    assert_eq!(fibonacci(94), 19740274219868223167);
}

async fn post_pr_comment(
    repo_owner: &str,
    repo_name: &str,
    pr_number: i32,
    comment_text: &str,
    github_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        repo_owner, repo_name, pr_number
    );

    let payload = json!({
        "body": comment_text
    });

    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", github_token))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment: {:?}", response.status());
    }

    Ok(())
}





 






   








}
