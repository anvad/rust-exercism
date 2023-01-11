use std::collections::HashMap;
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    commands: HashMap<String, String>,
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
            stack: vec![],
            commands: HashMap::new(),
        }
    }
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace();
        while let Some(token) = tokens.next() {
            match token.to_lowercase().as_str() {
                token if self.commands.contains_key(token) => {
                    let command = self.commands.get(token).unwrap().to_owned();
                    let _ = self.eval(&command)?;
                }
                "+" | "-" | "*" | "/" => {
                    let a = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
                    match token {
                        "+" => self.stack.push(a + b),
                        "-" => self.stack.push(b - a),
                        "*" => self.stack.push(a * b),
                        _ if a == 0 => return Err(Error::DivisionByZero),
                        _ => self.stack.push(b / a),
                    }
                }
                "dup" => {
                    let value = self.stack.last().ok_or_else(|| Error::StackUnderflow)?;
                    self.stack.push(*value);
                }
                "drop" => {
                    if self.stack.pop().is_none() {
                        return Err(Error::StackUnderflow);
                    }
                }
                "swap" => {
                    let a = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
                    self.stack.push(a);
                    self.stack.push(b);
                }
                "over" => {
                    let a = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
                    self.stack.push(b);
                    self.stack.push(a);
                    self.stack.push(b);
                }
                ":" => {
                    let alias = tokens
                        .next()
                        .ok_or_else(|| Error::InvalidWord)?
                        .to_lowercase();
                    if alias.starts_with(|c: char| c.is_digit(10)) {
                        return Err(Error::InvalidWord);
                    }
                    let mut commands = vec![];
                    let mut found = false;
                    while let Some(token) = tokens.next() {
                        if token == ";" {
                            found = true;
                            break;
                        }
                        let mut command = token.to_lowercase();
                        if let Some(cmd) = self.commands.get(&command) {
                            if cmd.parse::<Value>().is_ok() {
                                command = cmd.clone();
                            }
                        }
                        commands.push(command);
                    }
                    if !found {
                        return Err(Error::InvalidWord);
                    }
                    self.commands.insert(alias, commands.join(" "));
                }
                _ if token.starts_with(|a: char| a.is_digit(10)) => {
                    let value = token.parse::<Value>().map_err(|_| Error::InvalidWord)?;
                    self.stack.push(value);
                }
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }
}
