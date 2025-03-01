use std::env;
use reqwest::Client;
use crate::{extract_num::extract_numbers, fibonacci, get_pr::get_pr_body};

pub async fn process_pr(pr_number: u128) -> Result<(), Box<dyn std::error::Error>> {
    let pr_body = get_pr_body(pr_number).await?;
    let numbers = extract_numbers(&pr_body);

    let mut results = String::new();

    for num in numbers {
        let fib = fibonacci(num as u128);
        results.push_str(&format!("Fibonacci of {} is {}\n", num, fib));
    }

    // Post a comment with the results
    let repo = env::var("GITHUB_REPOSITORY")?;
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!("https://api.github.com/repos/{}/issues/{}/comments", repo, pr_number);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .bearer_auth(token)
        .json(&serde_json::json!({ "body": results }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment: {}", response.status());
    }

    Ok(())
}