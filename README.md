# CSE 542 Fall 2024 Lab 1

## Group Members
- Hussein Aljorani: a.hussein@wustl.edu
- Nick Cockran: c.nick@wustl.edu
- Becky Shofner: r.a.shofner@wustl.edu

##  Overview
This program reads a configuration file that defines the title of a play and maps character names to their corresponding 

script files. It processes these script files to generate a formatted output, simulating the recitation of a scene from the play "Rosencrantz and Guildenstern are Dead".

Due to the modularity of the project, our program was designed with a step-by-step approach, where each team member was responsible for completing a specific part before passing it on to the next. We used GitHub for version control and communication to ensure smooth collaboration. Testing involved fuzzing input files by introducing extra tokens, malformed lines, incorrect tokens, and additional whitespace to ensure robust error handling.
    
## Issues/Concerns:
Program takes in a false named file and tries to read it instead of exiting with error message BAD_CMD_LINE

## Insights, observations and questions encountered:
The match construct in Rust proved to be a powerful tool, particularly for destructuring tuples within vectors. However, finding good examples to guide its use in our specific scenario was challenging. Some directions in the assignment were slightly ambiguous, which led to different interpretations among team members. We addressed this by communicating frequently to align our approaches and clarify any misunderstandings. In hindsight, testing earlier in the process would have allowed us to catch issues sooner and refine our interpretations more efficiently. For future projects, earlier testing and more structured collaboration will be key improvements.

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
    
    
