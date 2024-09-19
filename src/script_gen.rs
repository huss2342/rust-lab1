use std::fs::File;
use std::io::{BufReader, BufRead};

type CharacterTextFile = String;
type PlayConfig = Vec((CharName, CharacterTextFile));

static TITLE_LINE : usize = 0;
static CHARACTER_CONFIG_LINE : usize = 1;
static CHARACTER_CONFIG_LINE_INDEX : usize = 0;
static CHAR_FILE_CONFIG_LINE_INDEX : usize = 1;
static CONFIG_LINE_TOKENS : usize = 2;

fn add_script_line(play : &mut Play, line : & Line, char_name : & CharName) {

    if line.len() > 0 {

        // FIXME The instructions here didn't make sense so I improvised (2nd paragraph in 8)
        if let Some(full_line) = line.split_once(' ') {
            let line_num_str = full_line.0.trim();
            let char_line = full_line.1.trim(); // TODO FIX THESE

            let line_num_res = line_num_str.parse::<usize>();

            // match the result of parsing and if successful push the line into the play
            match line_num_res {
                Err(_) => if WHINGE_MODE.load(Ordering::SeqCst) {
                    eprintln!("The token: \"{}\" does not represent a valid usize value.",
                              line_num_str);
                },
                Ok(line_num) =>
                    play.push((line_num, char_name.to_string(), char_line.to_string())),
            };

        }
    }
}