// divykj's solution is nice, understandable and short
// my original solution used hashmap to pair up opening/closing brackets
//  this (divykj's) solution conveniently avoids it by directly mapping the pairs in the match!

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    for ch in string.chars() {
        match ch {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '}' | ']' | ')' if stack.pop() != Some(ch) => return false,
            _ => (),
        }
    }
    stack.is_empty()
}
