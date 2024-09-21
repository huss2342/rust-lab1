use std::fs::File;
use std::io::{BufReader, BufRead};

// conditionally include declarations.rs if not already including it
// --> using WHINGE_MODE which is defined in that file
// #[cfg(not(WHINGE_MODE))]
// include!("declarations.rs"); // not sure if this is working right but it's making autocomplete work so I'm leaving it for now

// auto complete was being dumb and not working in general but the lines above didn't work for this file :(

type CharacterTextFile = String;
type PlayConfig = Vec<(CharName, CharacterTextFile)>;

// line numbers in character config files
static TITLE_LINE : usize = 0;
static CHARACTER_CONFIG_LINE : usize = 1;

// token indices and number of tokens
static CHARACTER_NAME_CONFIG_LINE_INDEX : usize = 0;
static CHARACTER_FILE_CONFIG_LINE_INDEX : usize = 1;
static CONFIG_LINE_TOKENS : usize = 2;


/// TODO Add function documentation, do this for everything in the future :)
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

/// TODO Add function documentation, do this for everything in the future :)
fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    /*
        found this from here because I was having a syntax issue
        https://users.rust-lang.org/t/rust-file-open-error-handling/50681
    */
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[X] ERROR: Failed to open file '{}': {}", file_name, e);
            return Err(2) // FIXME with a constant later
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
                return Err(3) // FIXME with a constant later
            }
        }

    }
}

/// TODO Add function documentation, do this for everything in the future :)
fn process_config(play: &mut Play, play_config: &PlayConfig) -> Result<(), u8>  {

    let mut file_lines_ref: Vec<String> = Vec::new();

    for config in play_config {
        // match tuple in play_config to destructure
        match config {
            (char_name, character_text_file) => {
                // try to get trimmed lines from file
                if let Err(_) = grab_trimmed_file_lines(character_text_file, &mut file_lines_ref) {
                    return Err(2) // FIXME with a constant later
                }
                for line in file_lines_ref.iter() {
                        add_script_line(play, line, char_name);
                }
            }
        }
    }
    Ok(()) // Can one of you explain why we put the unit tuple inside Ok()? - Nick
}

/// takes in a line from a config file,
/// then pushes the tokens to the PlayConfig variable that is passed in by reference
fn add_config(config_line: &String, play_config: &mut PlayConfig) {
    let config_line_tokens: Vec<&str> = config_line.split_whitespace().collect();

    if config_line_tokens.len() != CONFIG_LINE_TOKENS {
        if WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!("Provided config line has the wrong number of tokens.");
        }
    }

    if config_line_tokens.len() >= CONFIG_LINE_TOKENS {
        play_config.push((config_line_tokens[CHARACTER_NAME_CONFIG_LINE_INDEX].to_string(),
                            config_line_tokens[CHARACTER_FILE_CONFIG_LINE_INDEX].to_string())); // Hussein, this is why I was trying to keep the static variable names small...
    }
}

/// goes through a config file and if it doesn't return an error,
/// sets the play_title variable that is passed by reference,
/// then adds all lines to the play_config variable that is passed by reference.
///
fn read_config(config_file_name: &String, mut play_title: &mut String,
               play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines: Vec<String> = Vec::new();

    match grab_trimmed_file_lines(config_file_name, &mut lines) {
        Err(_) => return Err(FAILED_TO_GENERATE_SCRIPT),
        Ok(()) => if lines.len() <= CHARACTER_CONFIG_LINE {
            // return error if not enough lines to generate the script
            return Err(FAILED_TO_GENERATE_SCRIPT)
        } else {
            play_title = lines[TITLE_LINE]; // FIXME no clue why this isn't working
            for line in &lines { // TODO this needs to start at 1 and I can't remember exactly how to do that
                add_config(line, play_config);
            }
        }
    }


    return Ok(());
}