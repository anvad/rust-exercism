#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use crate::CalculatorInput::*;

// mine (iteration 1 - i.e. previous iteration) was such a procedural solution
// check out insid eoutclub's solution!

// i learnt (or remembered) a few things by looking at his solution
//  1. map can be used on an Option type
//  2. and_then assumes the called closure/function will return Option type
//     (or whatever container type was given to and_then)
//  3. whereas, map will wrap the raw result from closure/function into an Option
//     (or whatever original container type was given to map)
//  4. and_then is aka flat_map since it does not re-wrap the function/closure result
//  5. both map and flatmap will do an early return if the Option is of type None
//  5. as_slice()

use std::ops::{Add, Div, Mul, Sub};

fn binary_op(stack: &mut Vec<i32>, op: fn(i32, i32) -> i32) -> Option<i32> {
    stack
        .pop()
        .and_then(|right| stack.pop().map(|left| op(left, right)))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let stack = inputs.iter().try_fold(vec![], |mut stack, input| {
        // must return stack from this closure
        match *input {
            Value(num) => Some(num),
            Add => binary_op(&mut stack, i32::add),
            Subtract => binary_op(&mut stack, i32::sub),
            Multiply => binary_op(&mut stack, i32::mul),
            Divide => binary_op(&mut stack, i32::div),
        }
        .map(|num| {
            stack.push(num);
            stack
        })
    });
    stack.and_then(|stack| match stack.as_slice() {
        [num] => Some(*num),
        _ => None,
    })
}
