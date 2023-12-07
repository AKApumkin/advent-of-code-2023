use std::i32;
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
    let file_path: &str    = "src/input.txt";
    let whole_file: String = filename_to_string(file_path).unwrap();
    let wbyl: Vec<String>  = words_by_line(&whole_file);

    let mut count: i32 = 0;
    for line in wbyl {
        let mut replaced_string: String  = line.to_string();
        let numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for (index,number) in numbers.iter().enumerate()
        {
            let current_number = index + 1;
            let current_number_string:String = current_number.to_string();
            replaced_string = replaced_string.replace(number, current_number_string.as_str());
        }
        let re = Regex::new(r"[0-9]").unwrap();
        let the_matches: Vec<_> = re.find_iter(replaced_string.as_str()).map(|mat| mat.as_str()).collect();
  
        let first_number: &str = the_matches.clone().into_iter().next().unwrap();
        let last_number: &str  = the_matches.clone().into_iter().next_back().unwrap();

        let combined: String  = first_number.to_string() + &last_number.to_string();

        let final_number:i32 = combined.parse::<i32>().unwrap();
        count += final_number;
        print!("{} ", count);
    }
    println!("The final count is: {}", count);
}

/**
 * Read a file into a string
 */
fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file: File = File::open(s)?;
    let mut s: String = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

/**
 * Split a string into a vector of strings by line
 * @param s: a string
 * @return a vector of strings
 */
fn words_by_line<'a>(s: &'a str) -> Vec<String> {
    s.split_whitespace().map(|s: &str| s.to_string()).collect()
}
