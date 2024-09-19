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

fn add_script_line(play : &mut Play, line : &String, char_part_name : &String) {
    if line.is_empty() { return }

    let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) else {
        return // leave if split_once returns None
    };

    let first_token = first_token.trim();
    let rest_of_line = rest_of_line.trim();

    // match the result of parsing and if successful, push the line into the play
    match first_token.parse::<usize>() {
        //TODO: might need to do .clone() instead of .to_string() here?
        Ok(line_num) =>
            play.push((line_num, char_part_name.to_string(), rest_of_line.to_string())),
        Err(_) => if WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!("[X] The token: \"{}\" does not represent a valid usize value.",
                      first_token);
        },
    }
}
