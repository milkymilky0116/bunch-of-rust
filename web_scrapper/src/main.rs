use std::collections::HashMap;

use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut urls: Vec<String> = Vec::new();
    let mut url_status: HashMap<&String, StatusCode> = HashMap::new();

    urls.push(String::from("https://naver.com"));
    urls.push(String::from("https://google.com"));
    urls.push(String::from("https://github.com"));
    urls.push(String::from("http://naver.com/asdad"));

    for (_, element) in urls.iter().enumerate() {
        let resp = reqwest::get(element).await?;
        let status = resp.status();
        url_status.insert(element, status);
        println!("{}", status);
    }

    for (key, value) in &url_status {
        println!("URL: {}, Status: {}", key, value);
    }
    Ok(())
}

//https://weworkremotely.com/remote-jobs/search?term=python
