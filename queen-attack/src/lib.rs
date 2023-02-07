#[derive(Debug)]
pub struct ChessPosition(i32,i32);

#[derive(Debug)]
pub struct Queen{
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
/*         unimplemented!(
            "Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return None.",
            rank,
            file
        ); */
        match (rank,file){
            (0..=7,0..=7)=> Some(ChessPosition(rank,file)),
            _=>None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        /* unimplemented!(
            "Given the chess position {:?}, construct a Queen struct.",
            position
        ); */
        Queen{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        /* unimplemented!(
            "Determine if this Queen can attack the other Queen {:?}",
            other
        ); */
        let x_pos = (self.position.0 - other.position.0).abs();
        let y_pos = (self.position.1 - other.position.1).abs();

        x_pos == 0|| y_pos==0||x_pos==y_pos
    }
}
