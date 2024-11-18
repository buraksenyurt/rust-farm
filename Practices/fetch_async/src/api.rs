pub async fn fetch_data(name: &str, url: &str) -> String {
    println!("Fetching data from {}: {}", name, url);
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("API {} response received.", name);
                text
            }
            Err(e) => {
                eprintln!("Error parsing response from {}: {}", name, e);
                format!("API {} error: {}", name, e)
            }
        },
        Err(e) => {
            eprintln!("Error fetching data {}: {}", name, e);
            format!("API {} error: {}", name, e)
        }
    }
}
