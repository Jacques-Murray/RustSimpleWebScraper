use anyhow::Result;
use lazy_static::lazy_static;
use reqwest;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    const TARGET_URL: &str = "https://quotes.toscrape.com/";
    let quotes = scrape_quotes(TARGET_URL).await?;

    println!("\n--- Scraped Quotes ---\n");
    for (quote, author) in quotes {
        println!("\"{}\", - {}", quote.trim(), author.trim());
    }
    println!("\n-------------------------\n");

    Ok(())
}

// Parse selectors once at the top-level, outside of async context
lazy_static! {
    static ref QUOTE_SELECTOR: Selector =
        Selector::parse("div.quote").expect("Failed to parse quote selector");
    static ref TEXT_SELECTOR: Selector =
        Selector::parse("span.text").expect("Failed to parse text selector");
    static ref AUTHOR_SELECTOR: Selector =
        Selector::parse("small.author").expect("Failed to parse author selector");
}

async fn scrape_quotes(url: &str) -> Result<Vec<(String, String)>> {
    let res = reqwest::get(url).await?;

    if !res.status().is_success() {
        anyhow::bail!(
            "Failed to fetch the page at '{}'. Status: {}",
            url,
            res.status()
        );
    }

    let body = res.text().await?;
    let document = Html::parse_document(&body);

    let quotes: Vec<(String, String)> = document
        .select(&*QUOTE_SELECTOR)
        .filter_map(|element| {
            let quote_text = element
                .select(&*TEXT_SELECTOR)
                .next()?
                .text()
                .collect::<String>();
            let author_text = element
                .select(&*AUTHOR_SELECTOR)
                .next()?
                .text()
                .collect::<String>();
            Some((quote_text, author_text))
        })
        .collect();

    Ok(quotes)
}
