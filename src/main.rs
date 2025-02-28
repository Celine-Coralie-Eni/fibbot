use reqwest::blocking::Client;
<<<<<<< HEAD
=======
// use serde_json::{json, Value};
>>>>>>> refs/remotes/origin/master
use std::env::{self, args};
fn main() {
    println!("Hello World!");
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty() {
        println!("No arguments supplied.");
        return;
    } else if args.len() != 2 {
        println!("Fibbot requires exactly two parameters.");
        return;
    }

    let enable_fib = args[0].to_lowercase() == "true";

    let max_threshold: usize = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid max_threshold value! It must be an integer.");
            return;
        }
    };

    if enable_fib {
        println!("Fibbot enabled successfully with max_threshold: {}", max_threshold);
        let fib_result = fibonacci(max_threshold.try_into().unwrap());
        println!("Fibonacci result: {}", fib_result);
    } else {
        println!("Fibonacci calculation is disabled.");
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

fn extract_numbers(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            numbers.push(num);
        }
    }
    numbers
}

pub fn fibonacci(n: u128) -> u128 {
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
    assert!(std::panic::catch_unwind(|| fibonacci(u128::MAX)).is_err());
}

#[test]
fn test_fibonacci_efficiency() {
    // Test the function's efficiency for large inputs
    assert_eq!(fibonacci(93), 12200160415121876738);
    assert_eq!(fibonacci(94), 19740274219868223167);
}

pub fn get_pr_body(pr_number: u128) -> Result<String, Box<dyn std::error::Error>> {
    let repo = env::var("GITHUB_REPOSITORY")?;
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!(
        "https://api.github.com/repos/{}/pulls/{}/files",
        repo, pr_number
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .bearer_auth(token)
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        if let Some(body) = json.get("body") {
            return Ok(body.as_str().unwrap_or("").to_string());
        }
    }

    Err("Failed to get pull_request body".into())
}
