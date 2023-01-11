pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    all_defs: Vec<Definition>, // all the definitions ever, in chrono order
    max_defs: usize,
    stack: Vec<Value>, // we'll build the stack of evals from given input
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

    pub fn exec_user_defined_word(&mut self, t: &str) -> Result {
        println!("evaluating/exec'ing defined word: {t}");
        let saved_max_defs = self.max_defs;
        self.max_defs = self.all_defs[..self.max_defs]
            .iter()
            .rposition(|def| def.name == t)
            .unwrap();

        let new_input = self.all_defs[self.max_defs].value.join(" ");
        println!(
            "{t} maps to {new_input} with (max, saved)= ({},{})",
            self.max_defs, saved_max_defs
        );
        println!("self.stack (before eval)= {:?}", self.stack);
        let res = self.eval(&new_input);
        if res.is_err() {
            return res;
        }

        self.max_defs = saved_max_defs;
        Ok(())
    }

    pub fn exec_op(&mut self, op: &str) -> Result {
        let min_len = match op {
            "dup" => 1,
            "drop" => 1,
            _ => 2,
        };
        if self.stack.len() < min_len {
            return Err(Error::StackUnderflow);
        }

        let rhs = self.stack.pop().unwrap();
        match op {
            "dup" => {
                println!("executing dup. stack before: {:?}", self.stack);
                self.stack.push(rhs);
                self.stack.push(rhs);
                println!("rhs={rhs}. stack after: {:?}", self.stack);
                return Ok(());
            }
            "drop" => {
                return Ok(());
            }
            _ => (),
        };

        let lhs = self.stack.pop().unwrap();
        match op {
            "add" => Ok(self.stack.push(lhs + rhs)),
            "sub" => Ok(self.stack.push(lhs - rhs)),
            "mul" => Ok(self.stack.push(lhs * rhs)),
            "div" if rhs == 0 => Err(Error::DivisionByZero),
            "div" => Ok(self.stack.push(lhs / rhs)),
            "over" => {
                self.stack.push(lhs);
                self.stack.push(rhs);
                self.stack.push(lhs);
                Ok(())
            }
            "swap" => {
                self.stack.push(rhs);
                self.stack.push(lhs);
                Ok(())
            }
            _ => Err(Error::UnknownWord),
        }
    }

    pub fn is_previously_defined_word(&self, t: &str) -> bool {
        // here, we are only looking at the definitions defined up to this point, i.e. upto max_defs
        //  ignoring definitions that were defined later
        self.all_defs[..self.max_defs]
            .iter()
            .rfind(|def| def.name == t)
            .is_some()
    }

    pub fn parse_def<T: Iterator<Item = String>>(&mut self, tokens: &mut T) -> Result {
        // starting a new def!
        println!("starting new def!");
        let name: String;
        if let Some(token) = tokens.next() {
            if token.parse::<i32>().is_ok() {
                println!("name cannot be a number {token}! {:?}", Error::InvalidWord);
                return Err(Error::InvalidWord);
            }
            name = token;
            println!("name: {name}");
        } else {
            println!("{:?}", Error::InvalidWord);
            return Err(Error::InvalidWord);
        }

        let mut parsing_def = true;
        let mut value: Vec<String> = vec![];
        while let Some(token) = tokens.next() {
            if token == ";" {
                parsing_def = false;
                break;
            } else if token == ":" {
                println!("starting new def within a def! {:?}", Error::InvalidWord);
                return Err(Error::InvalidWord);
            } else {
                println!("value token: {token}");
                value.push(token);
            }
        }
        if parsing_def {
            // we've run out of tokens but failed to complete current def parsing
            println!("def incomplete! {:?}", Error::InvalidWord);
            return Err(Error::InvalidWord);
        }
        if value.len() == 0 {
            // at least one element needed
            println!("no value! {:?}", Error::InvalidWord);
            return Err(Error::InvalidWord);
        }

        self.all_defs.push(Definition { name, value });
        self.max_defs += 1;
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace().map(|s| s.to_ascii_lowercase());

        while let Some(token) = tokens.next() {
            match &token[..] {
                ":" => self.parse_def(&mut tokens)?,
                t if self.is_previously_defined_word(t) => self.exec_user_defined_word(t)?,
                "+" => self.exec_op("add")?,
                "-" => self.exec_op("sub")?,
                "*" => self.exec_op("mul")?,
                "/" => self.exec_op("div")?,
                "dup" => self.exec_op("dup")?,
                "drop" => self.exec_op("drop")?,
                "over" => self.exec_op("over")?,
                "swap" => self.exec_op("swap")?,
                t if t.parse::<i32>().is_ok() => {
                    self.stack.push(t.parse::<i32>().unwrap());
                    println!("stack after pushing number: {:?}", self.stack);
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }
        Ok(())
    }
}
