# Color guess game
First time with rust development. It's a small terminal game which covers more or less the 8 first chapters of the "[Rust programming language book](https://doc.rust-lang.org/stable/book/title-page.html)".  
 
For now there is no tests, (not yet) and the code is not perfect.

It's intended to work on Linux systems.

## How to play
Open your terminal and run `color-guess` 

A 5 colors sequence will be define randomly among 
- Red 🟥 
- Orange 🟧
- Yellow 🟨
- Gree 🟩
- Blue 🟦
- Purple 🟪
- White ⬜

You'll have to guess it. 5 rounds, each round you will be prompted. Just enter a sequence with the first letter color. If you find all the colors at the right place you win.

**Round example** :  
You want to try 🟥🟪🟨🟦🟥, type *rpybr*, valid with enter.   
No matter if you type lowercase or uppercase. Same if you enter more than 5 letters (Or less). It will just be ignore. You can only enter letters for now.   

If you enter the right color at the right place, you will see a green dot 🟢. If the color is in the sequence but not at the right place, an orange dot 🟠. 

## Evolutions (Maybe)
- Input: Add difficulty mode (Impact on rounds)
- Add more colors
- Ouput: Improve readability
