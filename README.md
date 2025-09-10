# Simple Web Scraper in Rust
This project is a web scraper that fetches quotes from `https://quotes.toscrape.com/` and prints them to the console.

## Concepts Covered
- **Asynchronous Programming:** Using `tokio` as the async runtime.
- **Making HTTP Requests:** Using the `reqwest` crate to perform a GET request.
- **HTML Parsing:** Using the `scraper` crate to parse the HTML documents and select elements using CSS selectors.
- **Error Handling:** Using the `error-chain` crate for more structured error handling.
- **Dependency Management:** Using Cargo to manage external crates.

## How to Run
1. **Build the program:**
   ```sh
   cargo build
   ```

2. **Run the program:**
   ```sh
   cargo run
   ```

   You will see a list of quotes scraped from the website printed in your terminal.
