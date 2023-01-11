use std::collections::HashMap;
use std::mem::replace;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
const INSTRUCTIONS: [&str; 8] = ["+", "-", "*", "/", "swap", "over", "drop", "dup"];
pub struct Forth {
    user_defs: HashMap<String, String>,
    stack: Vec<Value>,
}
#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}
impl Forth {
    pub fn new() -> Forth {
        Self {
            user_defs: HashMap::new(),
            stack: Vec::new(),
        }
    }
    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }
    pub fn eval(&mut self, input: &str) -> Result {
        let mut string = input.to_ascii_lowercase();
        let mut stack = self.stack.clone();
        self.eval_loop(&mut stack, &mut string)?;
        self.stack = stack;
        Ok(())
    }
    fn eval_loop(&mut self, stack: &mut Vec<Value>, input: &mut String) -> Result {
        while let Some(word) = Forth::next_word(input) {
            match word {
                w if w == String::from(":") => self.create_user_def(input)?,
                w if self.user_defs.contains_key(&w) => {
                    *input = self.user_defs[&w].clone() + " " + input.as_str()
                }
                w if INSTRUCTIONS.contains(&w.as_str()) => Forth::execute_instruction(stack, &w)?,
                w if w.parse::<Value>().is_ok() => stack.push(w.parse::<Value>().unwrap()),
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }
    fn next_word(input: &mut String) -> Option<String> {
        if let Some(idx) = input.find(' ') {
            let res = input.split_off(idx).trim().to_string();
            Some(replace(input, res))
        } else if input.is_empty() {
            None
        } else {
            Some(replace(input, String::new()))
        }
    }
    fn execute_instruction(stack: &mut Vec<Value>, word: &str) -> Result {
        let len = stack.len();
        if len < INSTRUCTIONS[..6].contains(&word) as usize + 1 {
            return Err(Error::StackUnderflow);
        }
        let last_element = stack.pop().unwrap();
        match word {
            "+" => stack[len - 2] += last_element,
            "-" => stack[len - 2] -= last_element,
            "*" => stack[len - 2] *= last_element,
            "/" => {
                if last_element == 0 {
                    return Err(Error::DivisionByZero);
                } else {
                    stack[len - 2] /= last_element;
                }
            }
            "dup" => stack.append(&mut vec![last_element, last_element]),
            "swap" => stack.insert(len - 2, last_element),
            "over" => stack.append(&mut vec![last_element, stack[len - 2]]),
            _ => (),
        }
        Ok(())
    }
    fn create_user_def(&mut self, input: &mut String) -> Result {
        if let Some(idx) = input.find(';') {
            let res = input.split_off(idx + 1).trim().to_string();
            let chunks = replace(input, res)
                .split_ascii_whitespace()
                .filter(|&chk| chk != ";" && !chk.is_empty())
                .map(|chk| chk.to_string())
                .collect::<Vec<String>>();
            if chunks[0].parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }
            let mut stack = Vec::new();
            let eval_result = self.eval_loop(&mut stack, &mut chunks[1..].join(" "));
            let mut value_str = stack
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            if eval_result.is_err() {
                if eval_result == Err(Error::StackUnderflow) {
                    value_str = chunks[1..].join(" ");
                } else if eval_result != Err(Error::UnknownWord) {
                    return eval_result;
                }
            }
            self.user_defs.insert(chunks[0].clone(), value_str);
            Ok(())
        } else {
            Err(Error::InvalidWord)
        }
    }
}
