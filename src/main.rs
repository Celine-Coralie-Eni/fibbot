use number::fibonacci;

fn main() {
    print!("Hello world");
        let n = 29; 
        println!("Fibonacci numbers up to {}:", n);
        for i in 0..n {
            println!("{}", fibonacci(i));
        }
    
    // if n = f32 {
    //     println!("invalid input");
    }
    


mod number;