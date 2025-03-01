use crate::extract_num::extract_numbers;

pub async fn get_pr_body(pr_number: u64) -> Vec<i32>{

    let files = octocrab::instance().pulls("Awungia112", "fibbot").list_files(pr_number).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let nums = extract_numbers(&files.as_str().to_string());
    nums
 }

 