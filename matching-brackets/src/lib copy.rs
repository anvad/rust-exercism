use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    // implement stack
    // when we see opening brace, we push
    //  when we see closing brace, we pop and check
    //   the popped element is opening of same type
    //  if not, we return false
    //  else, continue till we run out of chars in string
    //  if stack is empty, return true, else false

    let mut pairs = HashMap::new();
    pairs.insert(')', '(');
    pairs.insert(']', '[');
    pairs.insert('}', '{');

    let mut stack = Vec::<char>::new();
    string
        .chars()
        .filter(|&ch| pairs.iter().any(|(&l, &r)| ch == l || ch == r))
        .try_fold(true, |_, ch| {
            if pairs.contains_key(&ch) {
                let opening = stack.pop()?;
                if pairs.get(&ch).unwrap() != &opening {
                    return None;
                }
            } else {
                stack.push(ch);
            }
            Some(true)
        })
        .is_some()
        && stack.is_empty()
}
