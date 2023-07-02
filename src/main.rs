use select::{document::Document, predicate::Name};
use std::{env, io};
use std::{env::VarError, error::Error, fs::File, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = env::var("FEEDS").ok();

    match file_path {
        Some(_) => (),
        None => {
            panic!(
                "Please set up the $FEEDS environment variable {}",
                VarError::NotPresent
            )
        }
    };

    if args.len() > 2 {
        panic!("Please provide only one url.")
    }
    let link = &args[1];
    let results = scrape_url(link).await?;
    print!("{} link(s) found!\n", &results.len());
    for (i, result) in results.iter().enumerate() {
        println!("{}) {}", i, result);
    }
    print!("Which of these if any would you like?\n",);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{input}");
        }
        Err(error) => println!("error: {error}"),
    }
    let index = input.trim().parse::<i32>();

    // let index = input.parse::<i32>();
    match index {
        Ok(index) => {
            let mut file = File::options().append(true).open(file_path.unwrap())?;
            let url = results[index as usize].to_string();
            writeln!(&mut file, "{url}").ok();
        }
        Err(err) => println!("Error encountered {:?}", err),
    };

    Ok(())
}

fn is_feed(url: &str) -> bool {
    url.ends_with("feed")
        | url.contains("feed")
        | url.ends_with("xml")
        | url.contains("xml")
        | url.ends_with("rss")
        | url.contains("rss")
}

async fn scrape_url(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok({
        let res = reqwest::get(url).await?.text().await?;
        let mut results: Vec<String> = Vec::new();

        Document::from(res.as_str())
            .find(Name("link"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| match is_feed(x) {
                true => results.push((&x).to_string()),
                false => (),
            });
        Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| match is_feed(x) {
                true => results.push((&x).to_string()),
                false => (),
            });
        results
    })
}
