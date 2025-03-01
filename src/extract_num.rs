pub fn extract_numbers(content: &str) -> Vec<u32> {
    content
        .split_whitespace() // Split the content into words
        .filter_map(|word| word.parse::<u32>().ok()) // Try to parse each word as an i32
        .collect() // Collect the results into a Vec<i32>
}

 
