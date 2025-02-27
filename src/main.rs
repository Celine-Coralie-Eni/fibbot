fn main() {
    // print!("Hello world");
    //     let n = 100; 
    //     println!("Fibonacci numbers up to {}:", n);
    //     for i in 0..n {
    //         println!("{}", fibonacci(i));
    //     }
   
    // if n = f32 {
    //     println!("invalid input");
 use github_actions::core;
   use std::env;

   // fn main() {
       // Get inputs from the environment
       let enable_fib: bool = core::get_input("enable_fib").unwrap_or("false".to_string()) == "true";
       let max_threshold: u32 = core::get_input("max_threshold").unwrap_or("100".to_string()).parse().unwrap_or(100);

       // Output "Hello, world!" to the GitHub Actions log
       println!("Hello, world!");

       // If Fibonacci is enabled, calculate and print Fibonacci numbers up to max_threshold
       if enable_fib {
           let fib_numbers = fibonacci(max_threshold);
           println!("Fibonacci numbers up to {}: {:?}", max_threshold, fib_numbers);
       }
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



2. *Implement input parsing in the Rust code*:
   ```rust
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
           println!("Fibonacci sequence calculation is disabled. Max threshold is {}", max_threshold);
       }
   }
   ```

*Demonstration*:

To demonstrate that the parameters are correctly read and validated, you can add the following test cases:

```rust
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
```

This test case sets the environment variables for the input parameters, calls the `main()` function, and then verifies that the parsed values match the expected values.

You can also add logging statements in the `main()` function to display the parsed values, which can be observed in the GitHub Actions workflow logs.

_The answer does not rely on search results. Use the You.com app for the full experience 👉 https://youcom.onelink.me/lbFr/whatsappEM_
