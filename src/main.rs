fn main() {
    print!("Hello world");
        let n = 100; 
        println!("Fibonacci numbers up to {}:", n);
        for i in 0..n {
            println!("{}", fibonacci(i));
        }
   
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
   
