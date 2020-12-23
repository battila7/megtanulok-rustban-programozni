use std::cmp;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    sorted_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort_by(|a, b| b.cmp(a));

        HighScores {
            scores: scores.to_vec(),
            sorted_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&x| x)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.sorted_scores.first().map(|&x| x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let top_count = cmp::min(self.sorted_scores.len(), 3);
        
        self.sorted_scores[0..top_count].to_vec()
    }
}
