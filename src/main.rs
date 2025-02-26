fn main() {
    print!("Hello world");
    
    match Cli::from_args() {
        Ok(cli) => {
            if cli.enable_fib {
                println!("Generating Fibonacci sequence up to {}", cli.max_threshold);
                // Implement your Fibonacci sequence generation logic here
                let mut a = 0;
                let mut b = 1;
                while a <= cli.max_threshold {
                    println!("{}", a);
                    let next = a + b;
                    a = b;
                    b = next;
                }
            } else {
                println!("Fibonacci sequence generation is disabled");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
    
    
