const MAX_FRAMES: usize = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct Frame {
    rolls: Vec<Roll>,
}

#[derive(Copy,Clone, Default, Debug)]
pub struct Roll {
    pins: u16,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    current_roll: Option<Roll>,
    frames: Vec<Frame>,
}

impl Frame{
    fn new(r1:Roll,r2:Roll) -> Result<Self,Error>{
        if r1+r2>10{
            Err(Error::NotEnoughPinsLeft)
        }else{
            Ok(Self{
                rolls: vec![r1,r2],
            })
        }
    }

    fn strike()->Self{
        Self{
            rolls: vec![Roll{pins:10u16}],
        }
    }

    fn is_strike(&self) -> bool{
        self.rolls.first().map_or(false,|r| r.pins==10)
    }

    fn is_finished(&self)->bool{
        (!self.is_spare()&&!self.is_strike())||self.rolls.len()==3
    }

    fn is_spare(&self) -> bool{
        !self.is_strike() && self.rolls
                                .iter()
                                .take(2)
                                .map(|r| r.pins)
                                .sum::<u16>()==10u16
    }

    fn roll(&mut self,roll:Roll)->Result<(),Error>{
        let last = self.rolls.last().map(|last| *last).unwrap_or_default();

        if self.is_spare()||(self.is_strike() && (last==10||last+roll<=10)){
            self.rolls.push(roll);
            Ok(())
        }else{
            Err(Error::NotEnoughPinsLeft)
        }
    }


}


impl Roll{
    fn new(pins:u16)-> Result<Self,Error>{
        if pins>10{
            Err(Error::NotEnoughPinsLeft)
        }else{
            Ok(Self{pins})
        }
    }
}

impl std::cmp::PartialEq<u16> for Roll{
    fn eq(&self, c:&u16) -> bool{
        self.pins == *c
    }
}

impl std::ops::Add for Roll{
    type Output = u16;

    fn add(self, crt:Self) -> Self::Output{
        self.pins + crt.pins
    }
}



impl BowlingGame {
    pub fn new() -> Self {
        //unimplemented!();
        Self::default()
    }

    fn has_last_frame(&self)->bool{
        self.frames.len() == MAX_FRAMES
    }

    fn is_game_finished(&self)->bool{
        self.has_last_frame() && self.frames.last().unwrap().is_finished()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        //unimplemented!("Record that {} pins have been scored", pins);
        let roll = Roll::new(pins)?; // ?<- with Option
        if self.is_game_finished(){
            return Err(Error::GameComplete);
        }

        for last in self
                    .frames
                    .iter_mut()
                    .rev()
                    .take(2)
                    .filter(|last| !last.is_finished())
        {
            last.roll(roll)?;
        }
        
        if !self.has_last_frame(){
            if let Some(r) = self.current_roll.take(){
                self.frames.push(Frame::new(r,roll)?); //?<--
            }else if roll==10{
                self.frames.push(Frame::strike());
            }else{
                self.current_roll = Some(roll);
            }

        }
        Ok(()) // <--
    }

    pub fn score(&self) -> Option<u16> {
        //unimplemented!("Return the score if the game is complete, or None if not.");
        if self.is_game_finished(){
            Some(self.frames.iter().fold(0u16,|acc,f|{
                acc+f.rolls.iter().fold(0u16,|sum,r| sum+r.pins)
            }))
        }else{
            None
        }
    }
}





