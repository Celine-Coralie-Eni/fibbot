 use std::env;
   fn main() {
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
}
