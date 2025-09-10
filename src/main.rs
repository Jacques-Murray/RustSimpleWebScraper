use anyhow::Result;
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

    let quote_selector = Selector::parse("div.quote")?;
    let text_selector = Selector::parse("span.text")?;
    let author_selector = Selector::parse("small.author")?;

    let mut quotes = Vec::new();

    for element in document.select(&quote_selector) {
        let quote_text = element
            .select(&text_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No quote text found".to_string());

        let author_text = element
            .select(&author_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No author found".to_string());

        quotes.push((quote_text, author_text));
    }

    Ok(quotes)
}
