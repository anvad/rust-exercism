pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    all_defs: Vec<Definition>, // all the definitions ever, in chrono order
    max_defs: usize,           // used to restrict to looking at only previously created defs
    stack: Vec<Value>,         // we'll build the stack of Values from given input string
}

pub struct Definition {
    name: String,
    value: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            all_defs: vec![],
            max_defs: 0,
            stack: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace().map(|s| s.to_ascii_lowercase());

        while let Some(token) = tokens.next() {
            match &token[..] {
                ":" => self.parse_def(&mut tokens)?,
                t if self.is_previously_defined_word(t) => self.exec_user_defined_word(t)?,
                n if n.parse::<i32>().is_ok() => self.stack.push(n.parse::<i32>().unwrap()),
                "*" | "+" | "-" | "/" | "drop" | "dup" | "over" | "swap" => self.exec_op(&token)?,
                _ => Err(Error::UnknownWord)?,
            }
        }
        Ok(())
    }

    fn parse_def<T: Iterator<Item = String>>(&mut self, tokens: &mut T) -> Result {
        let name = tokens.next().ok_or(Error::InvalidWord)?;

        if name.parse::<i32>().is_ok() {
            println!("name cannot be a number {name}! {:?}", Error::InvalidWord);
            return Err(Error::InvalidWord);
        }

        let mut done_parsing_def = false;
        let mut value: Vec<String> = vec![];
        while let Some(token) = tokens.next() {
            if token == ";" {
                done_parsing_def = true;
                break;
            } else if token == ":" {
                println!("starting new def within a def! {:?}", Error::InvalidWord);
                return Err(Error::InvalidWord);
            } else {
                value.push(token);
            }
        }
        if !done_parsing_def || value.len() == 0 {
            // we've run out of tokens but failed to complete current def parsing
            // or no value captured
            return Err(Error::InvalidWord);
        }

        self.all_defs.push(Definition { name, value });
        self.max_defs += 1;
        Ok(())
    }

    fn is_previously_defined_word(&self, word: &str) -> bool {
        // here, we are only looking at the definitions defined up to this point, i.e. upto max_defs
        //  ignoring definitions that were defined later
        self.all_defs[..self.max_defs]
            .iter()
            .rfind(|def| def.name == word)
            .is_some()
    }

    fn exec_user_defined_word(&mut self, word: &str) -> Result {
        // we'll restrict ourselves to only those words defined
        //  before the passed in word
        let saved_max_defs = self.max_defs;
        self.max_defs = self.all_defs[..self.max_defs]
            .iter()
            .rposition(|def| def.name == word)
            .unwrap();

        let new_input = self.all_defs[self.max_defs].value.join(" ");
        self.eval(&new_input)?;

        self.max_defs = saved_max_defs;
        Ok(())
    }

    fn exec_op(&mut self, op: &str) -> Result {
        let rhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
        return match op {
            "dup" => Ok(self.stack.extend([rhs, rhs])),
            "drop" => Ok(()),
            _ => {
                let lhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
                match op {
                    "+" => Ok(self.stack.push(lhs + rhs)),
                    "-" => Ok(self.stack.push(lhs - rhs)),
                    "*" => Ok(self.stack.push(lhs * rhs)),
                    "/" if rhs == 0 => Err(Error::DivisionByZero),
                    "/" => Ok(self.stack.push(lhs / rhs)),
                    "over" => Ok(self.stack.extend([lhs, rhs, lhs])),
                    "swap" => Ok(self.stack.extend([rhs, lhs])),
                    _ => Err(Error::UnknownWord),
                }
            }
        };
    }
}
