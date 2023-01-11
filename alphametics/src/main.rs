// let's try to use backtracking. see https://algotree.org/algorithms/backtracking/

// in each state, we maintain a list of available options
//  if taking one available option does not work out, we try the next
//  once we are out of all available options, we go back to previous step
//  and then repeat by going to the next available option (at the previous step)
//  if the root (or starting step) runs out of options, then it means no solution is possible

// so what kind of struct should we use to maintain state and available options?

// as we progress thru each step, we'll have associated one more letter with a number
// so state will be:
//  hashmap of letter->number, (mut arg)
//  current_letter, (arg)
//  available_numbers_for_current_letter,
//  vec_remaining_letters (arg)
// all of these can be local variables or args in a recursively called function
//  so, forward progress means making another recursive call
//  unwind means going back to previous call
//  how do we exit?
//      let a call return a result<hashmap, err>
//      so, if we get an Ok(), we return to our caller

use std::collections::HashMap;

use alphametics;
fn main() {
    // alphametics::solve("AS + A == MOM");
    alphametics::solve("NO + NO + TOO == LATE");
    // alphametics::solve("SEND + MORE == MONEY");

    // 9k solver calls, 36k validate calls
    // 12k/48k next time. why is this not deterministic?
    // alphametics::solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");

    // 855k solver calls, 1.1m validate calls
    // 555k/687k with rev, 538k/659k without rev
    // alphametics::solve("THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES");

    let column = ['O', 'O', 'O', 'E'];
    let col_map = column
        .iter()
        .fold(HashMap::<char, u32>::new(), |mut col_map, ch| {
            *col_map.entry(*ch).or_insert(0) += 1;
            col_map
        });
    println!("col_map={:?}", col_map);
}
