use crate::{HEIGHT, WIDTH};

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    YELLOW,
    RED,
    NONE,
}

#[derive(Copy, Clone)]
pub struct GameState {
    turn: Color,
    fields: [[Color; HEIGHT]; WIDTH], //
}

impl GameState {
    pub fn new() -> GameState {
        return GameState { turn: Color::YELLOW, fields: [[Color::NONE; HEIGHT]; WIDTH] };
    }

    pub fn string_representation(&self) -> String {
        let mut representation = String::new();
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                let character = match self.fields[x][y] {
                    Color::YELLOW => { '⚪' }
                    Color::RED => { '⚫' }
                    Color::NONE => { '⭕' }
                };
                representation.push(character);
            }
            representation.push('\n');
        }
        return representation;
    }

    pub fn possible_moves(&self) -> Vec<GameState> {
        let mut states = Vec::new();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.fields[x][y] != Color::NONE {
                    continue;
                }
                let new_turn = match self.turn {
                    Color::YELLOW => { Color::RED }
                    Color::RED => { Color::YELLOW }
                    Color::NONE => { panic!("WTF happened here, it's nobody's turn :)") }
                };
                let mut new_fields = self.fields.clone();
                new_fields[x][y] = self.turn.clone();
                let new_state = GameState { turn: new_turn, fields: new_fields };
                states.push(new_state);
                break;
            }
        }
        return states;
    }

    pub fn winner(&self) -> Color {
        for color in [Color::YELLOW, Color::RED] {
            //rows
            if self.row_winner(color) || self.col_winner(color) {
                return color;
            }
            //diagonals
            todo!();
        }
        return Color::NONE;
    }

    //todo consolidate row and col into one function
    // make unnecessary checks obsolete, by comparing streak to spaces left
    fn row_winner(&self, color_to_check: Color) -> bool {
        for row in 0..HEIGHT {
            let mut streak = 0;
            for x in 0..WIDTH {
                if self.fields[x][row] == color_to_check {
                    streak = streak + 1;
                    if streak > 3 {
                        return true;
                    }
                } else {
                    streak = 0;
                }
            }
        }
        return false;
    }

    fn col_winner(&self, color_to_check: Color) -> bool {
        for col in 0..WIDTH {
            let mut streak = 0;
            for y in 0..HEIGHT {
                if self.fields[col][y] == color_to_check {
                    streak = streak + 1;
                    if streak > 3 {
                        return true;
                    }
                } else {
                    streak = 0;
                }
            }
        }
        return false;
    }

    fn diag_winner(&self, color_to_check: Color) -> bool {

    }
}
