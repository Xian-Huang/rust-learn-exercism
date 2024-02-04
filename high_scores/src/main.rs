pub struct HighScores{
    value:&[u32]
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores{value:scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.value
    }

    pub fn latest(&self) -> Option<u32> {
        Some(*self.value.last().unwrap())
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut max = self.value[0];
        for i in self.value{
            if *i>max{
                max = *i;
            }
        }
        Some(max)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut res = Vec::from(self.value);
        res.sort();
        Vec::from(&res[0..3])
    }
}


fn main() {
    let high_scores = HighScores::new(&[100, 0, 90, 30]);
    assert_eq!(high_scores.latest(), Some(30));
}

