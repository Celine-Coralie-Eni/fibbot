use crate::extract_num::extract_numbers;

pub async fn get_pr_body(pr_number: u64) -> Vec<u32>{

    let files = octocrab::instance().pulls("Celine-Coralie-Eni", "fibbot").list_files(pr_number).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let nums:Vec<u32> = extract_numbers(&files.as_str().to_string());
    nums
 }
