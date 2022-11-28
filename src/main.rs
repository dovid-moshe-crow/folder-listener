use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use reqwest::Client;
use urlencoding::encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("Argument 1 needs to be a path");
    let url = &args.get(2).expect("Argument 2 needs to be a url");

    let mut params = String::from("");

    if args.len() > 3 {
        for i in 3..args.len(){
            let str = args.get(i).unwrap().split("=").collect::<Vec<&str>>();
            let first = encode(str.get(0).expect("invalid query param"));
            let last = encode(str.get(1).expect("invalid query param"));

            params.push_str(&format!("&{}={}",first,last));
        }
    }


    println!("watching {}", path);
    println!("Sending events to {}", url);
    if let Err(e) = watch(&path, &url, &params).await {
        println!("error: {:?}", e)
    }

    Ok(())
}

async fn watch(path: &str, url: &str, params: &str) -> notify::Result<()> {
    let client = Client::new();

    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("could not create file listener");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(path.as_ref(), RecursiveMode::NonRecursive)
        .expect("could not create file listener");

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                EventKind::Create(_) => {
                    let file_name = event
                        .paths
                        .first()
                        .expect("could not get file name")
                        .file_name()
                        .expect("could not get file name");
                    println!("created: {:?}", file_name);

                    let full_url = format!(
                        "{}/?filename={}{}",
                        url,
                        encode(file_name.to_str().expect("could not encode filename")),
                        params
                    );

                    println!("{}", full_url);

                    let res = client.get(full_url).send().await;

                    match res {
                        Ok(response) => match response.text().await {
                            Ok(text) => println!("webhook: {}", text),
                            Err(err) => println!("http error: {:?}", err),
                        },
                        Err(e) => println!("http error: {:?}", e),
                    }
                }

                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
