#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn rpn_proc(items: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for item in items {
        match item {
            CalculatorInput::Value(number) => stack.push(*number),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();

                match item {
                    CalculatorInput::Add => stack.push(l + r),
                    CalculatorInput::Subtract => stack.push(l - r),
                    CalculatorInput::Multiply => stack.push(l * r),
                    CalculatorInput::Divide => stack.push(l / r),
                    _ => return None,
                }
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    return stack.pop();
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    /* unimplemented!(
            "Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
            inputs,
        );

        RPN Calculator in Rust
    http://www.math.bas.bg/bantchev/place/rpn/rpn.rust.html
        */
    if inputs.len() == 0 {
        return None;
    }
    rpn_proc(inputs)
}
