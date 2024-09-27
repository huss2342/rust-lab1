# CSE 542 Fall 2024 Lab 1
---

### Hussein Aljorani: a.hussein@wustl.edu
### Nick Cockran: c.nick@wustl.edu
### Becky Shofner: r.a.shofner@wustl.edu

---

    Overview:
    This program reads a configuration file that defines the title of a play and maps character names to their corresponding 
    script files. It processes these script files to generate a formatted output, simulating the recitation of a scene from the 
    play "Rosencrantz and Guildenstern are Dead".

    Due to the modularity of the project, our program was designed with a step-by-step approach, where each team member was 
    responsible for completing a specific part before passing it on to the next. We used GitHub for version control and 
    communication to ensure smooth collaboration. Testing involved fuzzing input files by introducing extra tokens, 
    malformed lines, incorrect tokens, and additional whitespace to ensure robust error handling.
    
    Issues/Concerns: 
    Program takes in a false named file and tries to read it instead of exiting with 
    error message BAD_CMD_LINE

    Insights, observations and questions you encountered while completing the assignment:
    The match construct in Rust proved to be a powerful tool, particularly for destructuring tuples within vectors. 
    However, finding good examples to guide its use in our specific scenario was challenging. Some directions in the 
    assignment were slightly ambiguous, which led to different interpretations among team members. We addressed this 
    by communicating frequently to align our approaches and clarify any misunderstandings. In hindsight, testing earlier 
    in the process would have allowed us to catch issues sooner and refine our interpretations more efficiently. 
    For future projects, earlier testing and more structured collaboration will be key improvements.
    
---





# FIX at the end:

Becky said: Completed tests on reading file with whinge, extra arguments, no argument, and misspelled arguments
- ### how are we commenting out lines? // or *//
- ### incorrect file name still goes through to read
- ### we are explicitly told to add use statements but it doesn't look like they are being utilized (step 7)
- ### We are explicitly told to add use std::env in main.rs, not declarations.rs (step 1)
- ### extra whitespace?
- ### go through *REVIEW: , TODO, FIXME* comments
- ### remove unnecessary comments


---

The second section of your ReadMe.txt file should provide detailed instructions for how to:

    1- unzip or otherwise unpack your files,
    2- build your program(s), and
    3- run your program(s) on the CEC Linux Lab machines where your lab solutions will be evaluated.

---

 
    
1) what the submitted program does, 
2) any places where your solution diverged from what was requested in the assignment, and 
3) the design decisions you made in developing your code. Please also ask any remaining questions you may have, in your readme file, and feedback on your impressions of the lab (what was easy, what was difficult, suggestions, etc.) is welcome.
