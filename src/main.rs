use std::collections::{HashMap, LinkedList};
fn main() {
    println!("Hello, world!");
}

struct Game {
    pairs_count: u32,
    player_count: u32,
    history: LinkedList<(MoveInstruction, MoveResult)>,
    next_player: u32,
    matches: HashMap<(u32, u32), (u32, u32)>,
}

impl Game {
    fn new(pairs_count: u32, player_count: u32) -> Self {
        assert!(
            (max_x + 1) * (max_y + 1) % 2 == 0,
            "expected even number of items"
        );
        assert!(
            player_count > 0,
            "at least one player has to play this game"
        );
        Self {
            size,
            player_count,
            history: LinkedList::new(),
            next_player: 0,
            matches: generate_matches(size),
        }
    }

    fn play(&mut self, move_instruction: MoveInstruction) -> MoveResult {
        if move_instruction.player_id != self.next_player {
            return MoveResult::InvalidPlayer;
        }
        if !self.is_move_valid(move_instruction.first_item, move_instruction.second_item) {
            return MoveResult::InvalidMove;
        }

        if self.matches.get(&move_instruction.first_item).unwrap() == &move_instruction.second_item
        {
            self.history
                .push_back((move_instruction, MoveResult::Match));
            MoveResult::Match
        } else {
            MoveResult::NoMatch
        }
    }

    fn is_move_valid(&self, first: (u32, u32), second: (u32, u32)) -> bool {
        todo!()
    }
}

use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_matches(max_x: u32, max_y: u32) -> HashMap<(u32, u32), (u32, u32)> {
    assert!((max_x + 1) * (max_y + 1) % 2 == 0);
    let num_pairs = (max_x + 1) * (max_y + 1) / 2;
    let mut first_x: Vec<u32> = (0..max_x).collect();
    let mut first_y: Vec<u32> = (0..max_y).collect();

    let mut rng = thread_rng();
    first_x.shuffle(&mut rng);
    first_y.shuffle(&mut rng);
}

struct MoveInstruction {
    player_id: u32,
    first_item: (u32, u32),
    second_item: (u32, u32),
}

enum MoveResult {
    Match,
    NoMatch,
    InvalidMove,
    InvalidPlayer,
}
