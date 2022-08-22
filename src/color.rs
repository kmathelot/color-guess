// TODO: Delete the NONE const and implement an option match type
const RED: char = '🟥';
const ORANGE: char = '🟧';
const YELLOW: char = '🟨';
const GREEN: char = '🟩';
const BLUE: char = '🟦';
const PURPLE: char = '🟪';
const WHITE: char = '⬜';
const BLACK: char = '⬛';
const NONE: char = ' ';

// Maybe consider a struct to avoid duplicate code
pub enum Color {
    Red, 
    Orange,
    Yellow,
    Green,
    Blue, 
    Purple,
    White,
    Black,
    None,
}

impl Color {
    // TODO: Implement None value or some
    // TODO: IMplement Black value
    pub fn from_char(color: char) -> char {
        match color {
            'R' => RED,
            'O' => ORANGE,
            'Y' => YELLOW,
            'G' => GREEN,
            'B' => BLUE,
            'P' => PURPLE,
            'W' => WHITE,
            _ => NONE,
        }
    }

    pub fn get_color(&self) -> char {
        match self {
            Color::Red => RED,
            Color::Orange => ORANGE,
            Color::Yellow => YELLOW,
            Color::Green => GREEN,
            Color::Blue => BLUE,
            Color::Purple => PURPLE,
            Color::White => WHITE,
            Color::Black => BLACK,
            Color::None => NONE
        }
    }

}