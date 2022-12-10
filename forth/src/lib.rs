pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default, Debug, Clone)]
struct CustomOperation {
    name: String,
    opration_chain: Vec<String>,
}

impl CustomOperation {
    fn new(name: String, opration_chain: Vec<String>) -> Self {
        Self {
            name,
            opration_chain,
        }
    }
}

pub struct Forth {
    stack: Vec<Value>,
    custom_ops: Vec<CustomOperation>,
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
        //unimplemented!()
        Forth {
            stack: Vec::<Value>::new(),
            custom_ops: Vec::<CustomOperation>::new(),
        }
    }

    fn apply_op(&mut self, op: &str) -> Result {
        match self
            .custom_ops
            .clone()
            .iter()
            .enumerate()
            .rfind(|(_, co)| co.name == op)
        {
            Some((i, co)) => self.eval(
                co.clone()
                    .opration_chain
                    .iter()
                    .flat_map(|x| {
                        self.custom_ops[..i.max(1)]
                            .iter()
                            .rfind(|o| o.name == *x)
                            .map(|o| o.opration_chain.to_owned())
                            .or(Some(vec![x.to_owned()]))
                            .unwrap()
                            .clone()
                    })
                    .collect::<Vec<_>>()
                    .join(" ") // MUST insert space
                    .as_str(),
            ),
            _ => match op {
                "+" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => Ok(self.stack.push(b + a)),
                    _ => Err(Error::StackUnderflow),
                },
                "-" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => Ok(self.stack.push(b - a)),
                    _ => Err(Error::StackUnderflow),
                },
                "*" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => Ok(self.stack.push(b * a)),
                    _ => Err(Error::StackUnderflow),
                },
                "/" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(0), Some(_)) => Err(Error::DivisionByZero),
                    (Some(a), Some(b)) => Ok(self.stack.push(b / a)),
                    _ => Err(Error::StackUnderflow),
                },
                "swap" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => Ok({
                        self.stack.push(a);
                        self.stack.push(b);
                    }),
                    _ => Err(Error::StackUnderflow),
                },
                "over" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => Ok({
                        self.stack.push(b);
                        self.stack.push(a);
                        self.stack.push(b);
                    }),
                    _ => Err(Error::StackUnderflow),
                },

                // single var action
                "dup" => match self.stack.pop() {
                    Some(x) => Ok({
                        self.stack.push(x);
                        self.stack.push(x);
                    }),
                    _ => Err(Error::StackUnderflow),
                },
                "drop" => match self.stack.pop() {
                    Some(_) => Ok(()),
                    _ => Err(Error::StackUnderflow),
                },
                _ => Err(Error::UnknownWord),
            },
        }
    }

    pub fn stack(&self) -> &[Value] {
        //unimplemented!()
        self.stack.as_slice()
    }

    pub fn parse_custom_op<T: Iterator<Item = String> + Clone>(&mut self, mut op: T) -> Result {
        match String::from_iter(op.clone()).parse::<Value>() {
            Ok(_) => Err(Error::InvalidWord),
            _ => Ok(op
                .next()
                .and_then(|name| {
                    Some(
                        self.custom_ops
                            .push(CustomOperation::new(name, Vec::from_iter(op))),
                    )
                })
                .unwrap()),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        //unimplemented!("result of evaluating '{}'", input)
        let mut iseq = input
            .split_ascii_whitespace()
            .map(|x| x.to_ascii_lowercase());

        while let Some(x) = iseq.next() {
            match x
                .parse::<Value>()
                .and_then(|num| Ok(self.stack.push(num)))
                .or(Err(x))
                .or_else(|x| match x.as_str() {
                    ":" => iseq
                        .clone()
                        .enumerate()
                        .find(|(_, x)| x == ";")
                        .and_then(|(i, _)| Some((i, iseq.clone().take(i))))
                        .and_then(|(next, op)| {
                            (0..=next).into_iter().for_each(|_| {
                                iseq.next();
                            });
                            Some(self.parse_custom_op(op))
                        })
                        .unwrap_or(Err(Error::InvalidWord)),
                    _ => self.apply_op(&x),
                }) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(()) // Some()...
    }
}
