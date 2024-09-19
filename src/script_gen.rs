use std::fs::File;
use std::io::{BufReader, BufRead};

type CharacterTextFile = String;
type PlayConfig = Vec((CharName, CharacterTextFile));

static TITLE_LINE : usize = 0;
static CHARACTER_CONFIG_LINE : usize = 1;
static CHARACTER_CONFIG_LINE_INDEX : usize = 0;
static CHAR_FILE_CONFIG_LINE_INDEX : usize = 1;
static CONFIG_LINE_TOKENS : usize = 2;