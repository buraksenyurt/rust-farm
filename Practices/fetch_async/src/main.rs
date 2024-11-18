/*
   Bu örnekte amaç basit bir asenkron programlama örneği geliştirmek.
   Aşağıdaki servislerden asenkron olarak veri çeken işleri icra etmeye çalışacağız.

   http://numbersapi.com/random/trivia
   https://catfact.ninja/fact
   https://randomuser.me/api/

   HTTP istekleri için reqwest, asenkron çalışma planlayıcısı olarak tokio kullanılıyor.
*/
mod api;
mod file;

#[tokio::main]
async fn main() {
    let apis = [
        ("trivia", "txt", "http://numbersapi.com/random/trivia"),
        ("catFacts", "json", "https://catfact.ninja/fact"),
        ("randomUser", "json", "https://randomuser.me/api/"),
    ];

    let mut tasks = vec![];

    for (name, format, api) in apis.iter() {
        let api = api.to_string();
        let name = name.to_string();
        let format = format.to_string();

        tasks.push(tokio::spawn(async move {
            let result = api::fetch_data(&name, &api).await;
            let file_name = format!("{}.{}", name, format);
            file::save_to_file(&file_name, &result)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Error writing to {}: {}", file_name, e);
                });
        }));
    }

    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Task error: {}", e);
        }
    }

    println!("All data has been saved to files...");
}
