#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (flen, slen) = (first_list.len(), second_list.len());
    if flen == slen {
        if first_list == second_list {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    } else if flen > slen {
        // this is not optimal. we should be able to skip
        //  multiple steps based on where the mismatch occurred
        for i in 0..(flen - slen + 1) {
            if first_list[i..(slen + i)] == *second_list {
                return Comparison::Superlist;
            }
        }
        return Comparison::Unequal;
    } else {
        for i in 0..(slen - flen + 1) {
            if second_list[i..(flen + i)] == *first_list {
                return Comparison::Sublist;
            }
        }
        return Comparison::Unequal;
    }
}
