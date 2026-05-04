#[derive(Debug)]
pub struct HighScores {
    scores_vec: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores_vec: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores_vec
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores_vec.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_vec.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores_vec.to_vec();
        sorted_scores.sort();
        sorted_scores.reverse();
        sorted_scores.iter().take(3).copied().collect()
    }
}