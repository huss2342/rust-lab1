# CSE 542 Fall 2024 Lab 1

## Group Members
- Hussein Aljorani: a.hussein@wustl.edu
- Nick Cockran: c.nick@wustl.edu
- Becky Shofner: r.a.shofner@wustl.edu

##  Overview
This program reads a configuration file that defines the title of a play and maps character names to their 
corresponding script files. It processes these script files to generate a formatted output, simulating the
recitation of a scene from a play. Our configuration file is for a portion of "Rosencrantz and Guildenstern are Dead".

Due to the modularity of the project, team members were responsible for completing specific parts of the assignment.
Each person did a step from the instructions and then passed it on to the next person. We used GitHub for version 
control and communication to ensure smooth collaboration. Testing involved fuzzing input files by introducing extra 
tokens, malformed lines, incorrect tokens, and additional whitespace to ensure proper error handling.

## Insights, observations and questions encountered:
The match construct in Rust proved to be a powerful tool, particularly for destructuring tuples within vectors. 
However, finding good examples to guide its use in our specific scenario was challenging. 
Some directions in the assignment were slightly ambiguous, which led to different interpretations 
among team members. We addressed this by communicating frequently to align our approaches. Additionally,
using the debugger to help see what the variables were passing/storing helped a lot.

## Tests Run
1. Run config file from terminal using incorrect file name
2. Mis-spelling of file name and/or "whinge"
3. Additional or less arguments in command line
4. Misspelling of tokens in config file
5. Removal and addition of number of tokens in config file 
6. Addition and removal of whitespace in both config and text files
7. Removal of number token or script line in text file

## Usage
1. Unzip the project folder.
2. Write a config file with its character text files in the root of the project directory, or use the one provided.
3. Run the main script using the following command:
   ```
   cargo run <config_file_name> [whinge]
   ```
   Where:
- `<config_file_name>` is the name of your configuration file (required)
- `[whinge]` is an optional parameter to enable additional error output

    Example:
    ```
    cargo run hamlet_ii_2_config.txt whinge
    ```
  