use error_chain::error_chain;
use reqwest;
use scraper::{Html, Selector};
use tokio;

// Error chain for cleaner error handling
error_chain! {
  foreign_links {
    ReqError(reqwest::Error);
    IoError(std::io::Error);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Make an asynchronous GET request to the target URL
    let res = reqwest::get("https://quotes.toscrape.com/").await?;

    // Ensure the request was successful
    if !res.status().is_success() {
        eprintln!("Failed to fetch the page. Status: {}", res.status());
        return Ok(());
    }

    // Read the response body as text
    let body = res.text().await?;

    // Parse the HTML document
    let document = Html::parse_document(&body);

    // Define selectors for the elements we want to extract
    let quote_selector = Selector::parse("div.quote").unwrap();
    let text_selector = Selector::parse("span.text").unwrap();
    let author_selector = Selector::parse("small.author").unwrap();

    println!("\n--- Scraped Quotes ---\n");

    // Iterate over each quote element found in the document
    for element in document.select(&quote_selector) {
        // Extract the text of the quote
        let quote_text = element
            .select(&text_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No quote text found".to_string());

        // Extract the author of the quote
        let author_text = element
            .select(&author_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "No author found".to_string());

        println!("\"{}\", - {}", quote_text.trim(), author_text.trim());
    }

    println!("\n-------------------------\n");

    Ok(())
}
