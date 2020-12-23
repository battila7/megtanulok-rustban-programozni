use std::cmp;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    sorted_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut sorted_scores = scores.to_vec();
        // Reverse comparison so that
        // sorted_scores will contain the scores
        // in decreasing order.
        sorted_scores.sort_unstable_by(|a, b| b.cmp(a));

        HighScores {
            scores: scores.to_vec(),
            sorted_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // first returns Option<&u32>
        // which we can turn into n Option<u32>
        // by using cloned as cloned
        // clones the contents of the Option.
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.sorted_scores.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // Vec.truncate works in place, truncating the
        // original Vec, thus we are better off creating
        // and copying a slice.
        let top_count = cmp::min(self.sorted_scores.len(), 3);
        
        self.sorted_scores[0..top_count].to_vec()
    }
}
