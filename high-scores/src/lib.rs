use std::cmp::min;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    scores_sorted: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        /* unimplemented!(
            "Construct a HighScores struct, given the scores: {:?}",
            scores
        ) */

        let scores = scores.to_vec();
        let mut scores_sorted = scores.clone();
        // partial_cmp()
        scores_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

        Self {
            scores,
            scores_sorted,
        }
    }

    fn len4s(&self) -> usize {
        self.scores.len()
    }
    pub fn scores(&self) -> &[u32] {
        //unimplemented!("Return all the scores as a slice")
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        //unimplemented!("Return the latest (last) score")

        match self.len4s() {
            0 => None,
            len => Some(self.scores()[len - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        //unimplemented!("Return the highest score")

        match self.len4s() {
            0 => None,
            _ => Some(*self.scores_sorted.first().unwrap()),
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        //unimplemented!("Return 3 highest scores")
        // len < 3, usage len
        self.scores_sorted[..min(3, self.len4s())].to_vec()
    }
}
