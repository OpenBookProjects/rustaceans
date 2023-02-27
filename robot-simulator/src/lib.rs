// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        //unimplemented!("Create a robot at (x, y) ({x}, {y}) facing {d:?}")
        return Robot { x, y, d };
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        //unimplemented!()
        match self.d {
            Direction::North => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::East,
                }
            }

            Direction::East => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::South,
                }
            }

            Direction::South => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::West,
                }
            }

            Direction::West => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::North,
                }
            }
        };
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        //unimplemented!()
        match self.d {
            Direction::North => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::West,
                }
            }
            Direction::East => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::North,
                }
            }
            Direction::South => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::East,
                }
            }
            Direction::West => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    d: Direction::South,
                }
            }
        };
    }

    #[must_use]
    pub fn advance(self) -> Self {
        //unimplemented!()

        match self.d {
            Direction::North => {
                return Robot {
                    x: self.x,
                    y: self.y + 1,
                    d: Direction::North,
                }
            }
            Direction::East => {
                return Robot {
                    x: self.x + 1,
                    y: self.y,
                    d: Direction::East,
                }
            }
            Direction::South => {
                return Robot {
                    x: self.x,
                    y: self.y - 1,
                    d: Direction::South,
                }
            }
            Direction::West => {
                return Robot {
                    x: self.x - 1,
                    y: self.y,
                    d: Direction::West,
                }
            }
        };
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        //unimplemented!("Follow the given sequence of instructions: {instructions}")
        for inster in instructions.chars() {
            match inster {
                'L' => self = self.turn_left(),
                'R' => self = self.turn_right(),
                'A' => self = self.advance(),
                _ => {}
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        //unimplemented!()
        return (self.x, self.y);
    }

    pub fn direction(&self) -> &Direction {
        //unimplemented!()
        return &self.d;
    }
}
