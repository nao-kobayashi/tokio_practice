use tokio::task::spawn;
use reqwest;

struct HttpGetResult {
    url: String,
    size: usize,
    has_error: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://www.rust-lang.org",
        "https://golang.org",
        "https://golang.org/doc",];

    let mut tasks = vec![];
    for url in urls {
        tasks.push(spawn(response_size(url)));
    }
    
    for task in tasks {
        let res = task.await?;
        if !res.has_error {
            println!("{} : size => {}", res.url, res.size);
        } else {
            println!("error! {}", res.url);
        }
    }

    Ok(())
}

async fn response_size(url: &str) -> HttpGetResult {
    let mut size = 0;
    let mut has_error = false;

    println!("Getting {}", url);

    if let Ok(res) = reqwest::get(url).await {
        if let Ok(body) =  res.text().await {
            size = body.len();
        } else {
            has_error = true;
        }
    } else {
        has_error = true;
    }

    println!("Complete {}", url);

    HttpGetResult{
        url: url.to_string(),
        size,
        has_error,
    }
}