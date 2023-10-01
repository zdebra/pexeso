use std::collections::LinkedList;
fn main() {
    println!("Starting game of pexeso!");

    let mut game = Game::new(GameSize::Tiny, 1);

    game.reveal();

    loop {
        println!("Enter your move:");
        let first_item = read_position();
        let second_item = read_position();
        let move_instruction = MoveInstruction {
            player_id: 0,
            first_item,
            second_item,
        };
        let move_result = game.play(move_instruction);
        match move_result {
            MoveResult::Match {
                first_item,
                second_item,
            } => {
                println!("Match!");
                println!("{}", game);
            }
            MoveResult::NoMatch {
                first_item,
                second_item,
            } => {
                println!("No match!");
                println!("{}", game);
            }
            MoveResult::InvalidMove => println!("Invalid move!"),
            MoveResult::InvalidPlayer => println!("Invalid player!"),
            MoveResult::InvalidMoveGameOver => {
                println!("Game over!");
                break;
            }
        }
    }
}

fn read_position() -> Position {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let parts: Vec<_> = input
        .split(",")
        .map(|part| {
            let part = part.trim();
            part.parse::<u32>().expect("invalid number")
        })
        .collect();
    assert!(parts.len() == 2, "invalid input");
    (parts[0], parts[1])
}

struct Game {
    size: GameSize,
    player_count: u32,
    history: LinkedList<(MoveInstruction, MoveResult)>,
    next_player: u32,
    board: Vec<Vec<Picture>>,
}

impl std::fmt::Display for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                let display_value = {
                    if self.history.iter().any(|(mv_instr, mv_res)| {
                        if mv_instr.first_item != (i as u32, j as u32)
                            && mv_instr.second_item != (i as u32, j as u32)
                        {
                            return false;
                        }

                        if let MoveResult::Match {
                            first_item: _,
                            second_item: _,
                        } = mv_res
                        {
                            return true;
                        } else {
                            return false;
                        }
                    }) {
                        let item = &self.board[i][j];
                        emojis::get_by_shortcode(&item.id).unwrap().as_str()
                    } else {
                        "🔵"
                    }
                };
                fmt.write_str(display_value).unwrap();
            }
            fmt.write_str("\n").unwrap();
        }
        Ok(())
    }
}

impl Game {
    fn new(size: GameSize, player_count: u32) -> Self {
        assert!(
            player_count > 0,
            "at least one player has to play this game"
        );
        Self {
            size: size.clone(),
            player_count,
            history: LinkedList::new(),
            next_player: 0,
            board: generate_board(size),
        }
    }

    fn reveal(&self) {
        print!("  ");
        for i in 0..self.board.len() {
            print!("{} ", i);
        }
        println!();
        print!(" |");
        for _ in 0..self.board.len() {
            print!("--");
        }
        println!();
        self.board.iter().enumerate().for_each(|(i, row)| {
            print!("{}|", i);
            row.iter().for_each(|item| {
                print!("{}", emojis::get_by_shortcode(&item.id).unwrap().as_str())
            });
            print!("\n")
        })
    }

    fn play(&mut self, move_instruction: MoveInstruction) -> MoveResult {
        let (x, y) = self.size.into_tuple();
        if self.history.len() as u32 == x * y {
            return MoveResult::InvalidMoveGameOver;
        }
        if move_instruction.player_id != self.next_player {
            return MoveResult::InvalidPlayer;
        }
        if !self.is_move_valid(move_instruction.first_item, move_instruction.second_item) {
            return MoveResult::InvalidMove;
        }

        self.next_player = (self.next_player + 1) % self.player_count;

        let (f_x, f_y) = move_instruction.first_item;
        let first_picture = &self.board[f_x as usize][f_y as usize];

        let (s_x, s_y) = move_instruction.second_item;
        let second_picture = &self.board[s_x as usize][s_y as usize];

        if first_picture == second_picture {
            self.history.push_back((
                move_instruction,
                MoveResult::Match {
                    first_item: first_picture.clone(),
                    second_item: second_picture.clone(),
                },
            ));
            MoveResult::Match {
                first_item: first_picture.clone(),
                second_item: second_picture.clone(),
            }
        } else {
            MoveResult::NoMatch {
                first_item: first_picture.clone(),
                second_item: second_picture.clone(),
            }
        }
    }

    fn is_move_valid(&self, first: Position, second: Position) -> bool {
        if first == second {
            return false;
        }

        // check if the move is already in history
        if self.history.iter().any(|(move_instruction, _)| {
            move_instruction.first_item == first && move_instruction.second_item == second
        }) {
            return false;
        }

        // check if move is out of bounds
        let (max_x, max_y) = self.size.into_tuple();
        if first.0 >= max_x || first.1 >= max_y || second.0 >= max_x || second.1 >= max_y {
            return false;
        }

        true
    }
}

use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_board(size: GameSize) -> Vec<Vec<Picture>> {
    let (max_x, max_y) = size.into_tuple();
    let total_items = ((max_x * max_y) / 2) as usize;
    assert!(
        emojis::Group::FoodAndDrink
            .emojis()
            .collect::<Vec<_>>()
            .len()
            >= total_items
    );

    let mut rng = thread_rng();
    let mut em: Vec<_> = emojis::Group::FoodAndDrink.emojis().collect();
    em.shuffle(&mut rng);

    let mut items = em.into_iter().take(total_items).collect::<Vec<_>>();
    let mut matches = items.clone();
    items.append(&mut matches);
    items.shuffle(&mut rng);

    let mut board = Vec::new();
    for _ in 0..max_x {
        let mut row = Vec::new();
        for _ in 0..max_y {
            row.push(Picture {
                id: items.pop().unwrap().shortcode().unwrap().to_string(),
            })
        }
        board.push(row);
    }
    board
}

type Position = (u32, u32);

struct MoveInstruction {
    player_id: u32,
    first_item: Position,
    second_item: Position,
}

#[derive(Clone, PartialEq)]
struct Picture {
    id: String,
}

enum MoveResult {
    Match {
        first_item: Picture,
        second_item: Picture,
    },
    NoMatch {
        first_item: Picture,
        second_item: Picture,
    },
    InvalidMove,
    InvalidPlayer,
    InvalidMoveGameOver,
}

const GAME_SIZE_TINY: Position = (4, 4);
const GAME_SIZE_SMALL: Position = (8, 8);
const GAME_SIZE_MEDIUM: Position = (16, 16);
const GAME_SIZE_LARGE: Position = (32, 32);

#[derive(Clone)]
enum GameSize {
    Tiny,
    Small,
    Medium,
    Large,
}

impl GameSize {
    fn into_tuple(&self) -> Position {
        match self {
            GameSize::Tiny => GAME_SIZE_TINY,
            GameSize::Small => GAME_SIZE_SMALL,
            GameSize::Medium => GAME_SIZE_MEDIUM,
            GameSize::Large => GAME_SIZE_LARGE,
        }
    }
}
