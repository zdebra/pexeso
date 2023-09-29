use std::collections::{HashMap, LinkedList};
fn main() {
    println!("Starting game of pexeso!");

    let mut game = Game::new(GameSize::Tiny, 1);

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
            MoveResult::Match => println!("Match!"),
            MoveResult::NoMatch => println!("No match!"),
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
    matches: HashMap<Position, Position>,
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
            matches: generate_matches(size),
        }
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
        if self.matches.get(&move_instruction.first_item).unwrap() == &move_instruction.second_item
        {
            self.history
                .push_back((move_instruction, MoveResult::Match));
            MoveResult::Match
        } else {
            MoveResult::NoMatch
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

fn generate_matches(size: GameSize) -> HashMap<Position, Position> {
    let (max_x, max_y) = size.into_tuple();
    let mut rng = thread_rng();
    let x_shuffled = {
        let mut x: Vec<u32> = (0..max_x).collect();
        x.shuffle(&mut rng);
        x
    };
    let y_shuffled = {
        let mut y: Vec<u32> = (0..max_y).collect();
        y.shuffle(&mut rng);
        y
    };

    let mut matches = HashMap::new();
    for i in 0..max_x {
        for j in 0..max_y {
            matches.insert((i, j), (x_shuffled[i as usize], y_shuffled[j as usize]));
        }
    }
    matches
}

type Position = (u32, u32);

struct MoveInstruction {
    player_id: u32,
    first_item: Position,
    second_item: Position,
}

enum MoveResult {
    Match,
    NoMatch,
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
