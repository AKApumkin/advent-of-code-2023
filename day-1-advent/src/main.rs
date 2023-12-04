/**
 * Take in a file with mixed strings
 * Extract out each lines interger pairs
 * Create a number from the pairs
 * Sum the numbers
 */
use std::io::{self, Read};
use std::fs::File;
use regex::Regex;

fn main() {
    let file_path = "src/input.txt";
    let whole_file = filename_to_string(file_path).unwrap();
    let wbyl = words_by_line(&whole_file);

    let mut count = 0;
    for line in wbyl {
        let matches = match_regex_numbers(&line);
        //println!("{:?}", matches);
        let matches_string = matches.unwrap().get(0).unwrap().as_str();
        // split string into first and last characters
        let first_char = matches_string.chars().next().unwrap();
        let last_char = matches_string.chars().next_back().unwrap();

        let combined_number = first_char.to_string() + &last_char.to_string();

        let to_int:i32 = combined_number.parse().unwrap();

        // add each int together then add to the global count
        count += to_int;
        println!("{:?}", count);
    }
    println!("The final count is: {}", count);
}


/**
 * Read a file into a string
 */
fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

/**
 * Split a string into a vector of strings by line
 * @param s: a string
 * @return a vector of strings
 */
fn words_by_line<'a>(s: &'a str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

/**
 * Match a regex pattern to a string finding the numbers
 * @param s: a string
 * @return a string of numbers
 */
fn match_regex_numbers(s: &str) -> Option<regex::Captures<'_>> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let matches = re.captures(s);
    return matches;
}