mod lib;
use crate::lib::*;
use std::io::{self, stdout, BufRead, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new().user_agent("Benten").build()?;
    let mut stdin = io::stdin().lock();
    let mut search_term = String::new();
    print!("Query: ");
    stdout().flush()?;
    stdin.read_line(&mut search_term)?;
    let search_request = SearchRequest::new(&search_term, 10, 0);
    let response = search_request.get(&client).await?;

    let mut index = 0;
    for manga in &response.data {
        println!("{index}: {:?}", manga.attributes.title);
        index += 1;
    }

    print!("Select from options: ");
    stdout().flush()?;

    let mut option = String::new();
    stdin.read_line(&mut option)?;
    let option: u32 = option.trim().parse()?;

    if let Some(manga) = response.data.get(option as usize) {
        for offset in 0.. {
            // TODO: handle error after over reaching offset
            let chapter_req = ChapterSearchRequest::new(&manga.id, 1, offset);
            let res = chapter_req.get(&client).await?;
            for i in res.data {
                i.download(&client).await?;
            }
        }
    } else {
        println!("Option Doesn't exisit?");
    }
    Ok(())
}
