// do we really need to do this? this is causing issues especially with script_gen.
include!("declarations.rs");
include!("script_gen.rs");
use std::fs;

// https://docs.rs/config-file/latest/config_file/

fn main() -> Result<(), u8>  {
    // open config file
    let mut config_file_name = String::new();

    match parse_args(&mut config_file_name) {
        Ok(()) => {
            println!("Configuration file name: {}", config_file_name);
        }
        Err(..) =>  {
            println!("Error: Bad command line arguments provided.");
            // std::process::exit(BAD_CMD_LINE);
            return Err(BAD_CMD_LINE);
        }
    }
    // return Ok(()) for success
    Ok(())
}


fn parse_args(config_file_name: &mut String) -> Result<(), u8> {

    let mut args: Vec<String> = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS || (args.len() == MAX_ARGS && args[OPT_WHINGE_POS] != "whinge") {
        usage(&args[PROG_NAME_POS]);
        return Err(BAD_CMD_LINE);
    }

    *config_file_name = args[CONFIG_POS].clone();

    if args.len() == MAX_ARGS {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    Ok(())
}

fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}

fn recite(title: &String,  play: &Play) {

    println!("Title is: {}", title);

    // initialize variable for current character
    let mut current_character: Option<&CharName> = None;

    for line_tuple in play {
        match line_tuple {
            // REVIEW: what if character is an empty string? is that possible?
            (_, character,_ ) if !character.is_empty() => {
                // do nothing if it's the same character as the current one
                if Some(character) ==  current_character {
                    return;
                }
                println!();
                println!("{}.", character); // print current character with "." after
                current_character = Some(character); // update current_character
            }
            (.., line) => {
                println!("{}", line);
            }
        }
    }
}

