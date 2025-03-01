use std::env;
use extract_num::extract_numbers;
use crate::fibonacci::fibonacci;

fn main() {
    // Get the inputs from environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string()) == "true";
    let max_threshold: i32 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100); 

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

    let pr_content = "This PR fixes issue 42 and adds 3 new features.";
    let numbers = extract_numbers(pr_content);
    println!("{:?}", numbers); // Output: [42, 3]

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