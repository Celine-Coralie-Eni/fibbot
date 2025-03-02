use std::env;
use reqwest::Client;

pub async fn post_comment(pr_content: &str) -> Result<(), reqwest::Error> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER")
        .unwrap_or_else(|_| "3".to_string())
        .parse::<u32>()
        .expect("Invalid PR_NUMBER");

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );
     println!( "url {}", url);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .json(&serde_json::json!({ "body": pr_content }))
        .send()
        .await?;

    if response.status().is_success() {
        println!(" 💯  ✅Comment posted successfully!");
    } else {
        println!(" ❌  Failed to post comment: {:?}", response.text().await?);
    }

    Ok(())
    

}
