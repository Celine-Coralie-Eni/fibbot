fn extract_numbers(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            numbers.push(num);
        }
    }
    numbers
}
