use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for a URL
    println!("Enter the URL of the website:");
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim(); // Remove newline characters

    // Fetch the webpage
    let response = get(url)?.text()?;

    // Parse the HTML
    let document = Html::parse_document(&response);

    // Define a selector for `<p>` tags
    let selector = Selector::parse("p").unwrap();

    // Create or overwrite a text file
    let mut file = File::create("paragraphs.txt")?;

    // Iterate over all `<p>` elements, write their text content to the file
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        writeln!(file, "{}", text)?; // Write to file with a newline
    }

    println!("Paragraphs have been saved to 'paragraphs.txt'");

    Ok(())
}
