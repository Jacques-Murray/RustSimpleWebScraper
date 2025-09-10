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
    static ref QUOTE_SELECTOR: Selector = Selector::parse("div.quote").unwrap();
    static ref TEXT_SELECTOR: Selector = Selector::parse("span.text").unwrap();
    static ref AUTHOR_SELECTOR: Selector = Selector::parse("small.author").unwrap();
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

    let mut quotes = Vec::new();

    for element in document.select(&*QUOTE_SELECTOR) {
        let quote_text = element
            .select(&*TEXT_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No quote text found".to_string());

        let author_text = element
            .select(&*AUTHOR_SELECTOR)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No author found".to_string());

        quotes.push((quote_text, author_text));
    }

    Ok(quotes)
}
