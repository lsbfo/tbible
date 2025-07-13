use serde::Deserialize;
use anyhow::Result;
use chrono::Datelike;

#[derive(Deserialize)]
struct ApiResponse {
    verses: Vec<Verse>,
}

#[derive(Deserialize)]
struct Verse {
    text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    if args.is_empty() {
        return daily_verse().await;
    }
    
    let input = args.join(" ");
    parse_and_fetch(&input).await
}

async fn daily_verse() -> Result<()> {
    let verses = [
        "john+3:16", "psalm+23:1", "romans+8:28", "philippians+4:13",
        "psalm+91:1", "jeremiah+29:11", "proverbs+3:5", "psalm+46:1",
        "matthew+5:3", "1john+4:19", "isaiah+40:31", "romans+12:2"
    ];
    
    let today = chrono::Utc::now().ordinal() as usize;
    let verse = verses[today % verses.len()];
    
    fetch_and_display(verse).await
}

async fn parse_and_fetch(input: &str) -> Result<()> {
    let reference = parse_reference(input);
    fetch_and_display(&reference).await
}

fn parse_reference(input: &str) -> String {
    let input = input.to_lowercase()
        .replace("ch ", "")      // psalm ch 3 -> psalm 3
        .replace("vs", ":")      // 3vs4 -> 3:4
        .replace(" ", "");       // remove spaces
    
    // Find where book name ends and numbers begin
    let book_end = input.find(char::is_numeric).unwrap_or(input.len());
    let book = &input[..book_end];
    let numbers = &input[book_end..];
    
    let book = expand_book_name(book);
    
    // Handle various formats
    let numbers = numbers
        .replace('-', ":")       // 3-16 -> 3:16
        .replace(':', ":");      // keep existing colons
    
    format!("{}+{}", book, numbers)
}

fn expand_book_name(short: &str) -> &str {
    match short {
        "gen" | "genesis" => "genesis",
        "ex" | "exo" | "exodus" => "exodus",
        "lev" | "leviticus" => "leviticus", 
        "num" | "numbers" => "numbers",
        "deut" | "dt" | "deuteronomy" => "deuteronomy",
        "josh" | "joshua" => "joshua",
        "judg" | "judges" => "judges",
        "ruth" => "ruth",
        "1sam" | "1samuel" => "1samuel",
        "2sam" | "2samuel" => "2samuel",
        "1ki" | "1kings" => "1kings",
        "2ki" | "2kings" => "2kings",
        "1chr" | "1chronicles" => "1chronicles",
        "2chr" | "2chronicles" => "2chronicles",
        "ezra" => "ezra",
        "neh" | "nehemiah" => "nehemiah",
        "est" | "esther" => "esther",
        "job" => "job",
        "ps" | "psa" | "psalm" | "psalms" => "psalms",
        "prov" | "pr" | "proverbs" => "proverbs",
        "eccl" | "ec" | "ecclesiastes" => "ecclesiastes",
        "song" | "songofsolomon" => "song+of+songs",
        "isa" | "isaiah" => "isaiah",
        "jer" | "jeremiah" => "jeremiah",
        "lam" | "lamentations" => "lamentations",
        "ezek" | "ez" | "ezekiel" => "ezekiel",
        "dan" | "daniel" => "daniel",
        "hos" | "hosea" => "hosea",
        "joel" => "joel",
        "amos" => "amos",
        "ob" | "oba" | "obadiah" => "obadiah",
        "jon" | "jonah" => "jonah",
        "mic" | "micah" => "micah",
        "nah" | "nahum" => "nahum",
        "hab" | "habakkuk" => "habakkuk",
        "zep" | "zephaniah" => "zephaniah",
        "hag" | "haggai" => "haggai",
        "zech" | "zec" | "zechariah" => "zechariah",
        "mal" | "malachi" => "malachi",
        "mt" | "mat" | "matthew" => "matthew",
        "mk" | "mar" | "mark" => "mark",
        "lk" | "luk" | "luke" => "luke",
        "jn" | "joh" | "john" => "john",
        "ac" | "act" | "acts" => "acts",
        "rom" | "romans" => "romans",
        "1cor" | "1corinthians" => "1corinthians",
        "2cor" | "2corinthians" => "2corinthians",
        "gal" | "galatians" => "galatians",
        "eph" | "ephesians" => "ephesians",
        "phil" | "php" | "philippians" => "philippians",
        "col" | "colossians" => "colossians",
        "1th" | "1thess" | "1thessalonians" => "1thessalonians",
        "2th" | "2thess" | "2thessalonians" => "2thessalonians",
        "1tim" | "1timothy" => "1timothy",
        "2tim" | "2timothy" => "2timothy",
        "tit" | "titus" => "titus",
        "phlm" | "phm" | "philemon" => "philemon",
        "heb" | "hebrews" => "hebrews",
        "jas" | "jam" | "james" => "james",
        "1pet" | "1pe" | "1peter" => "1peter",
        "2pet" | "2pe" | "2peter" => "2peter",
        "1jn" | "1john" => "1john",
        "2jn" | "2john" => "2john", 
        "3jn" | "3john" => "3john",
        "jude" => "jude",
        "rev" | "revelation" => "revelation",
        _ => short,
    }
}

