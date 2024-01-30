use benten::*;
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
            let chapter_req = ChapterSearchRequest::new(&manga.id)
                .with_limit(10)
                .with_offset(offset);
            let chapters = chapter_req.get(&client).await?;
            if chapters.data.is_empty() {
                // Break when there's no more chapters to download
                break;
            }

            for chapter in chapters.data {
                chapter.download(&client).await?;
            }
        }
    } else {
        println!("Option Doesn't exisit?");
    }
    Ok(())
}
