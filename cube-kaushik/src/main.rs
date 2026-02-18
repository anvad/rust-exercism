#[derive(Debug, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug)]
struct Orientation {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug, PartialEq)]
enum PieceType {
    Long,
    Short,
    Tiny,
}

#[derive(Debug, Clone)]
struct UnusedPieces {
    long: u8,
    short: u8,
    tiny: u8,
}

const CUBE_SIZE: u8 = 5;
const NUM_LONG_PIECES: u8 = 6;
const NUM_SHORT_PIECES: u8 = 6;
const NUM_TINY_PIECES: u8 = 5;
const NUM_PIECES: u8 = NUM_LONG_PIECES + NUM_SHORT_PIECES + NUM_TINY_PIECES;
// l1,... l6, s1, ... s3, t1. // ALL THE POSSIBLE ORIENTATIONS OF ALL PIECE TYPES
const POSSIBLE_PIECES: [Piece; 10] = [
    Piece {
        piece_type: PieceType::Long,
        orientation: Orientation { x: 1, y: 2, z: 4 },
    }, // 0
    Piece {
        piece_type: PieceType::Short,
        orientation: Orientation { x: 2, y: 2, z: 3 },
    }, // 6
    Piece {
        piece_type: PieceType::Long,
        orientation: Orientation { x: 4, y: 1, z: 2 },
    }, // 3
    Piece {
        piece_type: PieceType::Short,
        orientation: Orientation { x: 2, y: 3, z: 2 },
    }, // 7
    Piece {
        piece_type: PieceType::Long,
        orientation: Orientation { x: 2, y: 4, z: 1 },
    }, // 4
    Piece {
        piece_type: PieceType::Short,
        orientation: Orientation { x: 3, y: 2, z: 2 },
    }, // 8
    Piece {
        piece_type: PieceType::Tiny,
        orientation: Orientation { x: 1, y: 1, z: 1 },
    }, // 9
    Piece {
        piece_type: PieceType::Long,
        orientation: Orientation { x: 1, y: 4, z: 2 },
    }, // 1
    Piece {
        piece_type: PieceType::Long,
        orientation: Orientation { x: 2, y: 1, z: 4 },
    }, // 2
    Piece {
        piece_type: PieceType::Long,
        orientation: Orientation { x: 4, y: 2, z: 1 },
    }, // 5
];

struct Piece {
    piece_type: PieceType,
    orientation: Orientation,
}

#[derive(Debug, Clone)]
struct State {
    cube_piece_idxs: Vec<u8>, // each number is the index of the piece in the possible_pieces array
    filled_positions: Vec<bool>, // serialized unit cube positions, true if filled

    // these two state variables can be derived from cube_piece_idxs and filled_positions but are kept for performance
    cube_piece_positions: Vec<Position>, // starting positions of the pieces in the cube
    cube_unused_piece_nums: UnusedPieces, // number of unused pieces of each type
}
impl State {
    fn new() -> Self {
        State {
            cube_piece_idxs: Vec::with_capacity(NUM_PIECES as usize),
            filled_positions: vec![false; (CUBE_SIZE * CUBE_SIZE * CUBE_SIZE) as usize],
            cube_piece_positions: Vec::with_capacity(NUM_PIECES as usize),
            cube_unused_piece_nums: UnusedPieces {
                long: NUM_LONG_PIECES,
                short: NUM_SHORT_PIECES,
                tiny: NUM_TINY_PIECES,
            },
        }
    }

    /// if the piece overlaps with another piece, return false
    /// else update the state to reflect the piece being added and return true
    fn try_fill_positions(&self, pos: &Position, piece: &Piece) -> (bool, Vec<bool>) {
        let mut filled_positions = self.filled_positions.clone();
        for x in pos.x..(pos.x + piece.orientation.x) {
            for y in pos.y..(pos.y + piece.orientation.y) {
                for z in pos.z..(pos.z + piece.orientation.z) {
                    let unit_cube_pos_idx = get_index_from_position_coords(x, y, z);
                    if filled_positions[unit_cube_pos_idx as usize] {
                        return (false, self.filled_positions.clone());
                    }
                    filled_positions[unit_cube_pos_idx as usize] = true;
                }
            }
        }
        (true, filled_positions)
    }

