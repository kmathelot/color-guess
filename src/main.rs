use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::io;
use console::style;

pub mod color;

// Get a random value with a custom type
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-values-of-a-custom-type
impl Distribution<color::Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> color::Color {
        match rng.gen_range(0..7) {
            0 => color::Color::Red,
            1 => color::Color::Orange,
            2 => color::Color::Yellow,
            3 => color::Color::Green,
            4 => color::Color::Blue,
            5 => color::Color::Purple,
            6 => color::Color::White,
            7 => color::Color::Black,
            _ => color::Color::None
        }
    }
}

// Color guess game
fn main() {
    println!("Guess the {} sequence game !", style("color").bold());
    how_to_play();

    // TODO: Implement a difficulty selection which impact the rounds number
    let rounds = 5;
    let mut good_answers = 0;
    let mut color_sequence:Vec<char> = Vec::new();
    
    // Create a random Color sequence 
    for _i in 0..5 {
        let new_color: color::Color = rand::random();
        color_sequence.push(new_color.get_color());
    }

    println!("You'll have {} tries, get ready", rounds);
    // Guess start by default 5 tries
    // Rounds starts from 1, it's more human friendly.
    for guess in 1..=rounds {
        println!("Round {}", style(guess).bold());
        
        good_answers = 0; // Store the good answers
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed"); // Can be better
    
        // Let's check every characters of the input
        // By default the sequence has 5 colors. First idea was to force a 5 letters input with a while loop.
        // The main difficulty was to deal with the buffer. This way is easier
        for (i, c) in input.trim().to_uppercase().char_indices() {
            
            // Get a color char from a letter. It's possible to not having a color. 
            let next_color = color::Color::from_char(c);
            print!("{} ", next_color);

            // Check the corresponding value in the vector
            let plop = color_sequence.get(i);
            if let Some(color) = plop {
                if color == &next_color { // Good answer
                    println!("=> 🟢");
                    good_answers += 1;
                }
                else if color_sequence.contains(&next_color) { // Wrong place
                    println!("=> 🟠");
                }
                else {
                    println!(" ");
                }
            } else { break; } // If there is None at the index, we consider the end of the input. 
        }

        // We have a winner
        if good_answers == color_sequence.len() {
            println!("You Win");
            break;
        }
    }
    
    // Need to try again
    if good_answers < color_sequence.len() {
        println!("You lose");
        println!("The solution was : ");
        for color in color_sequence {
            print!("{color}\n");
        }
    }

}

// How to play
fn how_to_play() {
    print!("\n");
    print!("A sequence of 5 colors among 🟥, 🟧, 🟨, 🟩, 🟦, 🟪, ⬜ will be define.\n");
    print!("Based on a difficulty level, you'll have rounds to guess the sequence\n");
    print!("\n");
    print!("Each round you will have to enter your proposition, you can enter 5 letters :\n");
    print!("R: Red, O: Orange, Y: Yellow, G: Gree, B: Blue, P: Purple, W: White\n");
    print!("Each good color and well ordered is 🟢, good color but wrong place 🟠\n");
    print!("\n");
}