async fn fetch_and_display(reference: &str) -> Result<()> {
    let url = format!("https://bible-api.com/{}", reference);
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let api_response: ApiResponse = response.json().await?;
        
        if !api_response.verses.is_empty() {
            let clean_ref = reference
                .replace('+', " ")
                .replace(":", " ")
                .to_uppercase();
            
            // Beautiful header with perfect version alignment
            println!();
            println!("    ╭─────────────────────────────────────────────────────────╮");
            println!("    │                    TERMINAL BIBLE                       │");
            println!("    ╰─────────────────────────────────────────────────────────╯");
            println!();
            
            // Reference with Bible version perfectly aligned on the right
            let version = "NKJV"; // bible-api.com uses New King James Version
            let total_width = 59;
            let ref_len = clean_ref.len() + 2; // +2 for "❝ "
            let version_len = version.len();
            let spaces_needed = total_width - ref_len - version_len;
            
            println!("    ❝ {}{}{}", clean_ref, " ".repeat(spaces_needed), version);
            println!("    {}", "━".repeat(total_width));
            println!();
            
            // Display verses with proper spacing and verse numbers
            for (i, verse) in api_response.verses.iter().enumerate() {
                // Add extra spacing between verses for longer chapters
                if i > 0 {
                    println!(); // Empty line between verses
                }
                
                // Calculate verse number (assuming verses start from 1)
                let verse_num = i + 1;
                let verse_indicator = format!("{}:", verse_num);
                
                let wrapped_text = wrap_text(verse.text.trim(), 48); // Leave room for verse number
                
                // Display verse with bold verse number
                let lines: Vec<&str> = wrapped_text.lines().collect();
                for (line_idx, line) in lines.iter().enumerate() {
                    if line_idx == 0 {
                        // First line gets the bold verse number
                        println!("      \x1b[1m{}\x1b[0m {}", verse_indicator, line);
                    } else {
                        // Subsequent lines are indented to align with text
                        let indent = " ".repeat(verse_indicator.len() + 1);
                        println!("      {}{}", indent, line);
                    }
                }
            }
            
            println!();
            println!("    {}", "━".repeat(total_width));
            println!();
        } else {
            println!();
            println!("    ✳ No verses found for: {}", reference);
            println!();
        }
    } else {
        println!();
        println!("    ✳ Could not fetch: {}", reference);
        println!();
    }
    
    Ok(())
}

fn wrap_text(text: &str, width: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        // Check if adding this word would exceed the width
        if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line = String::new();
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines.join("\n")
}