    fn try_push_piece(&self, pos: &Position, piece_idx: u8) -> (bool, Self) {
        let piece = &POSSIBLE_PIECES[piece_idx as usize];
        let no_unused_piece = match piece.piece_type {
            PieceType::Long => self.cube_unused_piece_nums.long <= 0,
            PieceType::Short => self.cube_unused_piece_nums.short <= 0,
            PieceType::Tiny => self.cube_unused_piece_nums.tiny <= 0,
        };
        if no_unused_piece {
            return (false, self.clone());
        }
        let piece_sticks_out = pos.x + piece.orientation.x > CUBE_SIZE
            || pos.y + piece.orientation.y > CUBE_SIZE
            || pos.z + piece.orientation.z > CUBE_SIZE;
        if piece_sticks_out {
            return (false, self.clone());
        }
        let (successfull, filled_positions) = self.try_fill_positions(pos, piece);
        if !successfull {
            return (false, self.clone());
        }
        let mut cube_piece_idxs = self.cube_piece_idxs.clone();
        cube_piece_idxs.push(piece_idx);
        let mut cube_piece_positions = self.cube_piece_positions.clone();
        cube_piece_positions.push(pos.clone());
        let mut cube_unused_piece_nums: UnusedPieces = self.cube_unused_piece_nums.clone();
        match piece.piece_type {
            PieceType::Long => cube_unused_piece_nums.long -= 1,
            PieceType::Short => cube_unused_piece_nums.short -= 1,
            PieceType::Tiny => cube_unused_piece_nums.tiny -= 1,
        };
        (
            true,
            State {
                cube_piece_idxs,
                cube_piece_positions,
                cube_unused_piece_nums,
                filled_positions,
            },
        )
    }

    fn print_solution(&self) {
        println!("Solution:");
        let max = self.cube_piece_idxs.len();
        for i in 0..max {
            let piece = &POSSIBLE_PIECES[self.cube_piece_idxs[i] as usize];
            println!(
                "{}: {:?} at position {:?}, with orientation {:?}",
                i + 1,
                piece.piece_type,
                self.cube_piece_positions[i],
                piece.orientation
            );
        }
    }

    fn calc_next_position_idx(&self) -> Option<u8> {
        let calc_next_position_idx = self.filled_positions.iter().position(|&x| !x);
        if let Some(idx) = calc_next_position_idx {
            Some(idx as u8)
        } else {
            println!("All positions are filled!!!!!");
            None
        }
    }

    /// recursive function to find all solutions
    /// if state is not valid, return false
    /// if all pieces are placed, return true
    /// if all children are tried, return false
    /// */
    fn find_next(&self, nodes_traversed: &mut u32) -> Option<Self> {
        let Some(position_idx) = self.calc_next_position_idx() else {
            if self.cube_piece_idxs.len() == NUM_PIECES as usize {
                return Some(self.clone());
            } else {
                println!(
                    "All positions are filled but not all pieces are used up! {:?}",
                    self.cube_piece_idxs
                );
                return None;
            }
        };
        let position = get_position_from_index(position_idx);
        // loop thru all possible pieces and orientations
        // if piece fits in the position
        // update state - add piece to cube, and update filled_positions
        // call find_next with new state
        // if find_next returns a valid state, return it
        // else move to next possible piece
        let max = POSSIBLE_PIECES.len() as u8;
        for possible_piece_idx in 0..max {
            *nodes_traversed += 1;
            let (successful, new_state) = self.try_push_piece(&position, possible_piece_idx);
            if successful {
                if new_state.cube_piece_idxs.len() == NUM_PIECES as usize {
                    println!("Found a solution since all {NUM_PIECES} pieces are placed");
                    // print_solution(new_state)
                    return Some(new_state);
                }
                if let Some(result) = new_state.find_next(nodes_traversed) {
                    return Some(result);
                }
            }
        }
        None
    }
}

fn get_position_coords_from_index(idx: u8) -> (u8, u8, u8) {
    (
        idx % CUBE_SIZE,
        (idx / CUBE_SIZE) % CUBE_SIZE,
        (idx / (CUBE_SIZE * CUBE_SIZE)) % CUBE_SIZE,
    )
}
fn get_position_from_index(idx: u8) -> Position {
    let (x, y, z) = get_position_coords_from_index(idx);
    Position { x, y, z }
}
fn get_index_from_position_coords(x: u8, y: u8, z: u8) -> u8 {
    x + y * CUBE_SIZE + z * CUBE_SIZE * CUBE_SIZE
}
// fn get_index_from_position(pos: Position) -> u8 {
//     get_index_from_position_coords(pos.x, pos.y, pos.z)
// }

fn main() {
    let mut nodes_traversed = 0u32;
    let state = State::new();
    let solution_found = state.find_next(&mut nodes_traversed);
    println!(
        "num_filled_positions={}, num_nodes_traversed={}",
        (&state)
            .filled_positions
            .iter()
            .filter(|v| **v)
            .map(|v| *v)
            .collect::<Vec<bool>>()
            .len(),
        nodes_traversed,
    );
    if let Some(new_state) = solution_found {
        new_state.print_solution();
    } else {
        println!("No solution found!")
    }

    println!("7/5={}", 7u8 / 5u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_main() {
        let start = std::time::Instant::now();
        let mut nodes_traversed = 0u32;
        let state = State::new();
        let solution_found = state.find_next(&mut nodes_traversed);
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        if let Some(new_state) = solution_found {
            new_state.print_solution();
        }
    }
}
