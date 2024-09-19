use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::declarations::{Play, CharName, WHINGE_MODE, Ordering};

type CharacterTextFile = String;
type PlayConfig = Vec<(CharName, CharacterTextFile)>;

static TITLE_LINE : usize = 0;
static CHARACTER_CONFIG_LINE : usize = 1;
static CHARACTER_NAME_CONFIG_LINE_INDEX : usize = 0;
static CHARACTER_FILE_CONFIG_LINE_INDEX : usize = 1;
static CONFIG_LINE_TOKENS : usize = 2;

fn add_script_line(play: &mut Play, line: &String, char_part_name: &String) {
    if line.is_empty() { return }

    let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) else {
        return // leave if split_once returns None
    };

    let first_token = first_token.trim();
    let rest_of_line = rest_of_line.trim();

    // match the result of parsing and if successful, push the line into the play
    match first_token.parse::<usize>() {
        // REVIEW: might need to do .clone() instead of .to_string() here?
        Ok(line_num) =>
            play.push((line_num, char_part_name.to_string(), rest_of_line.to_string())),
        Err(_) => if WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!("[X] ERROR: The token \"{}\" does not represent a valid usize value.",
                      first_token);
        },
    }
}

fn grab_trimmmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    /*
        found this from here because I was having a syntax issue
        https://users.rust-lang.org/t/rust-file-open-error-handling/50681
    */
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[X] ERROR: Failed to open file '{}': {}", file_name, e);
            return Err(2)
        }
    };

    // https://doc.rust-lang.org/std/io/struct.BufReader.html
    let mut reader= BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => return Ok(()), // indicates success
            // REVIEW: is it possible that we push an empty line here?
            Ok(_) => file_lines.push(line.trim().to_string()),
            Err(e) => {
                eprintln!("[X] ERROR: Failed to read line '{}': {}", file_name, e);
                return Err(3)
            }
        }

    }
}