#[derive(Debug)]
pub struct Definition {
    name: String,
    value: Vec<String>,
}

fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4, 5];
    let mut ai = a.iter();
    // let b: Vec<_> = ai.enumerate().map(|(_, &n)| {
    //     if n % 2 == 0 {
    //         n + *(&ai).next().unwrap_or(&0)
    //     } else {
    //         n
    //     }
    // }).collect();
    let mut b = vec![0];
    while let Some(&n) = ai.next() {
        // let n = a[i];
        if n % 2 == 0 {
            b.push(n + ai.next().unwrap());
        } else {
            b.push(n);
        }
    }
    println!("b={:?}", b);

    let a = ": foo 1 ; : bar foo foo + ; : foo 5 ; bar ";
    let mut ti = a.split_whitespace().map(|s| s.to_ascii_lowercase());
    let mut defs: Vec<Definition> = vec![];
    while let Some(token) = ti.next() {
        if token == ":" {
            // starting a new def!
            println!("starting new def!");
            let name: String; // = "unknown".to_string();
            if let Some(token) = ti.next() {
                name = token; //.to_string();
                println!("name: {name}");
            } else {
                // return Err(Error::InvalidWord);
                println!("{:?}", forth::Error::InvalidWord);
                return;
            }

            let mut parsing_def = true;
            let mut value: Vec<String> = vec![];
            while let Some(token) = ti.next() {
                if token == ";" {
                    parsing_def = false;
                    break;
                } else {
                    println!("value token: {token}");
                    value.push(token);
                }
            }
            if parsing_def {
                // we've run out of tokens but failed to complete current def parsing
                // return Err(Error::InvalidWord);
                println!("def incomplete! {:?}", forth::Error::InvalidWord);
                return;
            }
            if value.len() == 0 {
                // at least one element needed
                // return Err(Error::InvalidWord);
                println!("no value! {:?}", forth::Error::InvalidWord);
                return;
            }

            let def = Definition { name, value };
            println!("name,value= {},{:?}", def.name, def.value);
            defs.push(def);
        }
    }
    println!("defs: {:?}", defs);
    // let new_defs: Vec<_> = defs
    //     .iter()
    //     .rev()
    //     .take_while(|def| def.name != "bar")
    //     .collect();

    // println!("new_defs: {:#?}", new_defs);
    // println!("defs: {:#?}", defs);

    let split_pos = defs.iter().rposition(|def| def.name == "foo").unwrap();
    let (lhs, rhs) = defs.split_at(split_pos);
    println!("lhs={:?}\nrhs={:?}", lhs, rhs);
}
