fn main() {
    let h = -3.7_f32;
    assert_eq!(h.floor(), -4.0);

    println!(
        "-1.2 as i8 is {} and as floor is {} and -3.7 floor is {}",
        -1.2 as i8,
        (-1.2_f32).floor(), // .floor is evaluated before the unary minus
        //  so, we have to parenthesize the unary minus to get floor of -1.2
        h.floor(),
    );
}
