use std::collections::{BTreeMap, BTreeSet};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    // because we want to grades to be returned as an ordered list
    //  and the students within a grade to also be returned as an ordered list
    //  it made sense to use BTreeMap and BTreeSet, rather than use
    //  HashMap and a Vec and call sort on HashMap output each time `.grades()` is called
    //  for students, Vec makes more sense than BTreeSet, since multiple students can have same name
    roster: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            roster: Default::default(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .and_modify(|students| {
                students.insert(student.to_owned());
            })
            .or_insert(BTreeSet::from([student.to_owned()]));
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .cloned()
            .collect()
    }
}
