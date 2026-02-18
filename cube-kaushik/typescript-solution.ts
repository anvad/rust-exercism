// keeping the index serialized and using a single array instead of a 3D array has a significant performance improvement
// using indexOf instead of for loop to find the next empty position also has a significant performance improvement
// avoiding filter and switch by storing the number of unused pieces of each type in the state has a significant performance improvement
// sequencing the possible_pieces array by the interleaving the long, short, and tiny pieces has a significant performance improvement

type Position = [number, number, number];
type Orientation = [number, number, number];
type PieceType = "long" | "short" | "tiny";

const cube_size = 5;
const [num_long_pieces, num_short_pieces, num_tiny_pieces] = [6, 6, 5];
const num_pieces = num_long_pieces + num_short_pieces + num_tiny_pieces;
type State = {
  cube_piece_idxs: number[]; // each number is the index of the piece in the possible_pieces array
  filled_positions: boolean[]; // serialized unit cube positions, true if filled

  // these two state variables can be derived from cube_piece_idxs and filled_positions but are kept for performance
  cube_piece_positions: Position[]; // starting positions of the pieces in the cube
  cube_unused_piece_nums: { long: number; short: number; tiny: number }; // number of unused pieces of each type
};
type Piece = {
  type: PieceType;
  orientation: Orientation;
};
let nodes_traversed = 0;

// l1,... l6, s1, ... s3, t1
const possible_pieces: Piece[] = [
  { type: "long", orientation: [1, 2, 4] }, // 0
  { type: "short", orientation: [2, 2, 3] }, // 6
  { type: "long", orientation: [4, 1, 2] }, // 3
  { type: "short", orientation: [2, 3, 2] }, // 7
  { type: "long", orientation: [2, 4, 1] }, // 4
  { type: "short", orientation: [3, 2, 2] }, // 8
  { type: "tiny", orientation: [1, 1, 1] }, // 9

  { type: "long", orientation: [1, 4, 2] }, // 1
  { type: "long", orientation: [2, 1, 4] }, // 2
  { type: "long", orientation: [4, 2, 1] }, // 5
];
function get_initial_state(): State {
  return {
    cube_piece_idxs: [],
    filled_positions: new Array(cube_size * cube_size * cube_size).fill(false),
    cube_piece_positions: [],
    cube_unused_piece_nums: {
      long: num_long_pieces,
      short: num_short_pieces,
      tiny: num_tiny_pieces,
    },
  };
}
function get_position_from_index(idx: number): Position {
  return [
    idx % cube_size,
    Math.floor(idx / cube_size) % cube_size,
    Math.floor(idx / (cube_size * cube_size)) % cube_size,
  ];
}
function get_index_from_position(pos: Position): number {
  return pos[0] + pos[1] * cube_size + pos[2] * cube_size * cube_size;
}

function calc_next_position_idx(state: State): number | null {
  const calc_next_position_idx = state.filled_positions.indexOf(false);
  if (calc_next_position_idx !== -1) {
    return calc_next_position_idx;
  }
  console.log("All positions are filled!!!!!");
  return null;
}

/**
 * if the piece is already used up, return false
 * if the piece goes out of bounds, return false
 * if the piece overlaps with another piece, return false
 * else return true and the new state
 * @param state current state
 * @param pos position to place the piece
 * @param piece_idx index of the piece in the possible_pieces array
 * @returns false and old state if the piece cannot be placed
 * @returns true and new state if the piece can be placed
 */
function try_push_piece(
  state: State,
  pos: Position,
  piece_idx: number,
): [boolean, State] {
  const piece = possible_pieces[piece_idx];
  if (state.cube_unused_piece_nums[piece.type] <= 0) {
    return [false, state];
  }

  const [x, y, z] = pos;
  const [ox, oy, oz] = piece.orientation;
  if (x + ox > cube_size || y + oy > cube_size || z + oz > cube_size) {
    return [false, state];
  }

  const [successful, new_filled_positions] = try_fill_positions(
    state.filled_positions,
    pos,
    piece,
  );
  if (!successful) {
    return [false, state];
  }

  return [
    true,
    {
      cube_piece_idxs: [...state.cube_piece_idxs, piece_idx],
      filled_positions: new_filled_positions,
      cube_piece_positions: [...state.cube_piece_positions, pos],
      cube_unused_piece_nums: {
        ...state.cube_unused_piece_nums,
        [piece.type]: state.cube_unused_piece_nums[piece.type] - 1,
      },
    },
  ];
}

/**
 * if the piece overlaps with another piece, return false
 * @param filled_positions - current filled positions
 * @param pos - position to place the piece
 * @param piece - piece to place (type and orientation)
 * @returns true and new filled_positions if the piece can be placed
 */
function try_fill_positions(
  filled_positions: boolean[],
  pos: Position,
  piece: Piece,
): [boolean, boolean[]] {
  const [x, y, z] = pos;
  const [ox, oy, oz] = piece.orientation;
  const new_filled_positions = [...filled_positions];
  for (let i = x; i < x + ox; i++) {
    for (let j = y; j < y + oy; j++) {
      for (let k = z; k < z + oz; k++) {
        const unit_cube_pos_idx = get_index_from_position([i, j, k]);
        if (filled_positions[unit_cube_pos_idx]) {
          return [false, filled_positions];
        }
        new_filled_positions[unit_cube_pos_idx] = true;
      }
    }
  }
  return [true, new_filled_positions];
}

function print_solution(state: State) {
  console.log("Solution:");
  for (let i = 0; i < state.cube_piece_idxs.length; i++) {
    const piece = possible_pieces[state.cube_piece_idxs[i]];
    console.log(
      `${i + 1}: ${piece.type} at position ${state.cube_piece_positions[i]}, with orientation ${piece.orientation}`,
    );
  }
}

/**
 * recursive function to find all solutions
 * if state is not valid, return null
 * if all pieces are placed, return state
 * if all children are tried, return null
 * */
function find_next(state: State = get_initial_state()): State | null {
  const position_idx = calc_next_position_idx(state);
  // console.log(`At level= ${state.cube.length}, position_idx=`, position_idx)
  if (position_idx === null) {
    if (state.cube_piece_idxs.length === num_pieces) {
      return state;
    } else {
      console.log(
        "All positions are filled but not all pieces are used up!",
        state.cube_piece_idxs,
      );
      return null;
    }
  }
  const position = get_position_from_index(position_idx);
  // loop thru all possible pieces and orientations
  // if piece fits in the position
  // update state - add piece to cube, and update filled_positions
  // call find_next with new state
  // if find_next returns a valid state, return it
  // else move to next possible piece
  for (
    let possible_piece_idx = 0;
    possible_piece_idx < possible_pieces.length;
    possible_piece_idx++
  ) {
    nodes_traversed++;
    const [successful, new_state] = try_push_piece(
      state,
      position,
      possible_piece_idx,
    );
    if (successful) {
      if (new_state.cube_piece_idxs.length === num_pieces) {
        console.log(
          `Found a solution since all ${num_pieces} pieces are placed`,
        );
        // print_solution(new_state)
        return new_state;
      }
      const result = find_next(new_state);
      if (result) {
        return result;
      }
    }
  }
  return null;
}

const solution = find_next();
console.log(
  "num_filled_positions=",
  solution?.filled_positions.filter((v) => v).length,
  `num_nodes_traversed=${nodes_traversed}`,
);
if (solution) {
  print_solution(solution);
} else {
  console.log("No solution found!");
}
